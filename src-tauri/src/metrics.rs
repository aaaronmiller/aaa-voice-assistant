use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use sysinfo::{CpuExt, NetworkData, NetworksExt, System, SystemExt};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MetricsError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),
    #[error("Failed to create database directory: {0}")]
    DirectoryError(#[from] std::io::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionMetric {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub provider: String,
    pub audio_duration_ms: u64,
    pub transcription_time_ms: u64,
    pub success: bool,
    pub text_length: usize,
    pub language: String,
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderMetrics {
    pub provider: String,
    pub total_count: u64,
    pub success_count: u64,
    pub avg_duration_ms: f64,
    pub total_audio_ms: u64,
    pub estimated_cost: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageMetrics {
    pub total_transcriptions: u64,
    pub total_audio_duration_ms: u64,
    pub total_processing_time_ms: u64,
    pub success_rate: f64,
    pub provider_metrics: Vec<ProviderMetrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_usage_percent: f32,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    pub gpu_memory_used_mb: Option<u64>,
    pub network_bytes_sent: u64,
    pub network_bytes_recv: u64,
}

pub struct MetricsTracker {
    conn: Connection,
}

impl MetricsTracker {
    pub fn new() -> Result<Self, MetricsError> {
        let db_path = Self::get_db_path()?;
        std::fs::create_dir_all(db_path.parent().unwrap())?;

        let conn = Connection::open(&db_path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS transcription_metrics (
                id TEXT PRIMARY KEY,
                timestamp INTEGER NOT NULL,
                provider TEXT NOT NULL,
                audio_duration_ms INTEGER NOT NULL,
                transcription_time_ms INTEGER NOT NULL,
                success INTEGER NOT NULL,
                text_length INTEGER NOT NULL,
                language TEXT NOT NULL,
                mode TEXT NOT NULL,
                estimated_cost REAL DEFAULT 0
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_metrics_timestamp ON transcription_metrics(timestamp)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_metrics_provider ON transcription_metrics(provider)",
            [],
        )?;

        Ok(Self { conn })
    }

    fn get_db_path() -> Result<PathBuf, MetricsError> {
        let mut config_dir = dirs::config_dir().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find config directory",
            )
        })?;
        config_dir.push("voice-assistant");
        Ok(config_dir.join("metrics.db"))
    }

    pub fn record_transcription(&self, metric: &TranscriptionMetric) -> Result<(), MetricsError> {
        self.conn.execute(
            "INSERT INTO transcription_metrics 
             (id, timestamp, provider, audio_duration_ms, transcription_time_ms, success, text_length, language, mode)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                metric.id,
                metric.timestamp.timestamp(),
                metric.provider,
                metric.audio_duration_ms as i64,
                metric.transcription_time_ms as i64,
                metric.success as i32,
                metric.text_length as i64,
                metric.language,
                metric.mode,
            ],
        )?;
        Ok(())
    }

    pub fn get_usage_metrics(&self, since: DateTime<Utc>) -> Result<UsageMetrics, MetricsError> {
        let total_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM transcription_metrics WHERE timestamp >= ?1",
            params![since.timestamp()],
            |row| row.get(0),
        )?;

        let total_audio: i64 = self.conn.query_row(
            "SELECT COALESCE(SUM(audio_duration_ms), 0) FROM transcription_metrics WHERE timestamp >= ?1",
            params![since.timestamp()],
            |row| row.get(0),
        )?;

        let total_processing: i64 = self.conn.query_row(
            "SELECT COALESCE(SUM(transcription_time_ms), 0) FROM transcription_metrics WHERE timestamp >= ?1",
            params![since.timestamp()],
            |row| row.get(0),
        )?;

        let success_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM transcription_metrics WHERE timestamp >= ?1 AND success = 1",
            params![since.timestamp()],
            |row| row.get(0),
        )?;

        let success_rate = if total_count > 0 {
            success_count as f64 / total_count as f64
        } else {
            1.0
        };

        let provider_metrics = self.get_provider_metrics(since)?;

        Ok(UsageMetrics {
            total_transcriptions: total_count as u64,
            total_audio_duration_ms: total_audio as u64,
            total_processing_time_ms: total_processing as u64,
            success_rate,
            provider_metrics,
        })
    }

    fn get_provider_metrics(
        &self,
        since: DateTime<Utc>,
    ) -> Result<Vec<ProviderMetrics>, MetricsError> {
        let mut stmt = self.conn.prepare(
            "SELECT 
                provider,
                COUNT(*) as total_count,
                SUM(CASE WHEN success = 1 THEN 1 ELSE 0 END) as success_count,
                AVG(transcription_time_ms) as avg_duration,
                SUM(audio_duration_ms) as total_audio,
                COALESCE(SUM(estimated_cost), 0) as total_cost
             FROM transcription_metrics 
             WHERE timestamp >= ?1
             GROUP BY provider",
        )?;

        let metrics = stmt
            .query_map(params![since.timestamp()], |row| {
                Ok(ProviderMetrics {
                    provider: row.get(0)?,
                    total_count: row.get::<_, i64>(1)? as u64,
                    success_count: row.get::<_, i64>(2)? as u64,
                    avg_duration_ms: row.get(3)?,
                    total_audio_ms: row.get::<_, i64>(4)? as u64,
                    estimated_cost: row.get(5)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(metrics)
    }

    pub fn estimate_cost(provider: &str, audio_duration_ms: u64, _text_length: usize) -> f64 {
        let audio_seconds = audio_duration_ms as f64 / 1000.0;
        let audio_minutes = audio_seconds / 60.0;

        match provider {
            "assemblyai" => audio_seconds * 0.0001,
            "deepgram" => audio_seconds * 0.0048,
            "openai-whisper" => audio_minutes * 0.006,
            "elevenlabs" => _text_length as f64 * 0.00002,
            "inworld" => audio_seconds * 0.001,
            _ => 0.0,
        }
    }

    pub fn clear_all(&self) -> Result<(), MetricsError> {
        self.conn.execute("DELETE FROM transcription_metrics", [])?;
        Ok(())
    }
}

impl Default for MetricsTracker {
    fn default() -> Self {
        Self::new().expect("Failed to create MetricsTracker")
    }
}

pub struct ResourceMonitor {
    system: System,
}

impl ResourceMonitor {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        Self { system }
    }

    pub fn get_resource_usage(&mut self) -> ResourceUsage {
        self.system.refresh_all();

        let cpu_usage = self.system.global_cpu_info().cpu_usage();
        let memory_used = self.system.used_memory() / 1024 / 1024;
        let memory_total = self.system.total_memory() / 1024 / 1024;

        let mut bytes_sent: u64 = 0;
        let mut bytes_recv: u64 = 0;
        for (_, data) in self.system.networks().iter() {
            bytes_sent += data.transmitted();
            bytes_recv += data.received();
        }

        ResourceUsage {
            cpu_usage_percent: cpu_usage,
            memory_used_mb: memory_used,
            memory_total_mb: memory_total,
            gpu_memory_used_mb: None,
            network_bytes_sent: bytes_sent,
            network_bytes_recv: bytes_recv,
        }
    }

    pub fn get_cpu_usage(&mut self) -> f32 {
        self.system.refresh_cpu();
        self.system.global_cpu_info().cpu_usage()
    }

    pub fn get_memory_usage(&self) -> (u64, u64) {
        (
            self.system.used_memory() / 1024 / 1024,
            self.system.total_memory() / 1024 / 1024,
        )
    }
}

impl Default for ResourceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

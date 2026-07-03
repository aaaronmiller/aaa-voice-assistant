use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HistoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),
    #[error("Failed to create database directory: {0}")]
    DirectoryError(#[from] std::io::Error),
    #[error("Export failed: {0}")]
    ExportError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub text: String,
    pub provider: String,
    pub confidence: Option<f32>,
    pub duration_ms: u64,
    pub mode: String,
    pub language: String,
}

pub struct HistoryManager {
    conn: Connection,
    retention_limit: usize,
}

impl HistoryManager {
    pub fn new() -> Result<Self, HistoryError> {
        let db_path = Self::get_db_path()?;
        std::fs::create_dir_all(db_path.parent().unwrap())?;

        let conn = Connection::open(&db_path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS transcriptions (
                id TEXT PRIMARY KEY,
                timestamp INTEGER NOT NULL,
                text TEXT NOT NULL,
                provider TEXT NOT NULL,
                confidence REAL,
                duration_ms INTEGER NOT NULL,
                mode TEXT NOT NULL,
                language TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_timestamp ON transcriptions(timestamp)",
            [],
        )?;

        conn.execute(
            "CREATE VIRTUAL TABLE IF NOT EXISTS transcriptions_fts USING fts5(text, content=transcriptions)",
            [],
        ).ok(); // FTS5 might not be available, ignore error

        Ok(Self {
            conn,
            retention_limit: 10000,
        })
    }

    fn get_db_path() -> Result<PathBuf, HistoryError> {
        let mut config_dir = dirs::config_dir().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find config directory",
            )
        })?;
        config_dir.push("voice-assistant");
        Ok(config_dir.join("history.db"))
    }

    pub fn add_entry(&self, entry: &HistoryEntry) -> Result<(), HistoryError> {
        self.conn.execute(
            "INSERT INTO transcriptions (id, timestamp, text, provider, confidence, duration_ms, mode, language)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                entry.id,
                entry.timestamp.timestamp(),
                entry.text,
                entry.provider,
                entry.confidence,
                entry.duration_ms as i64,
                entry.mode,
                entry.language,
            ],
        )?;

        self.enforce_retention()?;
        Ok(())
    }

    pub fn get_recent(&self, limit: usize) -> Result<Vec<HistoryEntry>, HistoryError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, timestamp, text, provider, confidence, duration_ms, mode, language
             FROM transcriptions
             ORDER BY timestamp DESC
             LIMIT ?1",
        )?;

        let entries = stmt
            .query_map(params![limit as i64], |row| {
                Ok(HistoryEntry {
                    id: row.get(0)?,
                    timestamp: DateTime::from_timestamp(row.get::<_, i64>(1)?, 0)
                        .unwrap_or_else(|| Utc::now()),
                    text: row.get(2)?,
                    provider: row.get(3)?,
                    confidence: row.get(4)?,
                    duration_ms: row.get::<_, i64>(5)? as u64,
                    mode: row.get(6)?,
                    language: row.get(7)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(entries)
    }

    pub fn search(&self, query: &str) -> Result<Vec<HistoryEntry>, HistoryError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, timestamp, text, provider, confidence, duration_ms, mode, language
             FROM transcriptions
             WHERE text LIKE ?1
             ORDER BY timestamp DESC
             LIMIT 100",
        )?;

        let search_pattern = format!("%{}%", query);
        let entries = stmt
            .query_map(params![search_pattern], |row| {
                Ok(HistoryEntry {
                    id: row.get(0)?,
                    timestamp: DateTime::from_timestamp(row.get::<_, i64>(1)?, 0)
                        .unwrap_or_else(|| Utc::now()),
                    text: row.get(2)?,
                    provider: row.get(3)?,
                    confidence: row.get(4)?,
                    duration_ms: row.get::<_, i64>(5)? as u64,
                    mode: row.get(6)?,
                    language: row.get(7)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(entries)
    }

    pub fn delete_entry(&self, id: &str) -> Result<(), HistoryError> {
        self.conn
            .execute("DELETE FROM transcriptions WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn clear_all(&self) -> Result<(), HistoryError> {
        self.conn.execute("DELETE FROM transcriptions", [])?;
        Ok(())
    }

    pub fn set_retention_limit(&mut self, limit: usize) {
        self.retention_limit = limit;
    }

    fn enforce_retention(&self) -> Result<(), HistoryError> {
        let count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM transcriptions", [], |row| row.get(0))?;

        if count as usize > self.retention_limit {
            let delete_count = count as usize - self.retention_limit;
            self.conn.execute(
                "DELETE FROM transcriptions WHERE id IN (
                    SELECT id FROM transcriptions ORDER BY timestamp ASC LIMIT ?1
                )",
                params![delete_count as i64],
            )?;
        }

        Ok(())
    }

    pub fn export_json(&self) -> Result<String, HistoryError> {
        let entries = self.get_recent(self.retention_limit)?;
        serde_json::to_string_pretty(&entries).map_err(|e| HistoryError::ExportError(e.to_string()))
    }

    pub fn export_csv(&self) -> Result<String, HistoryError> {
        let entries = self.get_recent(self.retention_limit)?;
        let mut csv =
            String::from("id,timestamp,text,provider,confidence,duration_ms,mode,language\n");

        for entry in entries {
            csv.push_str(&format!(
                "{},{},\"{}\",{},{},{},{},{}\n",
                entry.id,
                entry.timestamp.to_rfc3339(),
                entry.text.replace("\"", "\"\""),
                entry.provider,
                entry.confidence.map(|c| c.to_string()).unwrap_or_default(),
                entry.duration_ms,
                entry.mode,
                entry.language
            ));
        }

        Ok(csv)
    }
}

impl Default for HistoryManager {
    fn default() -> Self {
        Self::new().expect("Failed to create HistoryManager")
    }
}

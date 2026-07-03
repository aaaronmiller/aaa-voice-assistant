use std::sync::Arc;
use tauri::State;
use serde::{Deserialize, Serialize};
use chrono::Utc;
use uuid::Uuid;

use crate::config::Settings;
use crate::state::AppState;
use crate::audio::{AudioDevice, AudioLevels};
use crate::history::HistoryEntry;
use crate::metrics::{TranscriptionMetric, UsageMetrics, ResourceUsage};

#[derive(Debug, Serialize, Deserialize)]
pub struct TranscriptionResult {
    pub text: String,
    pub confidence: Option<f32>,
    pub provider: String,
    pub duration_ms: u64,
}

#[tauri::command]
pub async fn get_settings(state: State<'_, Arc<AppState>>) -> Result<Settings, String> {
    let config = state.config.lock().map_err(|e| e.to_string())?;
    Ok(config.get().clone())
}

#[tauri::command]
pub async fn update_settings(
    state: State<'_, Arc<AppState>>,
    settings: Settings,
) -> Result<(), String> {
    let mut config = state.config.lock().map_err(|e| e.to_string())?;
    config.update(settings).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_audio_devices() -> Result<Vec<AudioDevice>, String> {
    crate::audio::AudioManager::list_input_devices()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn start_recording(state: State<'_, Arc<AppState>>) -> Result<(), String> {
    let mut is_recording = state.is_recording.lock().map_err(|e| e.to_string())?;
    if *is_recording {
        return Ok(());
    }
    *is_recording = true;
    state.audio_buffer.lock().map_err(|e| e.to_string())?.clear();
    Ok(())
}

#[tauri::command]
pub async fn stop_recording(state: State<'_, Arc<AppState>>) -> Result<Vec<f32>, String> {
    let mut is_recording = state.is_recording.lock().map_err(|e| e.to_string())?;
    *is_recording = false;
    let buffer = state.audio_buffer.lock().map_err(|e| e.to_string())?;
    Ok(buffer.clone())
}

#[tauri::command]
pub async fn get_audio_levels(state: State<'_, Arc<AppState>>) -> Result<AudioLevels, String> {
    let buffer = state.audio_buffer.lock().map_err(|e| e.to_string())?;
    
    if buffer.is_empty() {
        return Ok(AudioLevels { rms: 0.0, peak: 0.0 });
    }

    let rms = buffer.iter().map(|&x| x * x).sum::<f32>().sqrt() / buffer.len() as f32;
    let peak = buffer.iter().map(|&x| x.abs()).fold(0.0, f32::max);

    Ok(AudioLevels { rms, peak })
}

#[tauri::command]
pub async fn transcribe(
    state: State<'_, Arc<AppState>>,
    audio_data: Vec<f32>,
) -> Result<TranscriptionResult, String> {
    let config = state.config.lock().map_err(|e| e.to_string())?;
    let provider_name = config.get().stt.provider.clone();
    let language = config.get().stt.language.clone();
    let auto_cap = config.get().auto_capitalization;
    let auto_punct = config.get().auto_punctuation;
    let cleanup_enabled = config.get().transcription_cleanup;
    let cleanup_prompt = config.get().cleanup_prompt.clone();
    let llm_provider = config.get().llm.provider.clone();
    let llm_model = config.get().llm.model.clone();
    drop(config);

    let start = std::time::Instant::now();
    
    let mut text = format!("Transcribed {} audio samples", audio_data.len());
    
    // Apply auto-capitalization if enabled
    if auto_cap {
        text = crate::cleanup::apply_auto_capitalization(&text);
    }
    
    // Apply auto-punctuation if enabled
    if auto_punct {
        text = crate::cleanup::apply_auto_punctuation(&text);
    }
    
    // Apply cleanup if enabled
    if cleanup_enabled {
        let cleanup_config = crate::cleanup::CleanupConfig {
            enabled: true,
            provider: llm_provider,
            model: llm_model,
            prompt_template: cleanup_prompt,
            max_tokens: 2048,
            timeout_ms: 5000,
        };
        let processor = crate::cleanup::CleanupProcessor::new(cleanup_config);
        text = processor.process(&text, None).unwrap_or(text);
    }
    
    let result = TranscriptionResult {
        text,
        confidence: Some(0.95),
        provider: provider_name.clone(),
        duration_ms: start.elapsed().as_millis() as u64,
    };
    
    // Record to history
    let history_entry = HistoryEntry {
        id: Uuid::new_v4().to_string(),
        timestamp: Utc::now(),
        text: result.text.clone(),
        provider: provider_name.clone(),
        confidence: result.confidence,
        duration_ms: result.duration_ms,
        mode: "ptt".to_string(),
        language: language.clone(),
    };
    
    if let Ok(history) = state.history.lock() {
        let _ = history.add_entry(&history_entry);
    }
    
    let metric = TranscriptionMetric {
        id: Uuid::new_v4().to_string(),
        timestamp: Utc::now(),
        provider: provider_name,
        audio_duration_ms: (audio_data.len() as f64 / 16.0) as u64,
        transcription_time_ms: result.duration_ms,
        success: true,
        text_length: result.text.len(),
        language,
        mode: "ptt".to_string(),
    };
    
    if let Ok(metrics) = state.metrics.lock() {
        let _ = metrics.record_transcription(&metric);
    }
    
    Ok(result)
}

#[tauri::command]
pub async fn paste_text(text: String) -> Result<(), String> {
    let clipboard = crate::clipboard::ClipboardManager::new()
        .map_err(|e| e.to_string())?;
    clipboard.paste_text(&text).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn register_hotkey(
    state: State<'_, Arc<AppState>>,
    id: String,
    hotkey: String,
) -> Result<(), String> {
    let hotkey_id = crate::hotkey::HotkeyId::from_str(&id)
        .ok_or_else(|| format!("Invalid hotkey ID: {}", id))?;

    state
        .hotkey_manager
        .register(hotkey_id, &hotkey)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn unregister_hotkey(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> Result<(), String> {
    let hotkey_id = crate::hotkey::HotkeyId::from_str(&id)
        .ok_or_else(|| format!("Invalid hotkey ID: {}", id))?;

    state
        .hotkey_manager
        .unregister(hotkey_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_gpu_backend(state: State<'_, Arc<AppState>>) -> Result<String, String> {
    Ok(format!("{:?}", state.gpu_backend))
}

#[tauri::command]
pub async fn get_history(
    state: State<'_, Arc<AppState>>,
    limit: usize,
) -> Result<Vec<HistoryEntry>, String> {
    let history = state.history.lock().map_err(|e| e.to_string())?;
    history.get_recent(limit).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_history(
    state: State<'_, Arc<AppState>>,
    query: String,
) -> Result<Vec<HistoryEntry>, String> {
    let history = state.history.lock().map_err(|e| e.to_string())?;
    history.search(&query).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_history_entry(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> Result<(), String> {
    let history = state.history.lock().map_err(|e| e.to_string())?;
    history.delete_entry(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn clear_history(state: State<'_, Arc<AppState>>) -> Result<(), String> {
    let mut history = state.history.lock().map_err(|e| e.to_string())?;
    history.clear_all().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_usage_metrics(state: State<'_, Arc<AppState>>) -> Result<UsageMetrics, String> {
    let metrics = state.metrics.lock().map_err(|e| e.to_string())?;
    let since = Utc::now() - chrono::Duration::days(30);
    metrics.get_usage_metrics(since).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_resource_usage(state: State<'_, Arc<AppState>>) -> Result<ResourceUsage, String> {
    let mut monitor = state.resource_monitor.lock().map_err(|e| e.to_string())?;
    Ok(monitor.get_resource_usage())
}

#[tauri::command]
pub async fn cleanup_text(
    state: State<'_, Arc<AppState>>,
    text: String,
    api_key: Option<String>,
) -> Result<String, String> {
    let config = state.config.lock().map_err(|e| e.to_string())?;
    let cleanup_config = crate::cleanup::CleanupConfig {
        enabled: config.get().transcription_cleanup,
        provider: config.get().llm.provider.clone(),
        model: config.get().llm.model.clone(),
        prompt_template: config.get().cleanup_prompt.clone(),
        max_tokens: 2048,
        timeout_ms: 5000,
    };
    drop(config);
    
    let processor = crate::cleanup::CleanupProcessor::new(cleanup_config);
    processor.process(&text, api_key.as_deref()).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn record_metric(
    state: State<'_, Arc<AppState>>,
    provider: String,
    audio_duration_ms: u64,
    transcription_time_ms: u64,
    success: bool,
    text_length: usize,
    language: String,
    mode: String,
) -> Result<(), String> {
    let metric = TranscriptionMetric {
        id: Uuid::new_v4().to_string(),
        timestamp: Utc::now(),
        provider,
        audio_duration_ms,
        transcription_time_ms,
        success,
        text_length,
        language,
        mode,
    };
    
    let metrics = state.metrics.lock().map_err(|e| e.to_string())?;
    metrics.record_transcription(&metric).map_err(|e| e.to_string())
}
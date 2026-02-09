use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("Failed to parse config file: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("Failed to write config file: {0}")]
    WriteError(#[from] std::io::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct STTConfig {
    pub provider: String,
    pub fallback_providers: Vec<String>,
    pub language: String,
    pub streaming: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TTSConfig {
    pub provider: String,
    pub voice: String,
    pub volume: f32,
    pub speed: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LLMConfig {
    pub provider: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HotkeyConfig {
    pub ptt: String,
    pub paste_last: String,
    pub toggle_wake_word: String,
    pub toggle_cleanup: String,
    pub open_history: String,
    pub open_settings: String,
    pub cancel_recording: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioConfig {
    pub input_device: String,
    pub output_device: String,
    pub noise_suppression: bool,
    pub auto_gain_control: bool,
    pub silence_threshold: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub stt: STTConfig,
    pub tts: TTSConfig,
    pub llm: LLMConfig,
    pub hotkeys: HotkeyConfig,
    pub audio: AudioConfig,
    pub wake_word: String,
    pub wake_word_enabled: bool,
    pub transcription_mode: String,
    pub auto_capitalization: bool,
    pub auto_punctuation: bool,
    pub transcription_cleanup: bool,
    pub cleanup_prompt: String,
    pub history_retention: u32,
    pub theme: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            stt: STTConfig {
                provider: "whisper.cpp".to_string(),
                fallback_providers: vec!["vosk".to_string()],
                language: "en".to_string(),
                streaming: false,
            },
            tts: TTSConfig {
                provider: "piper".to_string(),
                voice: "en_US-lessac-medium".to_string(),
                volume: 1.0,
                speed: 1.0,
            },
            llm: LLMConfig {
                provider: "gemini".to_string(),
                model: "gemini-pro".to_string(),
                temperature: 0.7,
                max_tokens: 2048,
            },
            hotkeys: HotkeyConfig {
                ptt: "F1".to_string(),
                paste_last: "Ctrl+Shift+V".to_string(),
                toggle_wake_word: "Ctrl+Shift+W".to_string(),
                toggle_cleanup: "Ctrl+Shift+C".to_string(),
                open_history: "Ctrl+Shift+H".to_string(),
                open_settings: "Ctrl+Shift+S".to_string(),
                cancel_recording: "Esc".to_string(),
            },
            audio: AudioConfig {
                input_device: "default".to_string(),
                output_device: "default".to_string(),
                noise_suppression: true,
                auto_gain_control: true,
                silence_threshold: 0.1,
            },
            wake_word: "hey computer".to_string(),
            wake_word_enabled: false,
            transcription_mode: "insert".to_string(),
            auto_capitalization: true,
            auto_punctuation: true,
            transcription_cleanup: false,
            cleanup_prompt: "Fix grammar, punctuation, and capitalization".to_string(),
            history_retention: 10000,
            theme: "system".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct ConfigurationStore {
    settings: Settings,
    config_path: PathBuf,
}

impl ConfigurationStore {
    pub fn load() -> Result<Self, ConfigError> {
        let config_path = Self::get_config_path()?;
        
        if config_path.exists() {
            let config_content = fs::read_to_string(&config_path)?;
            let settings = serde_json::from_str(&config_content)?;
            Ok(Self { settings, config_path })
        } else {
            let settings = Settings::default();
            fs::create_dir_all(config_path.parent().unwrap())?;
            fs::write(&config_path, serde_json::to_string_pretty(&settings)?)?;
            Ok(Self { settings, config_path })
        }
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        fs::write(&self.config_path, serde_json::to_string_pretty(&self.settings)?)?;
        Ok(())
    }

    pub fn get(&self) -> &Settings {
        &self.settings
    }

    pub fn get_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    pub fn update(&mut self, settings: Settings) -> Result<(), ConfigError> {
        self.settings = settings;
        self.save()
    }

    fn get_config_path() -> Result<PathBuf, ConfigError> {
        let mut config_dir = dirs::config_dir().ok_or_else(|| {
            ConfigError::ReadError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find config directory",
            ))
        })?;
        
        config_dir.push("voice-assistant");
        Ok(config_dir.join("config.json"))
    }
}

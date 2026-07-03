use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TTSError {
    #[error("Provider initialization failed: {0}")]
    InitError(String),
    #[error("Synthesis failed: {0}")]
    SynthesisError(String),
    #[error("Provider not found: {0}")]
    ProviderNotFound(String),
    #[error("No providers available")]
    NoProviders,
    #[error("Voice not found: {0}")]
    VoiceNotFound(String),
    #[error("Playback failed: {0}")]
    PlaybackError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Voice {
    pub id: String,
    pub name: String,
    pub language: String,
    pub gender: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesisResult {
    pub audio_data: Vec<u8>,
    pub sample_rate: u32,
    pub duration_ms: u64,
    pub provider: String,
    pub voice: String,
}

pub trait TTSProvider: Send + Sync {
    fn name(&self) -> &str;

    fn synthesize(&self, text: &str, voice: &str) -> Result<SynthesisResult, TTSError>;

    fn list_voices(&self) -> Vec<Voice>;

    fn supports_streaming(&self) -> bool {
        false
    }

    fn is_local(&self) -> bool;

    fn is_available(&self) -> bool {
        true
    }
}

pub struct TTSEngine {
    providers: HashMap<String, Box<dyn TTSProvider>>,
    active_provider: String,
    active_voice: String,
}

impl TTSEngine {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            active_provider: String::new(),
            active_voice: String::new(),
        }
    }

    pub fn register_provider(&mut self, provider: Box<dyn TTSProvider>) {
        let name = provider.name().to_string();
        if self.active_provider.is_empty() {
            self.active_provider = name.clone();
            if let Some(voice) = provider.list_voices().first() {
                self.active_voice = voice.id.clone();
            }
        }
        self.providers.insert(name, provider);
    }

    pub fn set_active_provider(&mut self, name: &str) -> Result<(), TTSError> {
        if self.providers.contains_key(name) {
            self.active_provider = name.to_string();
            Ok(())
        } else {
            Err(TTSError::ProviderNotFound(name.to_string()))
        }
    }

    pub fn set_voice(&mut self, voice_id: &str) {
        self.active_voice = voice_id.to_string();
    }

    pub fn synthesize(&self, text: &str) -> Result<SynthesisResult, TTSError> {
        let provider = self
            .providers
            .get(&self.active_provider)
            .ok_or_else(|| TTSError::ProviderNotFound(self.active_provider.clone()))?;

        if !provider.is_available() {
            return Err(TTSError::SynthesisError(format!(
                "Provider {} is not available",
                self.active_provider
            )));
        }

        provider.synthesize(text, &self.active_voice)
    }

    pub fn list_providers(&self) -> Vec<&str> {
        self.providers.keys().map(|s| s.as_str()).collect()
    }

    pub fn list_voices(&self) -> Vec<Voice> {
        self.providers
            .get(&self.active_provider)
            .map(|p| p.list_voices())
            .unwrap_or_default()
    }

    pub fn get_active_provider(&self) -> &str {
        &self.active_provider
    }

    pub fn get_active_voice(&self) -> &str {
        &self.active_voice
    }
}

impl Default for TTSEngine {
    fn default() -> Self {
        Self::new()
    }
}

// Piper TTS Provider stub
pub struct PiperTTSProvider {
    model_path: Option<String>,
    available: bool,
}

impl PiperTTSProvider {
    pub fn new() -> Self {
        Self {
            model_path: None,
            available: false,
        }
    }

    pub fn with_model(mut self, path: &str) -> Self {
        self.model_path = Some(path.to_string());
        self.available = true;
        self
    }
}

impl Default for PiperTTSProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl TTSProvider for PiperTTSProvider {
    fn name(&self) -> &str {
        "piper"
    }

    fn synthesize(&self, _text: &str, voice: &str) -> Result<SynthesisResult, TTSError> {
        if !self.available {
            return Err(TTSError::InitError("Piper model not loaded".to_string()));
        }

        let start = std::time::Instant::now();

        Ok(SynthesisResult {
            audio_data: vec![],
            sample_rate: 22050,
            duration_ms: start.elapsed().as_millis() as u64,
            provider: self.name().to_string(),
            voice: voice.to_string(),
        })
    }

    fn list_voices(&self) -> Vec<Voice> {
        vec![
            Voice {
                id: "en_US-lessac-medium".to_string(),
                name: "Lessac (US English)".to_string(),
                language: "en-US".to_string(),
                gender: Some("female".to_string()),
            },
            Voice {
                id: "en_US-ryan-medium".to_string(),
                name: "Ryan (US English)".to_string(),
                language: "en-US".to_string(),
                gender: Some("male".to_string()),
            },
        ]
    }

    fn is_local(&self) -> bool {
        true
    }

    fn is_available(&self) -> bool {
        self.available
    }
}

#[derive(Debug, Clone)]
pub struct InworldConfig {
    pub api_key: String,
    pub jwt_key: String,
    pub jwt_secret: String,
}

pub struct InworldTTSProvider {
    config: Option<InworldConfig>,
    client: reqwest::Client,
}

impl InworldTTSProvider {
    pub fn new() -> Self {
        Self {
            config: None,
            client: reqwest::Client::new(),
        }
    }

    pub fn with_credentials(mut self, api_key: &str, jwt_key: &str, jwt_secret: &str) -> Self {
        self.config = Some(InworldConfig {
            api_key: api_key.to_string(),
            jwt_key: jwt_key.to_string(),
            jwt_secret: jwt_secret.to_string(),
        });
        self
    }
}

impl Default for InworldTTSProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl TTSProvider for InworldTTSProvider {
    fn name(&self) -> &str {
        "inworld"
    }

    fn synthesize(&self, _text: &str, voice: &str) -> Result<SynthesisResult, TTSError> {
        let config = self.config.as_ref().ok_or(TTSError::InitError(
            "Inworld credentials not configured".to_string(),
        ))?;

        let start = std::time::Instant::now();
        let _ = &config.api_key;

        // TODO: Real Inworld TTS-1.5 API integration
        Ok(SynthesisResult {
            audio_data: vec![],
            sample_rate: 24000,
            duration_ms: start.elapsed().as_millis() as u64,
            provider: self.name().to_string(),
            voice: voice.to_string(),
        })
    }

    fn list_voices(&self) -> Vec<Voice> {
        vec![Voice {
            id: "inworld_1".to_string(),
            name: "Inworld Default".to_string(),
            language: "en-US".to_string(),
            gender: None,
        }]
    }

    fn supports_streaming(&self) -> bool {
        true
    }

    fn is_local(&self) -> bool {
        false
    }

    fn is_available(&self) -> bool {
        self.config.is_some()
    }
}

#[derive(Debug, Clone)]
pub struct ElevenLabsConfig {
    pub api_key: String,
}

pub struct ElevenLabsProvider {
    config: Option<ElevenLabsConfig>,
    client: reqwest::Client,
    voices_cache: Vec<Voice>,
}

impl ElevenLabsProvider {
    pub fn new() -> Self {
        Self {
            config: None,
            client: reqwest::Client::new(),
            voices_cache: Vec::new(),
        }
    }

    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.config = Some(ElevenLabsConfig {
            api_key: api_key.to_string(),
        });
        self.voices_cache = vec![
            Voice {
                id: "21m00Tcm4TlvDq8ikWAM".to_string(),
                name: "Rachel".to_string(),
                language: "en-US".to_string(),
                gender: Some("female".to_string()),
            },
            Voice {
                id: "AZnzlk1XvdvUeBnXmlld".to_string(),
                name: "Domi".to_string(),
                language: "en-US".to_string(),
                gender: Some("female".to_string()),
            },
            Voice {
                id: "EXAVITQu4vr4xnSDxMaL".to_string(),
                name: "Bella".to_string(),
                language: "en-US".to_string(),
                gender: Some("female".to_string()),
            },
        ];
        self
    }
}

impl Default for ElevenLabsProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl TTSProvider for ElevenLabsProvider {
    fn name(&self) -> &str {
        "elevenlabs"
    }

    fn synthesize(&self, text: &str, voice: &str) -> Result<SynthesisResult, TTSError> {
        let config = self.config.as_ref().ok_or(TTSError::InitError(
            "ElevenLabs API key not configured".to_string(),
        ))?;

        let start = std::time::Instant::now();

        // Make API request to ElevenLabs
        let request_body = serde_json::json!({
            "text": text,
            "model_id": "eleven_monolingual_v1",
            "voice_settings": {
                "stability": 0.5,
                "similarity_boost": 0.75,
            },
        });

        let client = reqwest::blocking::Client::new();
        let response = client
            .post(format!(
                "https://api.elevenlabs.io/v1/text-to-speech/{}",
                voice
            ))
            .header("xi-api-key", &config.api_key)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .map_err(|e| TTSError::SynthesisError(format!("ElevenLabs request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            return Err(TTSError::SynthesisError(format!(
                "ElevenLabs API error ({}): {}",
                status, body
            )));
        }

        let audio_data = response
            .bytes()
            .map_err(|e| TTSError::SynthesisError(format!("Failed to read audio data: {}", e)))?
            .to_vec();

        // ElevenLabs returns MP3 audio at 44100Hz
        let duration_ms = start.elapsed().as_millis() as u64;

        Ok(SynthesisResult {
            audio_data,
            sample_rate: 44100,
            duration_ms,
            provider: self.name().to_string(),
            voice: voice.to_string(),
        })
    }

    fn list_voices(&self) -> Vec<Voice> {
        self.voices_cache.clone()
    }

    fn supports_streaming(&self) -> bool {
        true
    }

    fn is_local(&self) -> bool {
        false
    }

    fn is_available(&self) -> bool {
        self.config.is_some()
    }
}

#[derive(Debug, Clone)]
pub struct OpenAITTSConfig {
    pub api_key: String,
    pub model: String,
}

pub struct OpenAITTSProvider {
    config: Option<OpenAITTSConfig>,
    client: reqwest::Client,
}

impl OpenAITTSProvider {
    pub fn new() -> Self {
        Self {
            config: None,
            client: reqwest::Client::new(),
        }
    }

    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.config = Some(OpenAITTSConfig {
            api_key: api_key.to_string(),
            model: "tts-1".to_string(),
        });
        self
    }

    pub fn with_model(mut self, model: &str) -> Self {
        if let Some(ref mut config) = self.config {
            config.model = model.to_string();
        }
        self
    }
}

impl Default for OpenAITTSProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl TTSProvider for OpenAITTSProvider {
    fn name(&self) -> &str {
        "openai-tts"
    }

    fn synthesize(&self, text: &str, voice: &str) -> Result<SynthesisResult, TTSError> {
        let config = self.config.as_ref().ok_or(TTSError::InitError(
            "OpenAI API key not configured".to_string(),
        ))?;

        let start = std::time::Instant::now();

        // Make API request to OpenAI TTS
        let request_body = serde_json::json!({
            "model": config.model,
            "input": text,
            "voice": voice,
        });

        let client = reqwest::blocking::Client::new();
        let response = client
            .post("https://api.openai.com/v1/audio/speech")
            .header("Authorization", format!("Bearer {}", config.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .map_err(|e| TTSError::SynthesisError(format!("OpenAI TTS request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            return Err(TTSError::SynthesisError(format!(
                "OpenAI TTS API error ({}): {}",
                status, body
            )));
        }

        let audio_data = response
            .bytes()
            .map_err(|e| TTSError::SynthesisError(format!("Failed to read audio data: {}", e)))?
            .to_vec();

        // OpenAI TTS returns MP3 audio at 24000Hz
        let duration_ms = start.elapsed().as_millis() as u64;

        Ok(SynthesisResult {
            audio_data,
            sample_rate: 24000,
            duration_ms,
            provider: self.name().to_string(),
            voice: voice.to_string(),
        })
    }

    fn list_voices(&self) -> Vec<Voice> {
        vec![
            Voice {
                id: "alloy".to_string(),
                name: "Alloy".to_string(),
                language: "en-US".to_string(),
                gender: None,
            },
            Voice {
                id: "echo".to_string(),
                name: "Echo".to_string(),
                language: "en-US".to_string(),
                gender: Some("male".to_string()),
            },
            Voice {
                id: "fable".to_string(),
                name: "Fable".to_string(),
                language: "en-US".to_string(),
                gender: None,
            },
            Voice {
                id: "onyx".to_string(),
                name: "Onyx".to_string(),
                language: "en-US".to_string(),
                gender: Some("male".to_string()),
            },
            Voice {
                id: "nova".to_string(),
                name: "Nova".to_string(),
                language: "en-US".to_string(),
                gender: Some("female".to_string()),
            },
            Voice {
                id: "shimmer".to_string(),
                name: "Shimmer".to_string(),
                language: "en-US".to_string(),
                gender: Some("female".to_string()),
            },
        ]
    }

    fn is_local(&self) -> bool {
        false
    }

    fn is_available(&self) -> bool {
        self.config.is_some()
    }
}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum STTError {
    #[error("Provider initialization failed: {0}")]
    InitError(String),
    #[error("Transcription failed: {0}")]
    TranscriptionError(String),
    #[error("Provider not found: {0}")]
    ProviderNotFound(String),
    #[error("No providers available")]
    NoProviders,
    #[error("Audio data is empty")]
    EmptyAudio,
    #[error("Language not supported: {0}")]
    LanguageNotSupported(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alternative {
    pub text: String,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transcription {
    pub text: String,
    pub confidence: Option<f32>,
    pub alternatives: Vec<Alternative>,
    pub language: String,
    pub duration_ms: u64,
    pub provider: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Language {
    EnUS,
    EnGB,
    EsES,
    FrFR,
    DeDE,
    ItIT,
    PtBR,
    JaJP,
    KoKR,
    ZhCN,
    ZhTW,
    RuRU,
    ArSA,
    HiIN,
    Custom(String),
}

impl Language {
    pub fn code(&self) -> &str {
        match self {
            Language::EnUS => "en-US",
            Language::EnGB => "en-GB",
            Language::EsES => "es-ES",
            Language::FrFR => "fr-FR",
            Language::DeDE => "de-DE",
            Language::ItIT => "it-IT",
            Language::PtBR => "pt-BR",
            Language::JaJP => "ja-JP",
            Language::KoKR => "ko-KR",
            Language::ZhCN => "zh-CN",
            Language::ZhTW => "zh-TW",
            Language::RuRU => "ru-RU",
            Language::ArSA => "ar-SA",
            Language::HiIN => "hi-IN",
            Language::Custom(code) => code,
        }
    }

    pub fn from_code(code: &str) -> Self {
        match code {
            "en-US" | "en" => Language::EnUS,
            "en-GB" => Language::EnGB,
            "es-ES" | "es" => Language::EsES,
            "fr-FR" | "fr" => Language::FrFR,
            "de-DE" | "de" => Language::DeDE,
            "it-IT" | "it" => Language::ItIT,
            "pt-BR" | "pt" => Language::PtBR,
            "ja-JP" | "ja" => Language::JaJP,
            "ko-KR" | "ko" => Language::KoKR,
            "zh-CN" | "zh" => Language::ZhCN,
            "zh-TW" => Language::ZhTW,
            "ru-RU" | "ru" => Language::RuRU,
            "ar-SA" | "ar" => Language::ArSA,
            "hi-IN" | "hi" => Language::HiIN,
            other => Language::Custom(other.to_string()),
        }
    }
}

pub trait STTProvider: Send + Sync {
    fn name(&self) -> &str;

    fn transcribe(&self, audio: &[f32], language: &Language) -> Result<Transcription, STTError>;

    fn supports_language(&self, language: &Language) -> bool;

    fn supports_streaming(&self) -> bool {
        false
    }

    fn is_local(&self) -> bool;

    fn is_available(&self) -> bool {
        true
    }
}

pub struct STTEngine {
    providers: HashMap<String, Box<dyn STTProvider>>,
    active_provider: String,
    fallback_chain: Vec<String>,
}

impl STTEngine {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            active_provider: String::new(),
            fallback_chain: Vec::new(),
        }
    }

    pub fn register_provider(&mut self, provider: Box<dyn STTProvider>) {
        let name = provider.name().to_string();
        if self.active_provider.is_empty() {
            self.active_provider = name.clone();
        }
        self.providers.insert(name, provider);
    }

    pub fn set_active_provider(&mut self, name: &str) -> Result<(), STTError> {
        if self.providers.contains_key(name) {
            self.active_provider = name.to_string();
            Ok(())
        } else {
            Err(STTError::ProviderNotFound(name.to_string()))
        }
    }

    pub fn set_fallback_chain(&mut self, chain: Vec<String>) {
        self.fallback_chain = chain;
    }

    pub fn transcribe(
        &self,
        audio: &[f32],
        language: &Language,
    ) -> Result<Transcription, STTError> {
        if audio.is_empty() {
            return Err(STTError::EmptyAudio);
        }

        let mut errors = Vec::new();

        // Try active provider first
        if let Some(provider) = self.providers.get(&self.active_provider) {
            if provider.is_available() && provider.supports_language(language) {
                match provider.transcribe(audio, language) {
                    Ok(result) => return Ok(result),
                    Err(e) => errors.push((provider.name().to_string(), e)),
                }
            }
        }

        // Try fallback chain
        for fallback_name in &self.fallback_chain {
            if let Some(provider) = self.providers.get(fallback_name) {
                if provider.is_available() && provider.supports_language(language) {
                    match provider.transcribe(audio, language) {
                        Ok(result) => return Ok(result),
                        Err(e) => errors.push((provider.name().to_string(), e)),
                    }
                }
            }
        }

        // All providers failed
        Err(STTError::TranscriptionError(format!(
            "All providers failed: {:?}",
            errors
        )))
    }

    pub fn list_providers(&self) -> Vec<&str> {
        self.providers.keys().map(|s| s.as_str()).collect()
    }

    pub fn get_active_provider(&self) -> &str {
        &self.active_provider
    }
}

impl Default for STTEngine {
    fn default() -> Self {
        Self::new()
    }
}

// WhisperCpp Provider implementation that invokes whisper.cpp binary (SYCL-enabled build optional).
pub struct WhisperCppProvider {
    model_path: Option<String>,
    gpu_memory_mb: usize,
    binary_path: Option<String>,
    available: bool,
}

impl WhisperCppProvider {
    pub fn new() -> Self {
        Self {
            model_path: None,
            gpu_memory_mb: 3072,
            binary_path: None,
            available: false,
        }
    }

    pub fn with_model(mut self, path: &str) -> Self {
        self.model_path = Some(path.to_string());
        self.available = true;
        self
    }

    pub fn with_gpu_memory(mut self, mb: usize) -> Self {
        self.gpu_memory_mb = mb;
        self
    }

    pub fn with_binary_path(mut self, path: &str) -> Self {
        self.binary_path = Some(path.to_string());
        self
    }
}

impl Default for WhisperCppProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl STTProvider for WhisperCppProvider {
    fn name(&self) -> &str {
        "whisper.cpp"
    }

    fn transcribe(&self, audio: &[f32], language: &Language) -> Result<Transcription, STTError> {
        if !self.available {
            return Err(STTError::InitError(
                "Whisper.cpp model not loaded".to_string(),
            ));
        }

        let model_path = self.model_path.as_ref().ok_or_else(|| {
            STTError::InitError("Whisper.cpp model path not set".to_string())
        })?;

        // Determine the binary to run: either binary_path, or "whisper-cpp" (or "whisper.cpp") from PATH.
        // Common binary name might be "main" from the whisper.cpp repo, but we accept environment.
        let binary = self
            .binary_path
            .as_deref()
            .or_else(|| std::env::var_os("WHISPER_CPP_BINARY").as_ref().map(std::ffi::OsStr::to_str))
            .flatten()
            .unwrap_or("whisper-cpp"); // default to "whisper-cpp"

        // Convert f32 audio to s16le PCM (16-bit little-endian)
        let mut pcm_bytes = Vec::with_capacity(audio.len() * 2);
        for &sample in audio {
            // Clamp and convert to i16
            let clamped = if sample >= 1.0 { 32767 } else if sample <= -1.0 { -32768 } else { (sample * 32768.0).round() as i16 };
            pcm_bytes.extend_from_slice(&clamped.to_le_bytes());
        }

        // Build command
        use std::process::Stdio;
        let mut cmd = std::process::Command::new(binary);
        cmd.arg("-m")
            .arg(model_path)
            .arg("-ar")
            .arg("16000")
            .arg("-ac")
            .arg("1")
            .arg("-f")
            .arg("s16le")
            .arg("-nt")
            .arg("2") // use 2 threads by default, could be self.gpu_memory_mb not used
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());

        // Optional: For SYCL, set OPENVINO_DEVICE if desired. Could be done via env var on system.
        // No extra action.

        let start = std::time::Instant::now();

        let mut child = cmd.spawn().map_err(|e| {
            STTError::InitError(format!("Failed to spawn whisper.cpp binary: {}", e))
        })?;

        {
            let mut stdin = child.stdin.take().ok_or_else(|| {
                STTError::InitError("Failed to open stdin for whisper.cpp".to_string())
            })?;
            stdin
                .write_all(&pcm_bytes)
                .map_err(|e| STTError::TranscriptionError(format!("Failed to write audio: {}", e)))?;
            // stdin closed automatically when dropped
        }

        let output = child.wait_with_output().map_err(|e| {
            STTError::TranscriptionError(format!("Failed to wait for whisper.cpp output: {}", e))
        })?;

        if !output.status.success() {
            return Err(STTError::TranscriptionError(format!(
                "whisper.cpp exited with code {:?}",
                output.status.code()
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        // The output may contain timestamps depending on flags; we want the raw text.
        // A simple approach: take the first line or trim all.
        let text = stdout.trim().to_string();

        let duration_ms = start.elapsed().as_millis() as u64;

        Ok(Transcription {
            text,
            confidence: None, // whisper.cpp doesn't provide confidence directly
            alternatives: vec![],
            language: language.code().to_string(),
            duration_ms,
            provider: self.name().to_string(),
        })
    }

    fn supports_language(&self, language: &Language) -> bool {
        matches!(
            language,
            Language::EnUS
                | Language::EnGB
                | Language::EsES
                | Language::FrFR
                | Language::DeDE
                | Language::ItIT
                | Language::PtBR
                | Language::JaJP
                | Language::KoKR
                | Language::ZhCN
                | Language::ZhTW
                | Language::RuRU
        )
    }

    fn is_local(&self) -> bool {
        true
    }

    fn is_available(&self) -> bool {
        self.available
    }
}

// Vosk Provider stub
pub struct VoskProvider {
    model_path: Option<String>,
    available: bool,
}

impl VoskProvider {
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

impl Default for VoskProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl STTProvider for VoskProvider {
    fn name(&self) -> &str {
        "vosk"
    }

    fn transcribe(&self, _audio: &[f32], language: &Language) -> Result<Transcription, STTError> {
        if !self.available {
            return Err(STTError::InitError("Vosk model not loaded".to_string()));
        }

        let start = std::time::Instant::now();

        Ok(Transcription {
            text: "[Vosk transcription placeholder]".to_string(),
            confidence: Some(0.85),
            alternatives: vec![],
            language: language.code().to_string(),
            duration_ms: start.elapsed().as_millis() as u64,
            provider: self.name().to_string(),
        })
    }

    fn supports_language(&self, _language: &Language) -> bool {
        true
    }

    fn is_local(&self) -> bool {
        true
    }

    fn is_available(&self) -> bool {
        self.available
    }
}

#[derive(Debug, Clone)]
pub struct AssemblyAIConfig {
    pub api_key: String,
    pub language: String,
}

pub struct AssemblyAIProvider {
    config: Option<AssemblyAIConfig>,
    client: reqwest::Client,
}

impl AssemblyAIProvider {
    pub fn new() -> Self {
        Self {
            config: None,
            client: reqwest::Client::new(),
        }
    }

    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.config = Some(AssemblyAIConfig {
            api_key: api_key.to_string(),
            language: "en".to_string(),
        });
        self
    }
}

impl Default for AssemblyAIProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl STTProvider for AssemblyAIProvider {
    fn name(&self) -> &str {
        "assemblyai"
    }

    fn transcribe(&self, _audio: &[f32], language: &Language) -> Result<Transcription, STTError> {
        let config = self.config.as_ref().ok_or(STTError::InitError(
            "AssemblyAI API key not configured".to_string(),
        ))?;

        let start = std::time::Instant::now();

        // TODO: Real AssemblyAI API integration
        // POST to https://api.assemblyai.com/v2/upload then /transcript
        let _ = &config.api_key;

        Ok(Transcription {
            text: "[AssemblyAI transcription placeholder]".to_string(),
            confidence: Some(0.92),
            alternatives: vec![],
            language: language.code().to_string(),
            duration_ms: start.elapsed().as_millis() as u64,
            provider: self.name().to_string(),
        })
    }

    fn supports_language(&self, _language: &Language) -> bool {
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
pub struct DeepgramConfig {
    pub api_key: String,
    pub model: String,
}

pub struct DeepgramProvider {
    config: Option<DeepgramConfig>,
    client: reqwest::Client,
}

impl DeepgramProvider {
    pub fn new() -> Self {
        Self {
            config: None,
            client: reqwest::Client::new(),
        }
    }

    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.config = Some(DeepgramConfig {
            api_key: api_key.to_string(),
            model: "nova-2".to_string(),
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

impl Default for DeepgramProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl STTProvider for DeepgramProvider {
    fn name(&self) -> &str {
        "deepgram"
    }

    fn transcribe(&self, audio: &[f32], language: &Language) -> Result<Transcription, STTError> {
        let config = self.config.as_ref().ok_or(STTError::InitError(
            "Deepgram API key not configured".to_string(),
        ))?;

        let start = std::time::Instant::now();

        // Convert audio to WAV format (16-bit PCM, mono, 16kHz)
        let wav_data = create_wav_audio(audio, 16000)?;
        let audio_base64 = base64::encode(&wav_data);

        // Build request body for Deepgram API
        let request_body = serde_json::json!({
            "audio": audio_base64,
            "model": config.model,
            "language": language.code(),
            "encoding": "linear16",
            "sample_rate": 16000,
            "channels": 1,
            "detect_language": false,
        });

        // Make synchronous HTTP request to Deepgram
        let client = reqwest::blocking::Client::new();
        let response = client
            .post("https://api.deepgram.com/v1/listen")
            .header("Authorization", format!("Token {}", config.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .map_err(|e| STTError::TranscriptionError(format!("Deepgram request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            return Err(STTError::TranscriptionError(format!(
                "Deepgram API error ({}): {}",
                status, body
            )));
        }

        let json: serde_json::Value = response
            .json()
            .map_err(|e| STTError::TranscriptionError(format!("Failed to parse response: {}", e)))?;

        // Extract transcription from Deepgram response
        let transcription = parse_deepgram_response(&json)?;

        Ok(Transcription {
            text: transcription.text,
            confidence: transcription.confidence,
            alternatives: transcription.alternatives,
            language: language.code().to_string(),
            duration_ms: start.elapsed().as_millis() as u64,
            provider: self.name().to_string(),
        })
    }

    fn supports_language(&self, _language: &Language) -> bool {
        true
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

/// Helper struct for parsing Deepgram response
struct DeepgramTranscription {
    text: String,
    confidence: Option<f32>,
    alternatives: Vec<Alternative>,
}

/// Parse Deepgram API response to extract transcription
fn parse_deepgram_response(json: &serde_json::Value) -> Result<DeepgramTranscription, STTError> {
    let results = json
        .get("results")
        .and_then(|r| r.get("channels"))
        .and_then(|c| c.as_array())
        .and_then(|arr| arr.first())
        .and_then(|ch| ch.get("alternatives"))
        .and_then(|alts| alts.as_array())
        .ok_or_else(|| {
            STTError::InvalidResponse("Invalid Deepgram response structure".to_string())
        })?;

    let alternatives: Vec<Alternative> = results
        .iter()
        .skip(1) // Skip the first one (it's the main transcription)
        .filter_map(|alt| {
            let text = alt.get("transcript")?.as_str()?.to_string();
            let confidence = alt.get("confidence").and_then(|c| c.as_f64()).map(|c| c as f32);
            Some(Alternative { text, confidence })
        })
        .collect();

    let first = results
        .first()
        .ok_or_else(|| STTError::InvalidResponse("No alternatives in response".to_string()))?;

    let text = first
        .get("transcript")
        .and_then(|t| t.as_str())
        .map(|s| s.to_string())
        .unwrap_or_default();

    let confidence = first.get("confidence").and_then(|c| c.as_f64()).map(|c| c as f32);

    Ok(DeepgramTranscription {
        text,
        confidence,
        alternatives,
    })
}

/// Create WAV audio data from f32 samples (16-bit PCM, mono)
fn create_wav_audio(samples: &[f32], sample_rate: u32) -> Result<Vec<u8>, STTError> {
    if samples.is_empty() {
        return Err(STTError::EmptyAudio);
    }

    const BITS_PER_SAMPLE: u16 = 16;
    const NUM_CHANNELS: u16 = 1;

    let data_size = samples.len() * 2; // 2 bytes per sample (16-bit)
    let file_size = 36 + data_size; // Total file size minus 8 bytes for RIFF header

    let mut wav = Vec::with_capacity(44 + data_size);

    // RIFF header
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&file_size.to_le_bytes());
    wav.extend_from_slice(b"WAVE");

    // fmt subchunk
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes()); // Subchunk1Size (16 for PCM)
    wav.extend_from_slice(&1u16.to_le_bytes()); // AudioFormat (1 = PCM)
    wav.extend_from_slice(&NUM_CHANNELS.to_le_bytes());
    wav.extend_from_slice(&sample_rate.to_le_bytes());
    wav.extend_from_slice(&(sample_rate as u32 * NUM_CHANNELS as u32 * BITS_PER_SAMPLE as u32 / 8).to_le_bytes()); // ByteRate
    wav.extend_from_slice(&(NUM_CHANNELS * BITS_PER_SAMPLE / 8).to_le_bytes()); // BlockAlign
    wav.extend_from_slice(&BITS_PER_SAMPLE.to_le_bytes());

    // data subchunk
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&(data_size as u32).to_le_bytes());

    // Convert f32 samples to 16-bit PCM
    for &sample in samples {
        let clamped = sample.max(-1.0).min(1.0);
        let pcm = (clamped * 32767.0) as i16;
        wav.extend_from_slice(&pcm.to_le_bytes());
    }

    Ok(wav)
}

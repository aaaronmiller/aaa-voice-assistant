use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CleanupError {
    #[error("Cleanup request failed: {0}")]
    RequestError(String),
    #[error("Cleanup provider not configured: {0}")]
    ProviderNotConfigured(String),
    #[error("Invalid response from cleanup provider: {0}")]
    InvalidResponse(String),
    #[error("Cleanup timeout")]
    Timeout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupConfig {
    pub enabled: bool,
    pub provider: String,
    pub model: String,
    pub prompt_template: String,
    pub max_tokens: u32,
    pub timeout_ms: u64,
}

impl Default for CleanupConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            provider: "gemini".to_string(),
            model: "gemini-pro".to_string(),
            prompt_template: "Fix grammar, punctuation, and capitalization in the following transcription. Preserve the original meaning and wording as much as possible. Only output the corrected text, nothing else.\n\nTranscription: {text}\n\nCorrected:".to_string(),
            max_tokens: 2048,
            timeout_ms: 5000,
        }
    }
}

pub struct CleanupProcessor {
    config: CleanupConfig,
    client: reqwest::Client,
}

impl CleanupProcessor {
    pub fn new(config: CleanupConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_millis(5000))
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
        }
    }

    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.config.timeout_ms = timeout_ms;
        self
    }

    pub fn set_prompt_template(&mut self, template: &str) {
        self.config.prompt_template = template.to_string();
    }

    pub fn process(&self, raw_text: &str, api_key: Option<&str>) -> Result<String, CleanupError> {
        if !self.config.enabled {
            return Ok(raw_text.to_string());
        }

        let api_key = api_key.ok_or_else(|| {
            CleanupError::ProviderNotConfigured("No API key provided".to_string())
        })?;

        if raw_text.trim().is_empty() {
            return Ok(raw_text.to_string());
        }

        let prompt = self.config.prompt_template.replace("{text}", raw_text);

        let start = std::time::Instant::now();

        // TODO: Actual API call based on provider
        let result = match self.config.provider.as_str() {
            "gemini" => self.call_gemini(&prompt, api_key),
            "openrouter" => self.call_openrouter(&prompt, api_key),
            "openai" => self.call_openai(&prompt, api_key),
            "anthropic" => self.call_anthropic(&prompt, api_key),
            _ => Err(CleanupError::ProviderNotConfigured(
                self.config.provider.clone(),
            )),
        };

        if start.elapsed().as_millis() as u64 > self.config.timeout_ms {
            return Err(CleanupError::Timeout);
        }

        result
    }

    fn call_gemini(&self, prompt: &str, _api_key: &str) -> Result<String, CleanupError> {
        // TODO: POST https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent
        Ok(format!(
            "[Cleaned via Gemini: {}]",
            prompt.chars().take(50).collect::<String>()
        ))
    }

    fn call_openrouter(&self, prompt: &str, _api_key: &str) -> Result<String, CleanupError> {
        // TODO: POST https://openrouter.ai/api/v1/chat/completions
        Ok(format!(
            "[Cleaned via OpenRouter: {}]",
            prompt.chars().take(50).collect::<String>()
        ))
    }

    fn call_openai(&self, prompt: &str, _api_key: &str) -> Result<String, CleanupError> {
        // TODO: POST https://api.openai.com/v1/chat/completions
        Ok(format!(
            "[Cleaned via OpenAI: {}]",
            prompt.chars().take(50).collect::<String>()
        ))
    }

    fn call_anthropic(&self, prompt: &str, _api_key: &str) -> Result<String, CleanupError> {
        // TODO: POST https://api.anthropic.com/v1/messages
        Ok(format!(
            "[Cleaned via Claude: {}]",
            prompt.chars().take(50).collect::<String>()
        ))
    }

    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.config.enabled = enabled;
    }

    pub fn get_config(&self) -> &CleanupConfig {
        &self.config
    }
}

impl Default for CleanupProcessor {
    fn default() -> Self {
        Self::new(CleanupConfig::default())
    }
}

pub fn apply_auto_capitalization(text: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in text.chars() {
        if capitalize_next && c.is_alphabetic() {
            result.extend(c.to_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }

        if c == '.' || c == '!' || c == '?' {
            capitalize_next = true;
        }
    }

    result
}

pub fn apply_auto_punctuation(text: &str) -> String {
    let text = text.trim();
    if text.is_empty() {
        return text.to_string();
    }

    let mut result = text.to_string();

    if !result.ends_with('.') && !result.ends_with('!') && !result.ends_with('?') {
        result.push('.');
    }

    result
}

pub fn apply_text_replacements(text: &str, replacements: &[(String, String)]) -> String {
    let mut result = text.to_string();
    for (pattern, replacement) in replacements {
        result = result.replace(pattern, replacement);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_capitalization() {
        assert_eq!(apply_auto_capitalization("hello world"), "Hello world");
        assert_eq!(apply_auto_capitalization("hello. world"), "Hello. World");
    }

    #[test]
    fn test_auto_punctuation() {
        assert_eq!(apply_auto_punctuation("hello world"), "hello world.");
        assert_eq!(apply_auto_punctuation("hello world!"), "hello world!");
    }

    #[test]
    fn test_text_replacements() {
        let replacements = vec![
            ("at sign".to_string(), "@".to_string()),
            ("percent".to_string(), "%".to_string()),
        ];
        assert_eq!(
            apply_text_replacements("my at sign is here", &replacements),
            "my @ is here"
        );
    }

    #[test]
    fn test_cleanup_disabled() {
        let config = CleanupConfig {
            enabled: false,
            ..Default::default()
        };
        let processor = CleanupProcessor::new(config);
        let result = processor.process("test text", Some("key")).unwrap();
        assert_eq!(result, "test text");
    }
}

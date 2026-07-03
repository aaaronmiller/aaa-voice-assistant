use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LLMError {
    #[error("Provider initialization failed: {0}")]
    InitError(String),
    #[error("Generation failed: {0}")]
    GenerationError(String),
    #[error("Provider not found: {0}")]
    ProviderNotFound(String),
    #[error("No providers available")]
    NoProviders,
    #[error("API key not configured")]
    MissingApiKey,
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    #[error("Context too long")]
    ContextTooLong,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationContext {
    pub session_id: String,
    pub messages: Vec<Message>,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub model: String,
}

impl ConversationContext {
    pub fn new(session_id: &str, model: &str) -> Self {
        let now = Utc::now();
        Self {
            session_id: session_id.to_string(),
            messages: Vec::new(),
            created_at: now,
            last_activity: now,
            model: model.to_string(),
        }
    }

    pub fn add_message(&mut self, role: &str, content: &str) {
        self.messages.push(Message {
            role: role.to_string(),
            content: content.to_string(),
            timestamp: Utc::now(),
        });
        self.last_activity = Utc::now();
    }

    pub fn clear(&mut self) {
        self.messages.clear();
        self.last_activity = Utc::now();
    }

    pub fn is_expired(&self, timeout_minutes: i64) -> bool {
        let now = Utc::now();
        let diff = now.signed_duration_since(self.last_activity);
        diff.num_minutes() > timeout_minutes
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationResult {
    pub text: String,
    pub model: String,
    pub provider: String,
    pub tokens_used: Option<u32>,
    pub duration_ms: u64,
}

pub trait LLMProvider: Send + Sync {
    fn name(&self) -> &str;

    fn generate(
        &self,
        prompt: &str,
        context: &ConversationContext,
    ) -> Result<GenerationResult, LLMError>;

    fn supports_tools(&self) -> bool {
        false
    }

    fn is_available(&self) -> bool {
        true
    }
}

pub struct LLMBackend {
    providers: HashMap<String, Box<dyn LLMProvider>>,
    active_provider: String,
    active_model: String,
    contexts: HashMap<String, ConversationContext>,
    context_timeout_minutes: i64,
}

impl LLMBackend {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            active_provider: String::new(),
            active_model: String::new(),
            contexts: HashMap::new(),
            context_timeout_minutes: 5,
        }
    }

    pub fn register_provider(&mut self, provider: Box<dyn LLMProvider>) {
        let name = provider.name().to_string();
        if self.active_provider.is_empty() {
            self.active_provider = name.clone();
        }
        self.providers.insert(name, provider);
    }

    pub fn set_active_provider(&mut self, name: &str) -> Result<(), LLMError> {
        if self.providers.contains_key(name) {
            self.active_provider = name.to_string();
            Ok(())
        } else {
            Err(LLMError::ProviderNotFound(name.to_string()))
        }
    }

    pub fn set_model(&mut self, model: &str) {
        self.active_model = model.to_string();
    }

    pub fn query(&mut self, session_id: &str, prompt: &str) -> Result<GenerationResult, LLMError> {
        // Get or create context
        let context = self
            .contexts
            .entry(session_id.to_string())
            .or_insert_with(|| ConversationContext::new(session_id, &self.active_model));

        // Check if context expired
        if context.is_expired(self.context_timeout_minutes) {
            context.clear();
        }

        // Get provider
        let provider = self
            .providers
            .get(&self.active_provider)
            .ok_or_else(|| LLMError::ProviderNotFound(self.active_provider.clone()))?;

        if !provider.is_available() {
            return Err(LLMError::GenerationError(format!(
                "Provider {} is not available",
                self.active_provider
            )));
        }

        let result = provider.generate(prompt, context)?;

        // Add to context
        context.add_message("user", prompt);
        context.add_message("assistant", &result.text);

        Ok(result)
    }

    pub fn clear_context(&mut self, session_id: &str) {
        if let Some(context) = self.contexts.get_mut(session_id) {
            context.clear();
        }
    }

    pub fn list_providers(&self) -> Vec<&str> {
        self.providers.keys().map(|s| s.as_str()).collect()
    }

    pub fn get_active_provider(&self) -> &str {
        &self.active_provider
    }
}

impl Default for LLMBackend {
    fn default() -> Self {
        Self::new()
    }
}

// Gemini Provider stub
pub struct GeminiProvider {
    api_key: Option<String>,
    model: String,
}

impl GeminiProvider {
    pub fn new() -> Self {
        Self {
            api_key: None,
            model: "gemini-pro".to_string(),
        }
    }

    pub fn with_api_key(mut self, key: &str) -> Self {
        self.api_key = Some(key.to_string());
        self
    }

    pub fn with_model(mut self, model: &str) -> Self {
        self.model = model.to_string();
        self
    }
}

impl Default for GeminiProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl LLMProvider for GeminiProvider {
    fn name(&self) -> &str {
        "gemini"
    }

    fn generate(
        &self,
        prompt: &str,
        context: &ConversationContext,
    ) -> Result<GenerationResult, LLMError> {
        let api_key = self.api_key.as_ref().ok_or(LLMError::MissingApiKey)?;
        let start = std::time::Instant::now();

        // Build messages from context
        let _messages: Vec<serde_json::Value> = context
            .messages
            .iter()
            .map(|m| {
                serde_json::json!({
                    "role": m.role,
                    "parts": [{"text": m.content}]
                })
            })
            .collect();

        // TODO: Actual Gemini API call
        // POST https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent?key={api_key}
        let _ = api_key;

        Ok(GenerationResult {
            text: format!("[Gemini response to: {}]", prompt),
            model: self.model.clone(),
            provider: self.name().to_string(),
            tokens_used: Some(100),
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }

    fn is_available(&self) -> bool {
        self.api_key.is_some()
    }
}

// OpenRouter Provider stub
pub struct OpenRouterProvider {
    api_key: Option<String>,
    model: String,
}

impl OpenRouterProvider {
    pub fn new() -> Self {
        Self {
            api_key: None,
            model: "openai/gpt-4o".to_string(),
        }
    }

    pub fn with_api_key(mut self, key: &str) -> Self {
        self.api_key = Some(key.to_string());
        self
    }

    pub fn with_model(mut self, model: &str) -> Self {
        self.model = model.to_string();
        self
    }
}

impl Default for OpenRouterProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl LLMProvider for OpenRouterProvider {
    fn name(&self) -> &str {
        "openrouter"
    }

    fn generate(
        &self,
        prompt: &str,
        context: &ConversationContext,
    ) -> Result<GenerationResult, LLMError> {
        let api_key = self.api_key.as_ref().ok_or(LLMError::MissingApiKey)?;
        let start = std::time::Instant::now();

        // Build messages from context
        let _messages: Vec<serde_json::Value> = context
            .messages
            .iter()
            .map(|m| {
                serde_json::json!({
                    "role": m.role,
                    "content": m.content
                })
            })
            .collect();

        // TODO: Actual OpenRouter API call
        // POST https://openrouter.ai/api/v1/chat/completions
        let _ = api_key;

        Ok(GenerationResult {
            text: format!("[OpenRouter response to: {}]", prompt),
            model: self.model.clone(),
            provider: self.name().to_string(),
            tokens_used: Some(100),
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }

    fn supports_tools(&self) -> bool {
        true
    }

    fn is_available(&self) -> bool {
        self.api_key.is_some()
    }
}

pub struct OpenAIProvider {
    api_key: Option<String>,
    model: String,
    client: reqwest::Client,
}

impl OpenAIProvider {
    pub fn new() -> Self {
        Self {
            api_key: None,
            model: "gpt-4o".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub fn with_api_key(mut self, key: &str) -> Self {
        self.api_key = Some(key.to_string());
        self
    }

    pub fn with_model(mut self, model: &str) -> Self {
        self.model = model.to_string();
        self
    }
}

impl Default for OpenAIProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl LLMProvider for OpenAIProvider {
    fn name(&self) -> &str {
        "openai"
    }

    fn generate(
        &self,
        prompt: &str,
        context: &ConversationContext,
    ) -> Result<GenerationResult, LLMError> {
        let api_key = self.api_key.as_ref().ok_or(LLMError::MissingApiKey)?;
        let start = std::time::Instant::now();

        // Build messages from context
        let mut messages: Vec<serde_json::Value> = context
            .messages
            .iter()
            .map(|m| {
                serde_json::json!({
                    "role": m.role,
                    "content": m.content
                })
            })
            .collect();

        // Add the current prompt as a user message
        messages.push(serde_json::json!({
            "role": "user",
            "content": prompt
        }));

        // Make API request to OpenAI
        let request_body = serde_json::json!({
            "model": self.model,
            "messages": messages,
            "temperature": 0.7,
        });

        let client = reqwest::blocking::Client::new();
        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .map_err(|e| LLMError::GenerationError(format!("OpenAI request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            return Err(LLMError::GenerationError(format!(
                "OpenAI API error ({}): {}",
                status, body
            )));
        }

        let json: serde_json::Value = response
            .json()
            .map_err(|e| LLMError::GenerationError(format!("Failed to parse response: {}", e)))?;

        // Extract response text from OpenAI response
        let text = json
            .get("choices")
            .and_then(|c| c.as_array())
            .and_then(|arr| arr.first())
            .and_then(|choice| choice.get("message"))
            .and_then(|msg| msg.get("content"))
            .and_then(|c| c.as_str())
            .ok_or_else(|| LLMError::GenerationError("Invalid OpenAI response structure".to_string()))?
            .to_string();

        // Extract token usage
        let tokens_used = json
            .get("usage")
            .and_then(|u| u.get("total_tokens"))
            .and_then(|t| t.as_u64())
            .map(|t| t as u32);

        Ok(GenerationResult {
            text,
            model: self.model.clone(),
            provider: self.name().to_string(),
            tokens_used,
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }

    fn supports_tools(&self) -> bool {
        true
    }

    fn is_available(&self) -> bool {
        self.api_key.is_some()
    }
}

pub struct AnthropicProvider {
    api_key: Option<String>,
    model: String,
    client: reqwest::Client,
}

impl AnthropicProvider {
    pub fn new() -> Self {
        Self {
            api_key: None,
            model: "claude-3-5-sonnet-20241022".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub fn with_api_key(mut self, key: &str) -> Self {
        self.api_key = Some(key.to_string());
        self
    }

    pub fn with_model(mut self, model: &str) -> Self {
        self.model = model.to_string();
        self
    }
}

impl Default for AnthropicProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl LLMProvider for AnthropicProvider {
    fn name(&self) -> &str {
        "anthropic"
    }

    fn generate(
        &self,
        prompt: &str,
        context: &ConversationContext,
    ) -> Result<GenerationResult, LLMError> {
        let api_key = self.api_key.as_ref().ok_or(LLMError::MissingApiKey)?;
        let start = std::time::Instant::now();

        // Build messages for Anthropic API (expects roles to be "user" or "assistant")
        let messages: Vec<serde_json::Value> = context
            .messages
            .iter()
            .chain(std::iter::once(&Message {
                role: "user".to_string(),
                content: prompt.to_string(),
                timestamp: Utc::now(),
            }))
            .map(|m| {
                // Convert "system" role to "user" for Anthropic compatibility
                let role = if m.role == "system" { "user" } else { &m.role };
                serde_json::json!({
                    "role": role,
                    "content": m.content
                })
            })
            .collect();

        // Make API request to Anthropic
        let request_body = serde_json::json!({
            "model": self.model,
            "messages": messages,
            "max_tokens": 4096,
            "temperature": 0.7,
        });

        let client = reqwest::blocking::Client::new();
        let response = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .map_err(|e| LLMError::GenerationError(format!("Anthropic request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            return Err(LLMError::GenerationError(format!(
                "Anthropic API error ({}): {}",
                status, body
            )));
        }

        let json: serde_json::Value = response
            .json()
            .map_err(|e| LLMError::GenerationError(format!("Failed to parse response: {}", e)))?;

        // Extract response text from Anthropic response
        let text = json
            .get("content")
            .and_then(|c| c.as_array())
            .and_then(|arr| arr.first())
            .and_then(|block| block.get("text"))
            .and_then(|t| t.as_str())
            .ok_or_else(|| {
                LLMError::GenerationError("Invalid Anthropic response structure".to_string())
            })?
            .to_string();

        // Extract token usage
        let tokens_used = json
            .get("usage")
            .and_then(|u| u.get("output_tokens"))
            .and_then(|t| t.as_u64())
            .map(|t| t as u32);

        Ok(GenerationResult {
            text,
            model: self.model.clone(),
            provider: self.name().to_string(),
            tokens_used,
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }

    fn supports_tools(&self) -> bool {
        true
    }

    fn is_available(&self) -> bool {
        self.api_key.is_some()
    }
}

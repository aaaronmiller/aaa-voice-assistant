use crate::providers::llm::{
    ConversationContext, GeminiProvider, LLMBackend, LLMProvider, OpenRouterProvider,
};
use proptest::prelude::*;

#[test]
fn test_context_creation() {
    let ctx = ConversationContext::new("test-session", "gemini-pro");
    assert_eq!(ctx.session_id, "test-session");
    assert_eq!(ctx.model, "gemini-pro");
    assert!(ctx.messages.is_empty());
}

#[test]
fn test_context_add_message() {
    let mut ctx = ConversationContext::new("test", "model");
    ctx.add_message("user", "Hello");
    ctx.add_message("assistant", "Hi there!");

    assert_eq!(ctx.messages.len(), 2);
    assert_eq!(ctx.messages[0].role, "user");
    assert_eq!(ctx.messages[1].role, "assistant");
}

#[test]
fn test_context_clear() {
    let mut ctx = ConversationContext::new("test", "model");
    ctx.add_message("user", "Hello");
    ctx.clear();

    assert!(ctx.messages.is_empty());
}

#[test]
fn test_context_expiration() {
    let ctx = ConversationContext::new("test", "model");
    assert!(!ctx.is_expired(5));
}

#[test]
fn test_llm_backend_creation() {
    let backend = LLMBackend::new();
    assert!(backend.get_active_provider().is_empty());
}

#[test]
fn test_provider_registration() {
    let mut backend = LLMBackend::new();
    backend.register_provider(Box::new(GeminiProvider::default()));

    let providers = backend.list_providers();
    assert!(providers.contains(&"gemini"));
}

#[test]
fn test_provider_availability() {
    let provider = GeminiProvider::default();
    assert!(!provider.is_available());

    let provider = GeminiProvider::new().with_api_key("test-key");
    assert!(provider.is_available());
}

proptest! {
    #[test]
    fn test_session_id_persistence(session_id in "[a-zA-Z0-9-]+") {
        let ctx = ConversationContext::new(&session_id, "test-model");
        prop_assert_eq!(ctx.session_id, session_id);
    }
}

proptest! {
    #[test]
    fn test_message_content(content in ".*") {
        let mut ctx = ConversationContext::new("test", "model");
        ctx.add_message("user", &content);

        prop_assert_eq!(ctx.messages.len(), 1);
        prop_assert_eq!(ctx.messages[0].content, content);
    }
}

#[test]
fn test_openrouter_supports_tools() {
    let provider = OpenRouterProvider::default();
    assert!(provider.supports_tools());
}

#[test]
fn test_gemini_does_not_support_tools_by_default() {
    let provider = GeminiProvider::default();
    assert!(!provider.supports_tools());
}

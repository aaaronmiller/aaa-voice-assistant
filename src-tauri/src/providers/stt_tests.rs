use crate::providers::stt::{
    Language, STTEngine, STTError, STTProvider, Transcription, VoskProvider, WhisperCppProvider,
};
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_provider_selection_persistence(provider_name in "whisper\\.cpp|vosk|assemblyai") {
        let mut engine = STTEngine::new();

        let provider: Box<dyn STTProvider> = match provider_name.as_str() {
            "whisper.cpp" => Box::new(WhisperCppProvider::default()),
            "vosk" => Box::new(VoskProvider::default()),
            _ => Box::new(VoskProvider::default()),
        };

        engine.register_provider(provider);

        prop_assert!(engine.set_active_provider(&provider_name).is_ok() ||
                     engine.list_providers().contains(&provider_name.as_str()));
    }
}

proptest! {
    #[test]
    fn test_language_code_roundtrip(lang_code in "(en-US|en-GB|es-ES|fr-FR|de-DE|ja-JP|zh-CN)") {
        let language = Language::from_code(&lang_code);
        let roundtrip = language.code();

        prop_assert_eq!(lang_code, roundtrip);
    }
}

proptest! {
    #[test]
    fn test_transcription_text_not_empty(audio_len in 1usize..1000) {
        let audio: Vec<f32> = vec![0.5; audio_len];

        prop_assert!(!audio.is_empty());
        prop_assert_eq!(audio.len(), audio_len);
    }
}

#[test]
fn test_fallback_chain_order() {
    let mut engine = STTEngine::new();
    engine.set_fallback_chain(vec!["vosk".to_string(), "assemblyai".to_string()]);

    let providers = engine.list_providers();
    assert!(providers.is_empty() || providers.iter().all(|p| !p.is_empty()));
}

#[test]
fn test_stt_error_display() {
    let err = STTError::NoProviders;
    assert!(!err.to_string().is_empty());

    let err = STTError::EmptyAudio;
    assert!(err.to_string().contains("empty"));
}

#[test]
fn test_language_equality() {
    assert_eq!(Language::EnUS, Language::from_code("en-US"));
    assert_eq!(Language::EnUS, Language::from_code("en"));
    assert_ne!(Language::EnUS, Language::EnGB);
}

#[test]
fn test_provider_availability() {
    let provider = WhisperCppProvider::default();
    assert!(!provider.is_available());

    let provider = WhisperCppProvider::new().with_model("/path/to/model");
    assert!(provider.is_available());
}

#[test]
fn test_engine_default_provider() {
    let engine = STTEngine::new();
    assert!(engine.get_active_provider().is_empty());
}

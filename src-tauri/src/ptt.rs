use std::sync::{Arc, Mutex};
use thiserror::Error;

use crate::audio::AudioManager;
use crate::clipboard::ClipboardManager;
use crate::providers::{Language, STTEngine};

#[derive(Error, Debug)]
pub enum PTTError {
    #[error("Recording failed: {0}")]
    RecordingError(String),
    #[error("Transcription failed: {0}")]
    TranscriptionError(String),
    #[error("Clipboard error: {0}")]
    ClipboardError(String),
    #[error("Already recording")]
    AlreadyRecording,
    #[error("Not recording")]
    NotRecording,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PTTMode {
    PressHoldRelease,
    PressStartStop,
}

pub struct PTTHandler {
    mode: PTTMode,
    is_recording: bool,
    audio_manager: Arc<Mutex<AudioManager>>,
    clipboard_manager: Arc<Mutex<ClipboardManager>>,
    stt_engine: Arc<Mutex<STTEngine>>,
}

impl PTTHandler {
    pub fn new(
        audio_manager: Arc<Mutex<AudioManager>>,
        clipboard_manager: Arc<Mutex<ClipboardManager>>,
        stt_engine: Arc<Mutex<STTEngine>>,
    ) -> Self {
        Self {
            mode: PTTMode::PressHoldRelease,
            is_recording: false,
            audio_manager,
            clipboard_manager,
            stt_engine,
        }
    }

    pub fn set_mode(&mut self, mode: PTTMode) {
        self.mode = mode;
    }

    pub fn get_mode(&self) -> PTTMode {
        self.mode
    }

    pub fn is_recording(&self) -> bool {
        self.is_recording
    }

    pub fn on_hotkey_press(&mut self) -> Result<(), PTTError> {
        match self.mode {
            PTTMode::PressHoldRelease => {
                if self.is_recording {
                    return Err(PTTError::AlreadyRecording);
                }
                self.start_recording()
            }
            PTTMode::PressStartStop => {
                if self.is_recording {
                    self.stop_and_process()
                } else {
                    self.start_recording()
                }
            }
        }
    }

    pub fn on_hotkey_release(&mut self) -> Result<(), PTTError> {
        match self.mode {
            PTTMode::PressHoldRelease => {
                if !self.is_recording {
                    return Err(PTTError::NotRecording);
                }
                self.stop_and_process()
            }
            PTTMode::PressStartStop => {
                // In toggle mode, release does nothing
                Ok(())
            }
        }
    }

    fn start_recording(&mut self) -> Result<(), PTTError> {
        // Save clipboard first
        self.clipboard_manager
            .lock()
            .map_err(|e| PTTError::ClipboardError(e.to_string()))?
            .save()
            .map_err(|e| PTTError::ClipboardError(e.to_string()))?;

        // Start audio recording
        self.audio_manager
            .lock()
            .map_err(|e| PTTError::RecordingError(e.to_string()))?
            .start_recording()
            .map_err(|e| PTTError::RecordingError(e.to_string()))?;

        self.is_recording = true;
        Ok(())
    }

    fn stop_and_process(&mut self) -> Result<(), PTTError> {
        // Stop recording
        let audio_data = self
            .audio_manager
            .lock()
            .map_err(|e| PTTError::RecordingError(e.to_string()))?
            .stop_recording()
            .map_err(|e| PTTError::RecordingError(e.to_string()))?;

        self.is_recording = false;

        // Transcribe
        let transcription = self
            .stt_engine
            .lock()
            .map_err(|e| PTTError::TranscriptionError(e.to_string()))?
            .transcribe(&audio_data, &Language::EnUS)
            .map_err(|e| PTTError::TranscriptionError(e.to_string()))?;

        // Paste the text
        self.clipboard_manager
            .lock()
            .map_err(|e| PTTError::ClipboardError(e.to_string()))?
            .paste_text(&transcription.text)
            .map_err(|e| PTTError::ClipboardError(e.to_string()))?;

        // Restore original clipboard
        self.clipboard_manager
            .lock()
            .map_err(|e| PTTError::ClipboardError(e.to_string()))?
            .restore()
            .map_err(|e| PTTError::ClipboardError(e.to_string()))?;

        Ok(())
    }

    pub fn cancel_recording(&mut self) -> Result<(), PTTError> {
        if !self.is_recording {
            return Err(PTTError::NotRecording);
        }

        // Stop recording without processing
        self.audio_manager
            .lock()
            .map_err(|e| PTTError::RecordingError(e.to_string()))?
            .stop_recording()
            .map_err(|e| PTTError::RecordingError(e.to_string()))?;

        // Restore clipboard
        self.clipboard_manager
            .lock()
            .map_err(|e| PTTError::ClipboardError(e.to_string()))?
            .restore()
            .map_err(|e| PTTError::ClipboardError(e.to_string()))?;

        self.is_recording = false;
        Ok(())
    }
}

use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AudioError {
    #[error("No audio devices found")]
    NoDevicesError,
    #[error("Failed to record audio: {0}")]
    RecordError(String),
    #[error("Failed to play audio: {0}")]
    PlayError(String),
    #[error("Audio not available: {0}")]
    NotAvailable(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
    pub sample_rate: u32,
    pub channels: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioLevels {
    pub rms: f32,
    pub peak: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioPreprocessor {
    pub noise_suppression: bool,
    pub auto_gain_control: bool,
    pub silence_threshold: f32,
}

impl Default for AudioPreprocessor {
    fn default() -> Self {
        Self {
            noise_suppression: true,
            auto_gain_control: true,
            silence_threshold: 0.1,
        }
    }
}

#[derive(Debug)]
pub struct AudioManager {
    input_device: Option<AudioDevice>,
    output_device: Option<AudioDevice>,
    preprocessor: AudioPreprocessor,
    is_recording: bool,
    audio_buffer: Arc<Mutex<Vec<f32>>>,
}

impl AudioManager {
    pub fn new() -> Result<Self, AudioError> {
        Ok(Self {
            input_device: None,
            output_device: None,
            preprocessor: AudioPreprocessor::default(),
            is_recording: false,
            audio_buffer: Arc::new(Mutex::new(Vec::new())),
        })
    }

    pub fn list_input_devices() -> Result<Vec<AudioDevice>, AudioError> {
        Ok(vec![])
    }

    pub fn list_output_devices() -> Result<Vec<AudioDevice>, AudioError> {
        Ok(vec![])
    }

    pub fn set_input_device(&mut self, device: AudioDevice) {
        self.input_device = Some(device);
    }

    pub fn set_output_device(&mut self, device: AudioDevice) {
        self.output_device = Some(device);
    }

    pub fn get_input_device(&self) -> Option<&AudioDevice> {
        self.input_device.as_ref()
    }

    pub fn get_output_device(&self) -> Option<&AudioDevice> {
        self.output_device.as_ref()
    }

    pub fn start_recording(&mut self) -> Result<(), AudioError> {
        if self.is_recording {
            return Ok(());
        }
        self.is_recording = true;
        self.audio_buffer.lock().unwrap().clear();
        Ok(())
    }

    pub fn stop_recording(&mut self) -> Result<Vec<f32>, AudioError> {
        if !self.is_recording {
            return Ok(Vec::new());
        }
        self.is_recording = false;
        let mut buffer = self.audio_buffer.lock().unwrap();
        let result = buffer.clone();
        buffer.clear();
        Ok(result)
    }

    pub fn play_audio(&self, _audio: &[f32]) -> Result<(), AudioError> {
        Ok(())
    }

    pub fn get_audio_levels(&self) -> AudioLevels {
        let buffer = self.audio_buffer.lock().unwrap();
        if buffer.is_empty() {
            return AudioLevels {
                rms: 0.0,
                peak: 0.0,
            };
        }
        let rms = buffer.iter().map(|&x| x * x).sum::<f32>().sqrt() / buffer.len() as f32;
        let peak = buffer.iter().map(|&x| x.abs()).fold(0.0, f32::max);
        AudioLevels { rms, peak }
    }

    pub fn set_preprocessor(&mut self, preprocessor: AudioPreprocessor) {
        self.preprocessor = preprocessor;
    }

    pub fn is_recording(&self) -> bool {
        self.is_recording
    }
}

/// Events that occur during audio device hot-swapping.
#[derive(Debug, Clone)]
pub enum DeviceChange {
    /// The current input device was switched to a new available device.
    Switched(AudioDevice),
    /// The current input device was disconnected and no replacement is available.
    Disconnected,
}

impl AudioManager {
    /// Checks for audio device changes and handles hot-swapping.
    /// If the current input device is no longer available, this method will:
    /// - Stop recording if active (but preserve the audio buffer).
    /// - Select a fallback input device from the available list, if any.
    /// Returns `Some(DeviceChange)` if a change occurred, otherwise `None`.
    pub fn check_device_changes(&mut self) -> Option<DeviceChange> {
        // Get the current list of available input devices.
        let available = match Self::list_input_devices() {
            Ok(devices) => devices,
            Err(_) => return None,
        };

        // Determine the appropriate device change using pure logic.
        let change = {
            let current = &self.input_device;
            Self::determine_device_change(current, &available)
        };

        if let Some(change) = change {
            match change {
                DeviceChange::Switched(ref new_device) => {
                    // If switching to a new device, update input device.
                    self.input_device = Some(new_device.clone());
                }
                DeviceChange::Disconnected => {
                    // No devices available; clear current input device.
                    self.input_device = None;
                }
            }
            // If we were recording, stop the recording session gracefully.
            if self.is_recording {
                self.is_recording = false;
                // Note: We intentionally do NOT clear the audio buffer here,
                // as the captured audio up to this point may still be useful.
            }
            Some(change)
        } else {
            None
        }
    }

    /// Pure function that determines what device change, if any, should occur
    /// given the current device and the list of available devices.
    /// This is testable without side effects.
    pub(crate) fn determine_device_change(
        current: &Option<AudioDevice>,
        available: &[AudioDevice],
    ) -> Option<DeviceChange> {
        match current {
            Some(dev) => {
                // Check if the current device is still present by ID.
                if available.iter().any(|d| d.id == dev.id) {
                    None
                } else {
                    // Current device missing; try to fall back to the first available.
                    if let Some(first) = available.first() {
                        Some(DeviceChange::Switched(first.clone()))
                    } else {
                        Some(DeviceChange::Disconnected)
                    }
                }
            }
            None => {
                // No current device; if devices are available we could auto-select,
                // but the spec does not require auto-selection when none set.
                // We return None to indicate no forced change.
                None
            }
        }
    }
}

impl Default for AudioManager {
    fn default() -> Self {
        Self::new().expect("Failed to create AudioManager")
    }
}

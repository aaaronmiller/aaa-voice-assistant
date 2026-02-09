use std::sync::{Arc, Mutex};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rodio::OutputStream;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AudioError {
    #[error("Failed to get default host: {0}")]
    HostError(#[from] cpal::HostUnavailable),
    #[error("No audio devices found")]
    NoDevicesError,
    #[error("Failed to get device info: {0}")]
    DeviceInfoError(#[from] cpal::DeviceError),
    #[error("Failed to build input stream: {0}")]
    BuildInputStreamError(#[from] cpal::BuildStreamError),
    #[error("Failed to build output stream: {0}")]
    BuildOutputStreamError(#[from] cpal::BuildStreamError),
    #[error("Failed to play audio: {0}")]
    PlayError(#[from] rodio::PlayError),
    #[error("Failed to record audio: {0}")]
    RecordError(#[from] std::io::Error),
}

#[derive(Debug, Clone)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
    pub sample_rate: u32,
    pub channels: u16,
}

#[derive(Debug)]
pub struct AudioLevels {
    pub rms: f32,
    pub peak: f32,
}

#[derive(Debug)]
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
        let host = cpal::default_host();
        let devices = host.input_devices()?;
        
        let mut result = Vec::new();
        for device in devices {
            let id = device.id().to_string();
            let name = device.name().unwrap_or_else(|_| "Unknown Device".to_string());
            
            let config = device.default_input_config()?;
            let sample_rate = config.sample_rate().0;
            let channels = config.channels();
            
            result.push(AudioDevice {
                id,
                name,
                sample_rate,
                channels,
            });
        }
        
        Ok(result)
    }

    pub fn list_output_devices() -> Result<Vec<AudioDevice>, AudioError> {
        let host = cpal::default_host();
        let devices = host.output_devices()?;
        
        let mut result = Vec::new();
        for device in devices {
            let id = device.id().to_string();
            let name = device.name().unwrap_or_else(|_| "Unknown Device".to_string());
            
            let config = device.default_output_config()?;
            let sample_rate = config.sample_rate().0;
            let channels = config.channels();
            
            result.push(AudioDevice {
                id,
                name,
                sample_rate,
                channels,
            });
        }
        
        Ok(result)
    }

    pub fn set_input_device(&mut self, device_id: &str) -> Result<(), AudioError> {
        let devices = Self::list_input_devices()?;
        let device = devices
            .into_iter()
            .find(|d| d.id == device_id)
            .ok_or(AudioError::NoDevicesError)?;
        
        self.input_device = Some(device);
        Ok(())
    }

    pub fn set_output_device(&mut self, device_id: &str) -> Result<(), AudioError> {
        let devices = Self::list_output_devices()?;
        let device = devices
            .into_iter()
            .find(|d| d.id == device_id)
            .ok_or(AudioError::NoDevicesError)?;
        
        self.output_device = Some(device);
        Ok(())
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

        let host = cpal::default_host();
        let device = match &self.input_device {
            Some(dev) => host
                .input_devices()?
                .find(|d| d.id().to_string() == dev.id)
                .ok_or(AudioError::NoDevicesError)?,
            None => host.default_input_device().ok_or(AudioError::NoDevicesError)?,
        };

        let config = device.default_input_config()?;
        let buffer = Arc::clone(&self.audio_buffer);

        let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut buffer = buffer.lock().unwrap();
            buffer.extend_from_slice(data);
        };

        let err_fn = move |err| {
            eprintln!("Error in recording: {:?}", err);
        };

        let stream = device.build_input_stream(&config.into(), input_data_fn, err_fn, None)?;
        stream.play()?;

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

    pub fn play_audio(&self, audio: &[f32]) -> Result<(), AudioError> {
        let (_stream, handle) = OutputStream::try_default()?;
        let sink = rodio::Sink::try_new(&handle)?;

        let source = rodio::source::Buffer::new(audio.len() as u32, 16000, audio);
        sink.append(source);
        sink.sleep_until_end();

        Ok(())
    }

    pub fn get_audio_levels(&self) -> AudioLevels {
        let buffer = self.audio_buffer.lock().unwrap();
        
        if buffer.is_empty() {
            return AudioLevels { rms: 0.0, peak: 0.0 };
        }

        let rms = buffer
            .iter()
            .map(|&x| x * x)
            .sum::<f32>()
            .sqrt()
            / buffer.len() as f32;
        
        let peak = buffer.iter().map(|&x| x.abs()).fold(0.0, f32::max);

        AudioLevels { rms, peak }
    }
}

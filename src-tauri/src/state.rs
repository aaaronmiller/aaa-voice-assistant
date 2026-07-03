use std::sync::{Arc, Mutex};

use crate::config::ConfigurationStore;
use crate::hardware::GpuBackend;
use crate::history::HistoryManager;
use crate::hotkey::HotkeyManager;
use crate::metrics::{MetricsTracker, ResourceMonitor};

pub struct AppState {
    pub config: Arc<Mutex<ConfigurationStore>>,
    pub hotkey_manager: HotkeyManager,
    pub history: Arc<Mutex<HistoryManager>>,
    pub metrics: Arc<Mutex<MetricsTracker>>,
    pub resource_monitor: Arc<Mutex<ResourceMonitor>>,
    pub gpu_backend: GpuBackend,
    pub is_recording: Arc<Mutex<bool>>,
    pub audio_buffer: Arc<Mutex<Vec<f32>>>,
}

impl AppState {
    pub fn new(gpu_backend: GpuBackend) -> Self {
        let config = ConfigurationStore::default();
        let hotkey_manager = HotkeyManager::new().expect("Failed to create hotkey manager");
        let history = HistoryManager::new().expect("Failed to create history manager");
        let metrics = MetricsTracker::new().expect("Failed to create metrics tracker");
        let resource_monitor = ResourceMonitor::new();

        Self {
            config: Arc::new(Mutex::new(config)),
            hotkey_manager,
            history: Arc::new(Mutex::new(history)),
            metrics: Arc::new(Mutex::new(metrics)),
            resource_monitor: Arc::new(Mutex::new(resource_monitor)),
            gpu_backend,
            is_recording: Arc::new(Mutex::new(false)),
            audio_buffer: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

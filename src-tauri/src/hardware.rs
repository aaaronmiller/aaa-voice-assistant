use std::process::Command;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub enum GpuBackend {
    IntelArcOptimized, // Target: 0x5693
    UniversalWhisper,  // Target: iGPU 0xa7a0 or others
}

pub fn detect_gpu_backend() -> GpuBackend {
    let output = Command::new("clinfo")
        .arg("-l")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();

    if output.contains("0x5693") {
        GpuBackend::IntelArcOptimized
    } else {
        GpuBackend::UniversalWhisper
    }
}

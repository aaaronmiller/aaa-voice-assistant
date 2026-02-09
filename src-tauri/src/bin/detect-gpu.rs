use voice_assistant::hardware;

fn main() {
    println!("Detecting GPU backend...");
    
    match hardware::detect_gpu_backend() {
        hardware::GpuBackend::IntelArcOptimized => {
            println!("✓ Intel Arc GPU detected (0x5693)");
        }
        hardware::GpuBackend::UniversalWhisper => {
            println!("✓ Universal GPU/CPU backend detected");
        }
    }
    
    println!("Detection complete");
}

use std::process::Command;
use std::str;

#[derive(Debug)]
pub enum GpuBackend {
    IntelArcOptimized,
    UniversalWhisper,
}

pub fn detect_gpu_backend() -> GpuBackend {
    if is_intel_arc_gpu() {
        GpuBackend::IntelArcOptimized
    } else {
        GpuBackend::UniversalWhisper
    }
}

fn is_intel_arc_gpu() -> bool {
    let clinfo_output = run_clinfo();
    
    match clinfo_output {
        Ok(output) => {
            let contains_arc_gpu = output.to_lowercase().contains("a370m") || 
                                  output.to_lowercase().contains("intel arc") ||
                                  output.to_lowercase().contains("0x5693");
            
            if contains_arc_gpu {
                println!("✓ Intel Arc GPU detected in clinfo output");
            } else {
                println!("✓ No Intel Arc GPU detected in clinfo output");
            }
            
            contains_arc_gpu
        },
        Err(e) => {
            println!("⚠️ Failed to run clinfo: {}", e);
            false
        }
    }
}

fn run_clinfo() -> Result<String, String> {
    let output = Command::new("clinfo")
        .output()
        .map_err(|e| format!("Failed to execute clinfo: {}", e))?;
    
    if output.status.success() {
        Ok(str::from_utf8(&output.stdout)
            .map_err(|e| format!("Failed to parse clinfo output: {}", e))?.to_string())
    } else {
        Err(format!("clinfo exited with code {}", output.status))
    }
}

fn main() {
    println!("Detecting GPU backend...");
    
    match detect_gpu_backend() {
        GpuBackend::IntelArcOptimized => {
            println!("✓ Intel Arc GPU detected (0x5693)");
        }
        GpuBackend::UniversalWhisper => {
            println!("✓ Universal GPU/CPU backend detected");
        }
    }
    
    println!("Detection complete");
}
# setup_whisper_cpp.ps1
Write-Host "Setting up whisper.cpp for Intel Arc GPU (OpenVINO)..."

if (-not (Test-Path "whisper.cpp")) {
    Write-Host "Cloning whisper.cpp repository..."
    git clone https://github.com/ggerganov/whisper.cpp.git
}

Set-Location whisper.cpp

# Download models
Write-Host "Downloading base model..."
if (Test-Path "models/download-ggml-model.cmd") {
    cmd /c "models\download-ggml-model.cmd base"
} elseif (Test-Path "models/download-ggml-model.sh") {
    bash ./models/download-ggml-model.sh base
} else {
    Write-Host "Could not find model download script. Please download ggml-base.bin to models/ folder manually."
}

Write-Host "Building with OpenVINO support..."
Write-Host "Ensure you have Intel OpenVINO Toolkit installed and environment variables set (run setupvars.bat)."
Write-Host "Ensure CMake is installed."

# Create build directory
if (-not (Test-Path "build")) {
    New-Item -ItemType Directory -Force -Path "build"
}
Set-Location build

# CMake build
# Note: Users might need to specify generator or OpenVINO paths if not in PATH.
try {
    cmake .. -DWHISPER_OPENVINO=1 -DCMAKE_BUILD_TYPE=Release
    cmake --build . --config Release
    Write-Host "Build process finished. Check Release/main.exe or Debug/main.exe depending on generator."
} catch {
    Write-Host "Error during build. Please check CMake and OpenVINO installation."
    Write-Host $_
}

Set-Location ..\..

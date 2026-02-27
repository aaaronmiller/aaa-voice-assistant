import os
import subprocess
import sys
import platform
import shutil

def detect_gpu():
    print("Detecting GPU...")
    # Check for NVIDIA
    try:
        subprocess.run(["nvidia-smi"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, check=True)
        print("NVIDIA GPU detected.")
        return "cuda"
    except (subprocess.CalledProcessError, FileNotFoundError):
        pass

    # Check for AMD (rocm)
    try:
        subprocess.run(["rocminfo"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, check=True)
        print("AMD GPU detected (ROCm).")
        return "rocm"
    except (subprocess.CalledProcessError, FileNotFoundError):
        pass

    # Check for Intel (clinfo or similar, simpler to ask user or assume OpenVINO if on Windows/Intel CPU)
    # On Windows, user specifically mentioned Arc.
    # We can check platform processor string?
    if "Intel" in platform.processor() or "Intel" in platform.machine():
         # Check if OpenVINO is installed?
         if os.environ.get("INTEL_OPENVINO_DIR"):
             print("Intel OpenVINO environment detected.")
             return "openvino"
         else:
             print("Intel CPU detected. OpenVINO is recommended for Arc/iGPU.")
             return "openvino_prompt"

    return "cpu"

def install_dependencies():
    print("Installing Python dependencies...")
    subprocess.check_call([sys.executable, "-m", "pip", "install", "-r", "requirements.txt"])

def setup_whisper_cpp(gpu_type):
    print(f"Setting up whisper.cpp for {gpu_type}...")
    if not os.path.exists("whisper.cpp"):
        subprocess.check_call(["git", "clone", "https://github.com/ggerganov/whisper.cpp.git"])

    os.chdir("whisper.cpp")

    # Download model
    model = "base"
    if platform.system() == "Windows":
        script = os.path.join("models", "download-ggml-model.cmd")
        subprocess.check_call([script, model], shell=True)
    else:
        script = os.path.join("models", "download-ggml-model.sh")
        subprocess.check_call(["bash", script, model])

    # Build
    os.makedirs("build", exist_ok=True)
    os.chdir("build")

    cmake_args = []
    if gpu_type == "cuda":
        cmake_args.append("-DWHISPER_CUBLAS=1")
    elif gpu_type == "rocm":
        cmake_args.append("-DWHISPER_HIPBLAS=1")
    elif gpu_type == "openvino":
        cmake_args.append("-DWHISPER_OPENVINO=1")
    elif gpu_type == "metal" or platform.system() == "Darwin":
        # Metal is default on Mac usually, but explicit flag helps
        # cmake_args.append("-DWHISPER_METAL=1")
        pass

    print(f"Running CMake with args: {cmake_args}")
    subprocess.check_call(["cmake", ".."] + cmake_args)
    subprocess.check_call(["cmake", "--build", ".", "--config", "Release"])

    os.chdir("../..")

def setup_openclaw():
    print("\n--- OpenClaw Setup ---")
    print("OpenClaw is an autonomous agent that runs in Docker.")
    choice = input("Do you want to clone and setup OpenClaw? (y/n): ").lower()
    if choice == 'y':
        if not os.path.exists("openclaw"):
            subprocess.check_call(["git", "clone", "https://github.com/openclaw/openclaw.git"])

        print("Please follow the OpenClaw documentation to run 'docker-compose up'.")
        print("This script can attempt to start it if Docker is installed.")
        start = input("Attempt to start OpenClaw via Docker Compose? (y/n): ").lower()
        if start == 'y':
            try:
                os.chdir("openclaw")
                subprocess.check_call(["docker-compose", "up", "-d"])
                print("OpenClaw started in background.")
            except Exception as e:
                print(f"Failed to start OpenClaw: {e}")
            finally:
                os.chdir("..")

def main():
    print("Welcome to AAA Voice Assistant Setup")

    # install_dependencies()

    gpu = detect_gpu()
    if gpu == "openvino_prompt":
        use_ov = input("Intel CPU detected. Enable OpenVINO optimization (requires OpenVINO toolkit)? (y/n): ").lower()
        gpu = "openvino" if use_ov == 'y' else "cpu"

    setup_whisper_cpp(gpu)

    setup_openclaw()

    print("\nSetup Complete!")
    print("Edit 'config.json' to set API keys and preferences.")
    print("Run 'python src/main.py' to start.")

if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""
Unified Quickstart Script for Voice Assistant
Detects OS, checks dependencies, runs setup, and starts the assistant.
"""

import sys
import os
import subprocess
import platform
import shutil

def check_command(cmd):
    return shutil.which(cmd) is not None

def install_system_dependencies():
    system = platform.system()
    if system == "Linux":
        print("Checking Linux system dependencies...")
        # Check for apt-get (Debian/Ubuntu)
        if check_command("apt-get"):
            # Check for portaudio and xclip (for clipboard)
            pkgs = []
            if not check_command("xclip") and not check_command("xsel"):
                pkgs.append("xclip")

            # Check for portaudio headers (tricky to check directly, assume needed if pyaudio fails)
            # But let's be proactive
            try:
                import pyaudio
            except ImportError:
                # likely need portaudio19-dev
                pkgs.append("portaudio19-dev")
                pkgs.append("python3-dev")

            if pkgs:
                print(f"Missing system packages: {', '.join(pkgs)}")
                choice = input("Install them with sudo apt-get? (y/n): ").lower()
                if choice == 'y':
                    subprocess.check_call(["sudo", "apt-get", "update"])
                    subprocess.check_call(["sudo", "apt-get", "install", "-y"] + pkgs)
    elif system == "Darwin":
        print("Checking macOS system dependencies...")
        if check_command("brew"):
            pkgs = []
            if not check_command("portaudio"):
                pkgs.append("portaudio")

            if pkgs:
                print(f"Missing brew packages: {', '.join(pkgs)}")
                choice = input("Install them with brew? (y/n): ").lower()
                if choice == 'y':
                    subprocess.check_call(["brew", "install"] + pkgs)
        else:
            print("Homebrew not found. Please install portaudio manually if PyAudio fails.")

def main():
    print("=== Voice Assistant Quickstart ===")

    # 1. Check Python version
    if sys.version_info < (3, 8):
        print("Error: Python 3.8+ is required.")
        sys.exit(1)

    # 2. Check basic tools
    if not check_command("git"):
        print("Error: git is required.")
        sys.exit(1)

    if not check_command("cmake"):
        print("Warning: cmake not found. Whisper.cpp build might fail.")
        input("Press Enter to continue anyway...")

    # 3. Install System Dependencies (Linux/Mac)
    install_system_dependencies()

    # 4. Run Setup Assistant
    print("\n--- Running Setup Wizard ---")
    try:
        subprocess.check_call([sys.executable, "setup_assistant.py"])
    except subprocess.CalledProcessError:
        print("Setup failed.")
        sys.exit(1)

    # 5. Start Assistant
    print("\n--- Starting Assistant ---")
    try:
        subprocess.check_call([sys.executable, "src/main.py"])
    except KeyboardInterrupt:
        print("\nExiting.")

if __name__ == "__main__":
    main()

import threading
import sys
import os
import json
import time

# Add src to path if needed (but running as module `python -m src.main` handles it)
sys.path.append(os.path.dirname(os.path.abspath(__file__)))

from assistant_logic import Assistant
from gui import TrayIcon
from config_manager import ConfigManager

def main():
    print("Initializing AAA Voice Assistant...")
    # Use ConfigManager to load config properly
    cm = ConfigManager()
    config = cm.config

    assistant = Assistant(config)

    try:
        assistant.start()
    except Exception as e:
        print(f"Failed to start assistant: {e}")
        return

    tray = TrayIcon(assistant)

    print("Running system tray icon...")
    try:
        tray.run()
    except KeyboardInterrupt:
        assistant.stop()
        print("Exiting.")

if __name__ == "__main__":
    main()

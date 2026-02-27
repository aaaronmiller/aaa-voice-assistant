import argparse
import sys
import os
import json
from src.config_manager import ConfigManager
from src.main import main as run_assistant
from setup_assistant import main as run_setup
from src.startup_manager import StartupManager

def list_voices():
    # Placeholder for TTS voice listing
    # System TTS might provide getProperty('voices')
    try:
        import pyttsx3
        engine = pyttsx3.init()
        voices = engine.getProperty('voices')
        for v in voices:
            print(f"ID: {v.id}, Name: {v.name}")
    except ImportError:
        print("pyttsx3 not installed.")

def set_config(key, value):
    config = ConfigManager()
    config.set(key, value)
    print(f"Set {key} to {value}")

def main():
    parser = argparse.ArgumentParser(description="Voice Assistant CLI")
    subparsers = parser.add_subparsers(dest="command", help="Available commands")

    # setup
    parser_setup = subparsers.add_parser("setup", help="Run setup wizard")

    # run
    parser_run = subparsers.add_parser("run", help="Start the assistant")

    # config
    parser_config = subparsers.add_parser("config", help="Manage configuration")
    parser_config.add_argument("key", help="Config key")
    parser_config.add_argument("value", help="Config value")

    # voice
    parser_voice = subparsers.add_parser("voice", help="Manage TTS voices")
    parser_voice.add_argument("--list", action="store_true", help="List available voices")
    parser_voice.add_argument("--set", help="Set voice ID")

    # startup
    parser_startup = subparsers.add_parser("startup", help="Manage startup settings")
    parser_startup.add_argument("--enable", action="store_true", help="Enable start on boot")
    parser_startup.add_argument("--disable", action="store_true", help="Disable start on boot")

    args = parser.parse_args()

    if args.command == "setup":
        run_setup()
    elif args.command == "run":
        run_assistant()
    elif args.command == "config":
        set_config(args.key, args.value)
    elif args.command == "voice":
        if args.list:
            list_voices()
        if args.set:
            set_config("voice_id", args.set)
    elif args.command == "startup":
        app_name = "VoiceAssistant"
        script_path = os.path.abspath("src/main.py")
        if args.enable:
            StartupManager.enable_startup(app_name, script_path)
        elif args.disable:
            StartupManager.disable_startup(app_name)
    else:
        parser.print_help()

if __name__ == "__main__":
    main()

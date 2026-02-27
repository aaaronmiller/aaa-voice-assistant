# src/uninstall.py
import os
import shutil
import platform
import sys

def uninstall_app(app_name="VoiceAssistant"):
    print(f"Uninstalling {app_name}...")
    system = platform.system()

    # 1. Remove Startup Entry
    if system == "Windows":
        import winreg
        try:
            key = winreg.OpenKey(winreg.HKEY_CURRENT_USER, r"Software\Microsoft\Windows\CurrentVersion\Run", 0, winreg.KEY_ALL_ACCESS)
            winreg.DeleteValue(key, app_name)
            winreg.CloseKey(key)
            print("Removed from Windows Registry.")
        except FileNotFoundError:
            pass
    elif system == "Linux":
        path = os.path.expanduser(f"~/.config/autostart/{app_name}.desktop")
        if os.path.exists(path):
            os.remove(path)
            print("Removed .desktop file.")
    elif system == "Darwin":
        path = os.path.expanduser(f"~/Library/LaunchAgents/com.{app_name.lower()}.plist")
        if os.path.exists(path):
            os.remove(path)
            print("Removed LaunchAgent plist.")

    # 2. Cleanup Logs/Config?
    # Usually good practice to ask user
    choice = input("Remove configuration and logs? (y/n): ").lower()
    if choice == 'y':
        if os.path.exists("config.json"):
            os.remove("config.json")
        if os.path.exists("assistant.log"):
            os.remove("assistant.log")
        print("Config and logs removed.")

    print("Uninstallation complete. You can now delete the project folder.")

if __name__ == "__main__":
    uninstall_app()

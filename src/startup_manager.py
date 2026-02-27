import os
import sys
import platform
import shutil

class StartupManager:
    @staticmethod
    def enable_startup(app_name, script_path):
        system = platform.system()
        if system == "Windows":
            import winreg
            key_path = r"Software\Microsoft\Windows\CurrentVersion\Run"
            try:
                key = winreg.OpenKey(winreg.HKEY_CURRENT_USER, key_path, 0, winreg.KEY_ALL_ACCESS)
                winreg.SetValueEx(key, app_name, 0, winreg.REG_SZ, f'"{sys.executable}" "{script_path}"')
                winreg.CloseKey(key)
                print("Added to Windows startup.")
            except Exception as e:
                print(f"Failed to add to Windows startup: {e}")

        elif system == "Linux":
            # Create .desktop file in ~/.config/autostart
            autostart_dir = os.path.expanduser("~/.config/autostart")
            os.makedirs(autostart_dir, exist_ok=True)
            desktop_file = os.path.join(autostart_dir, f"{app_name}.desktop")
            content = f"""[Desktop Entry]
Type=Application
Exec={sys.executable} {script_path}
Hidden=false
NoDisplay=false
X-GNOME-Autostart-enabled=true
Name={app_name}
Comment=Voice Assistant
"""
            try:
                with open(desktop_file, "w") as f:
                    f.write(content)
                print("Added to Linux startup.")
            except Exception as e:
                print(f"Failed to add to Linux startup: {e}")

        elif system == "Darwin":
            # Create LaunchAgent plist
            launch_agents = os.path.expanduser("~/Library/LaunchAgents")
            os.makedirs(launch_agents, exist_ok=True)
            plist_path = os.path.join(launch_agents, f"com.{app_name.lower()}.plist")
            content = f"""<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.{app_name.lower()}</string>
    <key>ProgramArguments</key>
    <array>
        <string>{sys.executable}</string>
        <string>{script_path}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
</dict>
</plist>
"""
            try:
                with open(plist_path, "w") as f:
                    f.write(content)
                print("Added to macOS startup.")
            except Exception as e:
                print(f"Failed to add to macOS startup: {e}")

    @staticmethod
    def disable_startup(app_name):
        system = platform.system()
        if system == "Windows":
            import winreg
            key_path = r"Software\Microsoft\Windows\CurrentVersion\Run"
            try:
                key = winreg.OpenKey(winreg.HKEY_CURRENT_USER, key_path, 0, winreg.KEY_ALL_ACCESS)
                winreg.DeleteValue(key, app_name)
                winreg.CloseKey(key)
                print("Removed from Windows startup.")
            except Exception as e:
                print(f"Failed to remove from Windows startup: {e}")
        elif system == "Linux":
            desktop_file = os.path.expanduser(f"~/.config/autostart/{app_name}.desktop")
            if os.path.exists(desktop_file):
                os.remove(desktop_file)
                print("Removed from Linux startup.")
        elif system == "Darwin":
            plist_path = os.path.expanduser(f"~/Library/LaunchAgents/com.{app_name.lower()}.plist")
            if os.path.exists(plist_path):
                os.remove(plist_path)
                print("Removed from macOS startup.")

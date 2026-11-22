# src/overlay.py
import tkinter as tk
from threading import Thread
import time
import logging

import os

def play_beep():
    """Play a simple beep sound for feedback."""
    system = os.name
    if system == 'posix':
        # Linux/Mac: try system beep or bell
        try:
            # Terminal bell
            print('\a')
            # Also try paplay/beep if available
            if os.system('which paplay > /dev/null 2>&1') == 0:
                os.system('paplay /usr/share/sounds/freedesktop/stereo/bell.oga 2>/dev/null &')
            elif os.system('which beep > /dev/null 2>&1') == 0:
                os.system('beep -f 1000 -l 50 2>/dev/null &')
        except:
            pass
    elif system == 'nt':
        try:
            import winsound
            winsound.Beep(1000, 50)
        except:
            pass

class OverlayWindow:
    def __init__(self, font_size=14):
        self.root = None
        self.label = None
        self.thread = None
        self.active = False
        self.text_queue = []
        self.font_size = font_size

    def _create_window(self):
        self.root = tk.Tk()
        self.root.title("AAA Voice Assistant Overlay") # Accessible title
        self.root.overrideredirect(True) # Frameless
        self.root.attributes("-topmost", True)
        self.root.attributes("-alpha", 0.7) # Slightly transparent


        try:
            screen_width = self.root.winfo_screenwidth()
            screen_height = self.root.winfo_screenheight()

            width = 400
            height = 60
            x = (screen_width - width) // 2
            y = screen_height - 120 # Just above taskbar typically

            self.root.geometry(f"{width}x{height}+{x}+{y}")
            self.root.configure(bg='#222')

            self.label = tk.Label(self.root, text="Waiting...", fg="#eee", bg="#222", font=("Segoe UI", 14))
            self.label.pack(expand=True, fill='both', padx=10, pady=5)

            self.root.after(100, self._check_updates)
            self.root.mainloop()
        except Exception as e:
            logging.error(f"Failed to create overlay window: {e}")
            self.active = False
            self.root = None

    def _check_updates(self):
        if self.text_queue and self.root and self.label:
            try:
                text, color = self.text_queue.pop(0)
                self.label.config(text=text, fg=color)
            except Exception as e:
                logging.error(f"Overlay update failed: {e}")

        if self.root:
            self.root.after(100, self._check_updates)

    def start(self):
        if not self.active:
            self.active = True
            self.thread = Thread(target=self._create_window, daemon=True)
            self.thread.start()

    def update_status(self, text, color="#eee"):
        self.text_queue.append((text, color))

    def stop(self):
        if self.root:
            try:
                self.root.quit() # This might not be thread safe depending on tk ver, but quit() signals mainloop to stop
            except Exception as e:
                logging.error(f"Error stopping overlay: {e}")
            self.active = False

# src/overlay.py
import tkinter as tk
from threading import Thread
import time
import platform
import subprocess
import os

def play_beep():
    """Portable beep sound."""
    system = platform.system()
    try:
        if system == "Darwin":
             # macOS system beep
             subprocess.run(["afplay", "/System/Library/Sounds/Tink.aiff"], stderr=subprocess.DEVNULL)
        elif system == "Linux":
             # beep command or echo -e '\a' or play a sound
             # Using paplay for PulseAudio if available is better than \a
             # But keeping it simple for now, maybe use tts engine to beep?
             # Fallback to standard bell
             print('\a', flush=True)
        elif system == "Windows":
             import winsound
             winsound.MessageBeep()
    except Exception:
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
        self.root.title("Voice Assistant Overlay") # Accessibility: Title for screen readers
        self.root.overrideredirect(True) # Frameless
        self.root.attributes("-topmost", True)
        self.root.attributes("-alpha", 0.7) # Slightly transparent

        screen_width = self.root.winfo_screenwidth()
        screen_height = self.root.winfo_screenheight()

        width = 500 # Slightly wider
        height = 80 # Slightly taller for larger font
        x = (screen_width - width) // 2
        y = screen_height - 140

        self.root.geometry(f"{width}x{height}+{x}+{y}")
        self.root.configure(bg='#222')

        self.label = tk.Label(self.root, text="Waiting...", fg="#eee", bg="#222", font=("Segoe UI", self.font_size))
        self.label.pack(expand=True, fill='both', padx=10, pady=5)

        self.root.after(100, self._check_updates)
        self.root.mainloop()

    def _check_updates(self):
        if self.text_queue:
            text, color = self.text_queue.pop(0)
            self.label.config(text=text, fg=color)
            # Accessibility: Update title so screen reader knows state changed
            if self.root:
                self.root.title(f"Voice Assistant: {text}")

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
            self.root.quit()
            self.active = False

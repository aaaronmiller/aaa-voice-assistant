"""
Breakboard - Physical interrupt button handler.
Monitors a designated key (default F12) to interrupt assistant operations.
"""

import threading
import time

# Optional keyboard module
try:
    import keyboard

    KEYBOARD_AVAILABLE = True
except ImportError:
    KEYBOARD_AVAILABLE = False
    keyboard = None


def is_breakboard_available():
    """Check if breakboard can be used."""
    return KEYBOARD_AVAILABLE


class BreakboardListener:
    def __init__(self, interrupt_key="f12"):
        self.interrupt_key = interrupt_key
        self.running = False
        self.thread = None
        self.callback = None
        self.pressed = False

    def set_callback(self, callback):
        """Set a function to call when interrupt is triggered."""
        self.callback = callback

    def start(self):
        """Start the breakboard listener thread."""
        if not KEYBOARD_AVAILABLE:
            raise RuntimeError("keyboard module not available")

        self.running = True
        self.thread = threading.Thread(target=self._listen_loop, daemon=True)
        self.thread.start()

    def stop(self):
        """Stop the listener."""
        self.running = False
        if self.thread:
            self.thread.join(timeout=2)

    def _listen_loop(self):
        """Main loop polling the interrupt key."""
        while self.running:
            try:
                # Check if key is pressed
                if keyboard.is_pressed(self.interrupt_key):
                    if not self.pressed:
                        self.pressed = True
                        if self.callback:
                            self.callback()
                        # Debounce: wait for release
                        while keyboard.is_pressed(self.interrupt_key) and self.running:
                            time.sleep(0.01)
                        self.pressed = False
                time.sleep(0.01)
            except Exception as e:
                print(f"Breakboard error: {e}")
                time.sleep(0.1)

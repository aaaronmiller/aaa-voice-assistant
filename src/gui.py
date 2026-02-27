import pystray
from PIL import Image, ImageDraw
import threading
import sys
import time

def create_image(width, height, color1, color2):
    image = Image.new('RGB', (width, height), color1)
    dc = ImageDraw.Draw(image)
    dc.rectangle(
        (width // 2, 0, width, height // 2),
        fill=color2)
    dc.rectangle(
        (0, height // 2, width // 2, height),
        fill=color2)
    return image

class TrayIcon:
    def __init__(self, assistant):
        self.assistant = assistant
        self.icon = None

    def setup(self):
        # Create a simple icon
        image = create_image(64, 64, 'black', 'white')

        menu = pystray.Menu(
            pystray.MenuItem('Toggle Wake Word', self.on_toggle_wake_word, checked=lambda item: getattr(self.assistant, 'wake_word_enabled', True)),
            pystray.MenuItem('Quit', self.on_quit)
        )

        self.icon = pystray.Icon("AAA Voice Assistant", image, "AAA Voice Assistant", menu)

    def on_toggle_wake_word(self, icon, item):
        # Toggle the attribute on assistant
        current = getattr(self.assistant, 'wake_word_enabled', True)
        setattr(self.assistant, 'wake_word_enabled', not current)

        # If disabled, we might want to stop listening if it was triggered by wake word
        # But let's keep it simple.

        # Update menu check state (pystray handles this via the lambda)

    def on_quit(self, icon, item):
        self.assistant.stop()
        icon.stop()
        # sys.exit(0) # pystray.stop() returns control to main thread

    def run(self):
        self.setup()
        self.icon.run()

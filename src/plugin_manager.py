# src/plugin_manager.py
import importlib
import os
import glob
import sys

class PluginManager:
    def __init__(self, plugin_dir="plugins"):
        self.plugin_dir = plugin_dir
        self.loaded_plugins = {}

        if not os.path.exists(plugin_dir):
            os.makedirs(plugin_dir)
            with open(os.path.join(plugin_dir, "__init__.py"), "w") as f:
                pass

    def load_plugins(self):
        sys.path.append(os.path.abspath(self.plugin_dir))

        # Load all .py files in plugins/
        files = glob.glob(os.path.join(self.plugin_dir, "*.py"))
        for f in files:
            name = os.path.basename(f)[:-3] # strip .py
            if name == "__init__":
                continue

            try:
                module = importlib.import_module(f"plugins.{name}")
                if hasattr(module, "setup"):
                    module.setup()
                self.loaded_plugins[name] = module
                print(f"Loaded plugin: {name}")
            except Exception as e:
                print(f"Failed to load plugin {name}: {e}")

    def get_stt_providers(self):
        providers = []
        for p in self.loaded_plugins.values():
            if hasattr(p, "get_stt_provider"):
                providers.append(p.get_stt_provider())
        return providers

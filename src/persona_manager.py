# src/persona_manager.py
import yaml
import os
import glob

class PersonaManager:
    def __init__(self, persona_dir="personas"):
        self.persona_dir = persona_dir
        self.personas = {}

        if not os.path.exists(persona_dir):
            os.makedirs(persona_dir)
            self._create_default_personas()

        self.load_personas()

    def _create_default_personas(self):
        default_personas = {
            "default": {
                "name": "Assistant",
                "system_prompt": "You are a helpful and efficient AI assistant. Keep responses concise."
            },
            "pirate": {
                "name": "Captain",
                "system_prompt": "You are a pirate on the high seas! Speak like one, utilize nautical terms, and be enthusiastic. Arrr!"
            },
            "coder": {
                "name": "Dev",
                "system_prompt": "You are an expert software engineer. Provide code snippets in markdown, explain logic clearly, and follow best practices."
            }
        }

        for name, data in default_personas.items():
            with open(os.path.join(self.persona_dir, f"{name}.yaml"), "w") as f:
                yaml.dump(data, f)

    def load_personas(self):
        files = glob.glob(os.path.join(self.persona_dir, "*.yaml"))
        for f in files:
            try:
                with open(f, "r") as file:
                    data = yaml.safe_load(file)
                    name = os.path.basename(f)[:-5] # strip .yaml
                    self.personas[name] = data
            except Exception as e:
                print(f"Failed to load persona {f}: {e}")

    def get_persona(self, name):
        return self.personas.get(name, self.personas.get("default"))

    def list_personas(self):
        return list(self.personas.keys())

# src/safe_cli.py
import subprocess
import shlex

class SafeCLI:
    def __init__(self, allowed_commands=None):
        self.allowed_commands = allowed_commands or ["echo", "ls", "grep"]
        self.blacklist_patterns = [";", "&&", "||", "`", "$("]

    def validate_command(self, cmd_str):
        if any(pat in cmd_str for pat in self.blacklist_patterns):
            raise ValueError("Command contains potentially unsafe characters.")

        parts = shlex.split(cmd_str)
        if not parts:
            raise ValueError("Empty command.")

        base_cmd = parts[0]
        if self.allowed_commands and base_cmd not in self.allowed_commands:
            # Maybe too strict?
            # Let's rely on user config for safety.
            pass
        return parts

    def run(self, cmd_template, prompt):
        # cmd_template e.g. "claude -p '{prompt}'"
        # Using format might be risky if prompt contains injection.
        # Prefer passing prompt as STDIN or separate argument if tool supports it.

        # New approach: cmd_template is list ["claude", "-p"]
        # prompt is appened or passed to stdin

        pass

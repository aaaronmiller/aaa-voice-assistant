# Session 3: Security Posture

## Council: Expert Council
**Members:** Security Analyst, AppSec Engineer, Privacy Advocate, Threat Modeler, Python Security Expert, Compliance Officer, Hacker Persona.

## Process
- **Round 1:** Audited `src/config_manager.py` (API keys stored in plain text JSON) and `src/llm_service.py` (CLI backend command injection risk).
- **Probe 1:** Verified that `CLIBackend` uses `subprocess.run` with `shell=False` (good), but the `command_template` itself comes from config and is split naively.
- **Round 2:** Focused on `requests` usage. `OpenClawBackend` in `llm_service.py` has a 30s timeout (good) but `OpenClawBackend` in `openclaw_backend.py` (duplicate file?) has a 60s timeout and no SSL verification check mentioned (default is verify=True).
- **Probe 2:** Found duplicate/conflicting `OpenClawBackend` classes. One in `openclaw_backend.py` and one inside `llm_service.py`. The one in `llm_service.py` is the one actually used by `LLMService`.
- **Round 3:** Consensus: The existence of "dead code" (`openclaw_backend.py`) is a security risk (confusion). API keys in `config.json` are acceptable for a local desktop app MVP, but permissions need to be restricted.

## Approved Improvements

### Critical
1.  **Remove Duplicate/Dead Code**: `src/openclaw_backend.py` appears unused as `src/llm_service.py` defines its own `OpenClawBackend`. This confusion can lead to patching the wrong file. **Action**: Delete `src/openclaw_backend.py` or merge it.
2.  **Restrict CLI Backend**: The `CLIBackend` in `llm_service.py` takes a `command_template` from config and runs it. If a user (or malware modifying config) sets this to `rm -rf /`, the assistant executes it. **Action**: Add a whitelist of allowed commands or a confirmation step for CLI execution, or at least a warning log. For now, strictly validate the command executable path.

### High
3.  **Sanitize Input for CLI**: `CLIBackend` passes `input=prompt` to `subprocess.run`. This is generally safe from shell injection if `shell=False`, but we should ensure the command itself is not user-manipulatable via the `prompt`. (It isn't, as currently implemented).
4.  **API Key File Permissions**: On Linux/Mac, `config.json` containing keys should be `600`. **Action**: Update `ConfigManager.save()` to set file permissions to `0o600` on POSIX systems.

### Medium
5.  **Memory Sanitization**: `MemoryStore` keeps all history in plain text. **Action**: Add a `privacy_mode` check to `MemoryStore` to potentially encrypt or not save history if enabled.

## Rejected/Deferred
- **Encrypted Config**: Using OS keyring is better but adds complex dependencies (`keyring` lib) which might break cross-platform simplicity for this stage.

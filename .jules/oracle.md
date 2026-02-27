## 2025-05-15 - [Configuration Source of Truth]
**Discovery:** `src/main.py` re-implemented configuration loading logic manually instead of using the existing `src/config_manager.py`.
**Impact:** Critical platform-specific settings (like Windows path fixes in `ConfigManager`) were being ignored when running via `main.py`, potentially breaking the app on Windows.
**Action:** Always search for existing managers/handlers before implementing logic in entry points. Single Source of Truth must be enforced.

## 2025-05-15 - [Import-Time Side Effects]
**Discovery:** `src/assistant_logic.py` performs top-level imports of hardware-dependent modules (`keyboard`, `audio_recorder`), making it impossible to test the class without having those libraries installed and functioning.
**Impact:** CI pipelines fail because they lack audio drivers or X11/display servers, preventing even logic tests from running.
**Action:** Use dependency injection or lazy imports for hardware interfaces. Alternatively, use `sys.modules` mocking in tests *before* importing the subject module.

## 2025-05-15 - [Duplicate Logic/Dead Code]
**Discovery:** `src/openclaw_backend.py` defines an `OpenClawBackend` class, but `src/llm_service.py` defines a *different* class with the same name and uses that one.
**Impact:** Developers might patch the wrong file, believing they are fixing a bug, while the actual running code remains unchanged. Security patches could be missed.
**Action:** Aggressively prune unused files. If a file is not imported by `main` or its dependencies, it should be deleted or integrated.

## 2025-05-15 - [Orphaned Infrastructure]
**Discovery:** `src/logger.py` existed but was not imported by any other file. The entire application was using `print()` for debugging.
**Impact:** Users and devs had no persistent logs to debug crashes.
**Action:** Search for infrastructure files (logger, utils) that are not referenced in the codebase and either wire them up or delete them.

## 2025-05-15 - [Feedback Modality]
**Discovery:** The application relied on `print('\a')` for audio feedback, which is unreliable and inaccessible to users who can't hear the system bell or have it disabled.
**Impact:** Blind/Low-vision users have no reliable way to know if the system is listening without TTS announcements (which block recording).
**Action:** Implement a dedicated `SoundManager` for non-verbal auditory cues (beeps, chimes) to indicate state changes.

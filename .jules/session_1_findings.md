# Session 1: Architecture & Code Quality

## Council: Expert Council
**Members:** Architect, Maintainability Expert, Python Specialist, Refactoring Lead, Domain Expert, Error Handling Specialist, Skeptic.

## Process
- **Round 1:** Identification of `Assistant` as a God Object and `main.py` duplicating configuration logic.
- **Probe 1:** Confirmed "God Object" anti-pattern risks (scalability, maintenance).
- **Round 2:** Focused on the `config_manager.py` vs `main.py` discrepancy. `main.py` misses OS-specific path adjustments present in `ConfigManager`.
- **Probe 2:** Verified code. `ConfigManager._detect_system_settings` fixes Windows paths. `main.py` does not.
- **Round 3:** Consensus on immediate fixes vs long-term refactoring.

## Approved Improvements

### Critical
1.  **Unify Configuration Loading**: `src/main.py` implements its own `load_config` function, bypassing `src/config_manager.py`. This leads to inconsistency, specifically missing `_detect_system_settings` logic for Windows paths. **Action**: Refactor `main.py` to use `ConfigManager`.

### High
2.  **Fix Import Hack**: `src/assistant_logic.py` uses a `try-except ImportError` block to handle relative vs absolute imports. This indicates improper package execution. **Action**: Standardize imports to absolute (`from src.x import y`) and ensure entry point sets `PYTHONPATH` or runs as module.

### Medium
3.  **Standardize Logging**: The codebase relies heavily on `print()` for status and debugging. **Action**: Integrate `src.logger` across `assistant_logic.py` and `main.py`.
4.  **Extract VAD Logic**: The `_wake_word_loop` in `Assistant` mixes flow control with signal processing (RMS calculation). **Action**: Extract silence detection logic into a dedicated method `_check_silence` or move to `AudioRecorder`.

## Rejected/Deferred
- **Full God Object Decomposition**: Breaking `Assistant` into `UIManager`, `AudioManager`, `InferenceEngine` is too large for this cycle. Deferred to future architectural overhaul.

# Session 2: Testing Strategy & Coverage

## Council: Expert Council
**Members:** QA Lead, Test Architect, CI Specialist, Python Test Guru, Mocking Expert, Coverage Analyst, Realist.

## Process
- **Round 1:** Initial scan of `tests/` showed only 3 tests. `test_integration.py` failed due to missing dependencies (`numpy`, `keyboard`).
- **Probe 1:** Verified environment setup. `pip install` fixed immediate errors, but the dependency chain for tests is brittle (requires system-level audio libs if not fully mocked).
- **Round 2:** `test_integration.py` uses `unittest.mock` but still imports modules that import hardware libs (`keyboard`, `pyaudio` via `AudioRecorder`).
- **Probe 2:** Confirmed that `src/assistant_logic.py` imports `AudioRecorder` at module level. Even if we mock `AudioRecorder` in the test, the *import* of `assistant_logic` triggers the *import* of `audio_recorder`, which triggers `import pyaudio`.
- **Round 3:** Consensus: We must insulate the logic from hardware dependencies for CI/testing.

## Approved Improvements

### Critical
1.  **Robust Mocking Strategy**: The current integration tests fail in environments without audio hardware/libs because the `import` statements in `assistant_logic.py` are not guarded or the mocks are applied too late. **Action**: Refactor imports in `assistant_logic.py` to allow dependency injection or use `sys.modules` patching in tests *before* importing the module under test.

### High
2.  **Add Unit Tests for Core Logic**: There are zero unit tests for `Assistant._process_audio_buffer` logic in isolation (checking VAD logic, etc). **Action**: Create `tests/test_assistant_logic.py` focusing on state transitions without real threading/audio.
3.  **Fix Deprecation Warnings**: `datetime.datetime.utcnow()` is deprecated. **Action**: Replace with `datetime.datetime.now(datetime.timezone.utc)`.

### Medium
4.  **CI-Friendly Requirements**: Create a `requirements-test.txt` that installs `numpy`, `mock`, etc., but perhaps avoids `pyaudio` if we can mock it out properly.

## Rejected/Deferred
- **End-to-End Audio Testing**: Simulating real audio streams is too complex for this phase.

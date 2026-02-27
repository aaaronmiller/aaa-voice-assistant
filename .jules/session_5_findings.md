# Session 5: UX & Accessibility

## Council: Expert Council
**Members:** UX Designer, Accessibility Champion, Interaction Designer, Voice UI Expert, Frontend Dev, User Advocate, Critic.

## Process
- **Round 1:** Review `src/overlay.py`. It creates a frameless Tkinter window with hardcoded colors and fonts. No ARIA support (not possible in Tkinter easily, but window title helps).
- **Probe 1:** Verified that the overlay is purely visual. Screen reader users get ZERO feedback about state changes (Listening, Processing) unless they have TTS enabled. But `SystemTTS` is blocking, so we can't announce "Listening" via TTS easily without delaying recording.
- **Round 2:** Discussed audio cues. `print('\a')` is used for "beep". This is primitive and might not work on all terminals/OSs.
- **Probe 2:** `winsound.Beep` or playing a small wav file is better.
- **Round 3:** Consensus: Improve visual contrast and add distinct audio cues for state transitions.

## Approved Improvements

### Critical
1.  **Audio Cues**: Replace `print('\a')` with a robust cross-platform beep or sound player (using `pyaudio` or `subprocess` to play a wav). This is vital for eyes-free usage. **Action**: Add a `SoundManager` or add methods to `TTSProvider` to play system sounds.

### High
2.  **Visual Accessibility**: The overlay uses `#eee` on `#222`. Contrast is okay, but font size is fixed. **Action**: Make font size configurable or larger by default (e.g., 24pt).
3.  **State Announcement**: For screen readers, we should update the window title to reflect status, not just the label text. Tools like NVDA can be configured to read title changes. **Action**: `self.root.title(text)` in `overlay.py`.

### Medium
4.  **Error Feedback**: Currently errors are printed to console. **Action**: Display errors in the overlay (in Red) for at least 3 seconds so the user knows why nothing happened.

## Rejected/Deferred
- **GUI Settings Panel**: Too much work for this sprint. CLI is fine for now.

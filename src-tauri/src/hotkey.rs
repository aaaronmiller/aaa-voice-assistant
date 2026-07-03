


use global_hotkey::{code, GlobalHotKeyManager, HotKey, Modifiers};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HotkeyError {
    #[error("Failed to register hotkey: {0}")]
    RegistrationError(String),
    #[error("Hotkey conflict detected: {0}")]
    ConflictError(String),
    #[error("Hotkey not found: {0}")]
    NotFoundError(String),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum HotkeyId {
    PTT,
    PasteLastTranscription,
    ToggleWakeWord,
    ToggleCleanup,
    OpenHistory,
    OpenSettings,
    CancelRecording,
    SwitchProfile,
}

impl HotkeyId {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "ptt" => Some(Self::PTT),
            "paste_last_transcription" => Some(Self::PasteLastTranscription),
            "toggle_wake_word" => Some(Self::ToggleWakeWord),
            "toggle_cleanup" => Some(Self::ToggleCleanup),
            "open_history" => Some(Self::OpenHistory),
            "open_settings" => Some(Self::OpenSettings),
            "cancel_recording" => Some(Self::CancelRecording),
            "switch_profile" => Some(Self::SwitchProfile),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::PTT => "ptt",
            Self::PasteLastTranscription => "paste_last_transcription",
            Self::ToggleWakeWord => "toggle_wake_word",
            Self::ToggleCleanup => "toggle_cleanup",
            Self::OpenHistory => "open_history",
            Self::OpenSettings => "open_settings",
            Self::CancelRecording => "cancel_recording",
            Self::SwitchProfile => "switch_profile",
        }
    }
}

pub struct HotkeyManager {
    manager: GlobalHotKeyManager,
    registered_hotkeys: HashMap<HotkeyId, HotKey>,
    hotkey_strings: HashMap<HotkeyId, String>,
}

impl HotkeyManager {
    pub fn new() -> Result<Self, HotkeyError> {
        Ok(Self {
            manager: GlobalHotKeyManager::new()
                .map_err(|e| HotkeyError::RegistrationError(e.to_string()))?,
            registered_hotkeys: HashMap::new(),
            hotkey_strings: HashMap::new(),
        })
    }

    /// Parse a hotkey string like "Ctrl+Shift+Space" or "CommandOrControl+Shift+A"
    /// into Modifiers and a key code
    fn parse_hotkey(hotkey_str: &str) -> Result<(Modifiers, u32), HotkeyError> {
        let parts: Vec<&str> = hotkey_str.split('+').collect();
        if parts.is_empty() {
            return Err(HotkeyError::RegistrationError(
                "Invalid hotkey string".to_string(),
            ));
        }

        let mut modifiers = Modifiers::empty();
        let mut key_code: Option<u32> = None;

        for part in parts {
            let part_lower = part.trim().to_lowercase();
            match part_lower.as_str() {
                "ctrl" | "control" => modifiers |= Modifiers::CONTROL,
                "alt" => modifiers |= Modifiers::ALT,
                "shift" => modifiers |= Modifiers::SHIFT,
                "meta" | "cmd" | "command" | "super" | "win" | "windows" => {
                    modifiers |= Modifiers::META
                }
                // Single letters and special keys
                _ if part_lower.len() == 1 => {
                    let c = part_lower.chars().next().unwrap();
                    key_code = Some(code(c));
                }
                "space" => key_code = Some(code::Code::Space.into()),
                "enter" | "return" => key_code = Some(code::Code::Enter.into()),
                "escape" | "esc" => key_code = Some(code::Code::Escape.into()),
                "tab" => key_code = Some(code::Code::Tab.into()),
                "backspace" => key_code = Some(code::Code::Backspace.into()),
                "delete" => key_code = Some(code::Code::Delete.into()),
                "up" | "arrowup" => key_code = Some(code::Code::ArrowUp.into()),
                "down" | "arrowdown" => key_code = Some(code::Code::ArrowDown.into()),
                "left" | "arrowleft" => key_code = Some(code::Code::ArrowLeft.into()),
                "right" | "arrowright" => key_code = Some(code::Code::ArrowRight.into()),
                "home" => key_code = Some(code::Code::Home.into()),
                "end" => key_code = Some(code::Code::End.into()),
                "pageup" => key_code = Some(code::Code::PageUp.into()),
                "pagedown" => key_code = Some(code::Code::PageDown.into()),
                "f1" => key_code = Some(code::Code::F1.into()),
                "f2" => key_code = Some(code::Code::F2.into()),
                "f3" => key_code = Some(code::Code::F3.into()),
                "f4" => key_code = Some(code::Code::F4.into()),
                "f5" => key_code = Some(code::Code::F5.into()),
                "f6" => key_code = Some(code::Code::F6.into()),
                "f7" => key_code = Some(code::Code::F7.into()),
                "f8" => key_code = Some(code::Code::F8.into()),
                "f9" => key_code = Some(code::Code::F9.into()),
                "f10" => key_code = Some(code::Code::F10.into()),
                "f11" => key_code = Some(code::Code::F11.into()),
                "f12" => key_code = Some(code::Code::F12.into()),
                _ => {
                    return Err(HotkeyError::RegistrationError(format!(
                        "Unknown key in hotkey string: {}",
                        part
                    )));
                }
            }
        }

        let kc = key_code.ok_or_else(|| {
            HotkeyError::RegistrationError("No key specified in hotkey string".to_string())
        })?;

        Ok((modifiers, kc))
    }

    /// Check if a hotkey string conflicts with already registered hotkeys (different IDs) or
    /// with system reserved hotkeys. This does not modify the manager's state.
    pub fn check_conflict(&self, id: HotkeyId, hotkey_str: &str) -> Result<(), HotkeyError> {
        let (modifiers, key_code) = Self::parse_hotkey(hotkey_str)?;
        let hotkey = HotKey::new(Some(modifiers), key_code);

        // Check against already registered hotkeys for different IDs
        for (other_id, other_hk) in &self.registered_hotkeys {
            if *other_id != id && *other_hk == hotkey {
                return Err(HotkeyError::ConflictError(format!(
                    "Hotkey conflicts with already registered {:?}",
                    other_id
                )));
            }
        }

        // Check against a small set of known system hotkeys (best effort)
        if Self::is_reserved_system_hotkey(modifiers, key_code) {
            return Err(HotkeyError::ConflictError(
                "Hotkey is reserved by the system".to_string()
            ));
        }

        Ok(())
    }

    pub fn register(&mut self, id: HotkeyId, hotkey_str: &str) -> Result<(), HotkeyError> {
        // Parse hotkey first
        let (modifiers, key_code) = Self::parse_hotkey(hotkey_str)?;
        let hotkey = HotKey::new(Some(modifiers), key_code);

        // Check for conflicts with other IDs
        for (other_id, other_hk) in &self.registered_hotkeys {
            if *other_id != id && *other_hk == hotkey {
                return Err(HotkeyError::ConflictError(format!(
                    "Hotkey conflicts with already registered {:?}",
                    other_id
                )));
            }
        }

        // Check against system reserved hotkeys
        if Self::is_reserved_system_hotkey(modifiers, key_code) {
            return Err(HotkeyError::ConflictError(
                "Hotkey is reserved by the system".to_string()
            ));
        }

        // Unregister if already registered for this ID
        self.unregister(id)?;

        // Attempt to register with global manager
        self.manager
            .register(hotkey)
            .map_err(|e| {
                if e.to_string().to_lowercase().contains("already")
                    || e.to_string().to_lowercase().contains("registered")
                {
                    HotkeyError::ConflictError(format!(
                        "Hotkey conflicts with system or other application: {}",
                        e
                    ))
                } else {
                    HotkeyError::RegistrationError(e.to_string())
                }
            })?;

        self.registered_hotkeys.insert(id, hotkey);
        self.hotkey_strings.insert(id, hotkey_str.to_string());

        log::info!("Registered hotkey {:?} as {}", id, hotkey_str);
        Ok(())
    }

    /// Determine if the given hotkey is a known reserved system hotkey.
    /// This is a best-effort check for common system shortcuts.
    fn is_reserved_system_hotkey(modifiers: global_hotkey::Modifiers, key_code: u32) -> bool {
        use global_hotkey::code::Code;
        // Alt+F4 (close window)
        if modifiers == global_hotkey::Modifiers::ALT && key_code == Code::F4.into() {
            return true;
        }
        // Win+E (File Explorer)
        if modifiers == global_hotkey::Modifiers::META && key_code == Code::E.into() {
            return true;
        }
        // Win+R (Run dialog)
        if modifiers == global_hotkey::Modifiers::META && key_code == Code::R.into() {
            return true;
        }
        // Ctrl+Shift+Esc (Task Manager)
        if modifiers == (global_hotkey::Modifiers::CONTROL | global_hotkey::Modifiers::SHIFT)
            && key_code == Code::Escape.into()
        {
            return true;
        }
        // Ctrl+Alt+Del is a secure attention sequence (cannot be registered). Not typically catchable.
        // The underlying crate will reject it anyway.
        false
    }

    pub fn unregister(&mut self, id: HotkeyId) -> Result<(), HotkeyError> {
        if let Some(hotkey) = self.registered_hotkeys.remove(&id) {
            self.manager
                .unregister(hotkey)
                .map_err(|e| HotkeyError::RegistrationError(e.to_string()))?;
            self.hotkey_strings.remove(&id);
        }
        Ok(())
    }

    pub fn start_listening(&self) {
        // Handled by Tauri plugin events
    }

    pub fn list_registered(&self) -> Vec<(HotkeyId, String)> {
        self.hotkey_strings
            .iter()
            .map(|(id, s)| (*id, s.clone()))
            .collect()
    }
}

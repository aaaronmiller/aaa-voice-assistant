use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HotkeyError {
    #[error("Failed to register hotkey: {0}")]
    RegistrationError(#[from] global_hotkey::Error),
    #[error("Hotkey conflict detected: {0}")]
    ConflictError(String),
    #[error("Hotkey not found: {0}")]
    NotFoundError(String),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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

#[derive(Debug, Clone)]
pub struct HotkeyConfig {
    pub ptt: String,
    pub paste_last: String,
    pub toggle_wake_word: String,
    pub toggle_cleanup: String,
    pub open_history: String,
    pub open_settings: String,
    pub cancel_recording: String,
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        Self {
            ptt: "F1".to_string(),
            paste_last: "Ctrl+Shift+V".to_string(),
            toggle_wake_word: "Ctrl+Shift+W".to_string(),
            toggle_cleanup: "Ctrl+Shift+C".to_string(),
            open_history: "Ctrl+Shift+H".to_string(),
            open_settings: "Ctrl+Shift+S".to_string(),
            cancel_recording: "Esc".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct HotkeyManager {
    manager: GlobalHotKeyManager,
    registered_hotkeys: HashMap<HotkeyId, HotKey>,
    hotkey_map: HashMap<HotKey, HotkeyId>,
    callbacks: Arc<Mutex<HashMap<HotkeyId, Box<dyn Fn() + Send>>>>,
}

impl HotkeyManager {
    pub fn new() -> Result<Self, HotkeyError> {
        Ok(Self {
            manager: GlobalHotKeyManager::new()?,
            registered_hotkeys: HashMap::new(),
            hotkey_map: HashMap::new(),
            callbacks: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub fn register(
        &mut self,
        id: HotkeyId,
        hotkey_str: &str,
        callback: impl Fn() + Send + 'static,
    ) -> Result<(), HotkeyError> {
        let hotkey = Self::parse_hotkey(hotkey_str)?;
        
        // Check if hotkey is already registered
        if self.hotkey_map.contains_key(&hotkey) {
            return Err(HotkeyError::ConflictError(format!(
                "Hotkey {} already registered",
                hotkey_str
            )));
        }

        // Unregister existing hotkey for this id if it exists
        if let Some(existing_hotkey) = self.registered_hotkeys.remove(&id) {
            self.manager.unregister(existing_hotkey);
            self.hotkey_map.remove(&existing_hotkey);
        }

        self.manager.register(hotkey)?;
        self.registered_hotkeys.insert(id, hotkey);
        self.hotkey_map.insert(hotkey, id);
        self.callbacks.lock().unwrap().insert(id, Box::new(callback));

        Ok(())
    }

    pub fn unregister(&mut self, id: HotkeyId) -> Result<(), HotkeyError> {
        if let Some(hotkey) = self.registered_hotkeys.remove(&id) {
            self.manager.unregister(hotkey);
            self.hotkey_map.remove(&hotkey);
            self.callbacks.lock().unwrap().remove(&id);
        }

        Ok(())
    }

    pub fn start_listening(&self) {
        let callbacks = Arc::clone(&self.callbacks);
        
        let _ = std::thread::spawn(move || {
            for event in GlobalHotKeyEvent::receiver() {
                if let Some(id) = callbacks
                    .lock()
                    .unwrap()
                    .keys()
                    .find(|&&key| callbacks.lock().unwrap().contains_key(&key))
                {
                    let id = *id;
                    if let Some(callback) = callbacks.lock().unwrap().get(&id) {
                        callback();
                    }
                }
            }
        });
    }

    pub fn list_registered(&self) -> Vec<(HotkeyId, String)> {
        self.registered_hotkeys
            .iter()
            .map(|(id, hotkey)| (*id, Self::format_hotkey(hotkey)))
            .collect()
    }

    fn parse_hotkey(hotkey_str: &str) -> Result<HotKey, HotkeyError> {
        let mut modifiers = Modifiers::empty();
        let mut code = None;

        for part in hotkey_str.split('+') {
            let part = part.trim().to_lowercase();
            match part.as_str() {
                "ctrl" | "control" => modifiers.insert(Modifiers::CTRL),
                "alt" => modifiers.insert(Modifiers::ALT),
                "shift" => modifiers.insert(Modifiers::SHIFT),
                "super" | "cmd" => modifiers.insert(Modifiers::SUPER),
                _ => {
                    // Parse key code
                    code = Some(Self::parse_key_code(&part));
                }
            };
        }

        let code = code.ok_or_else(|| {
            HotkeyError::RegistrationError(global_hotkey::Error::InvalidHotKey)
        })?;

        Ok(HotKey::new(Some(modifiers), code))
    }

    fn parse_key_code(part: &str) -> Code {
        // TODO: Implement proper key code parsing
        // This is a simplified version
        match part.as_str() {
            "f1" => Code::F1,
            "f2" => Code::F2,
            "f3" => Code::F3,
            "f4" => Code::F4,
            "f5" => Code::F5,
            "f6" => Code::F6,
            "f7" => Code::F7,
            "f8" => Code::F8,
            "f9" => Code::F9,
            "f10" => Code::F10,
            "f11" => Code::F11,
            "f12" => Code::F12,
            "esc" => Code::Escape,
            "tab" => Code::Tab,
            "enter" => Code::Return,
            "space" => Code::Space,
            _ => {
                if part.len() == 1 {
                    let c = part.chars().next().unwrap().to_ascii_lowercase();
                    Code::Key(c)
                } else {
                    Code::Unknown
                }
            }
        }
    }

    fn format_hotkey(hotkey: &HotKey) -> String {
        let mut parts = Vec::new();
        
        if let Some(modifiers) = hotkey.modifiers {
            if modifiers.contains(Modifiers::CTRL) {
                parts.push("Ctrl");
            }
            if modifiers.contains(Modifiers::ALT) {
                parts.push("Alt");
            }
            if modifiers.contains(Modifiers::SHIFT) {
                parts.push("Shift");
            }
            if modifiers.contains(Modifiers::SUPER) {
                parts.push("Super");
            }
        }

        // Format key code
        let key_str = match hotkey.code {
            Code::F1 => "F1",
            Code::F2 => "F2",
            Code::F3 => "F3",
            Code::F4 => "F4",
            Code::F5 => "F5",
            Code::F6 => "F6",
            Code::F7 => "F7",
            Code::F8 => "F8",
            Code::F9 => "F9",
            Code::F10 => "F10",
            Code::F11 => "F11",
            Code::F12 => "F12",
            Code::Escape => "Esc",
            Code::Tab => "Tab",
            Code::Return => "Enter",
            Code::Space => "Space",
            Code::Key(c) => &c.to_string(),
            _ => "Unknown",
        };
        
        parts.push(key_str);
        
        parts.join("+")
    }
}

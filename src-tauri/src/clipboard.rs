use arboard::Clipboard;
use std::cell::RefCell;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClipboardError {
    #[error("Failed to access clipboard: {0}")]
    AccessError(String),
    #[error("Failed to paste text: {0}")]
    PasteError(String),
}

impl From<arboard::Error> for ClipboardError {
    fn from(e: arboard::Error) -> Self {
        ClipboardError::AccessError(e.to_string())
    }
}

#[derive(Debug, Clone)]
pub enum ClipboardContent {
    Text(String),
    Image(Vec<u8>),
    Files(Vec<String>),
}

pub struct ClipboardManager {
    saved_content: Option<ClipboardContent>,
    clipboard: RefCell<Clipboard>,
}

impl ClipboardManager {
    pub fn new() -> Result<Self, ClipboardError> {
        Ok(Self {
            saved_content: None,
            clipboard: RefCell::new(Clipboard::new()?),
        })
    }

    pub fn save(&mut self) -> Result<(), ClipboardError> {
        if let Ok(text) = self.clipboard.borrow_mut().get_text() {
            self.saved_content = Some(ClipboardContent::Text(text));
        } else {
            self.saved_content = None;
        }
        Ok(())
    }

    pub fn restore(&mut self) -> Result<(), ClipboardError> {
        if let Some(content) = &self.saved_content {
            if let ClipboardContent::Text(text) = content {
                self.clipboard.borrow_mut().set_text(text.clone())?;
            }
        }
        Ok(())
    }

    pub fn paste_text(&self, text: &str) -> Result<(), ClipboardError> {
        let current_content = self
            .clipboard
            .borrow_mut()
            .get_text()
            .map(ClipboardContent::Text)
            .ok();

        self.clipboard.borrow_mut().set_text(text.to_string())?;

        let mut enigo = enigo::Enigo::new(&enigo::Settings::default())
            .map_err(|e| ClipboardError::PasteError(e.to_string()))?;

        use enigo::{Key, Keyboard};

        enigo
            .key(Key::Control, enigo::Direction::Press)
            .map_err(|e| ClipboardError::PasteError(e.to_string()))?;
        enigo
            .key(Key::Unicode('v'), enigo::Direction::Click)
            .map_err(|e| ClipboardError::PasteError(e.to_string()))?;
        enigo
            .key(Key::Control, enigo::Direction::Release)
            .map_err(|e| ClipboardError::PasteError(e.to_string()))?;

        if let Some(content) = current_content {
            std::thread::sleep(std::time::Duration::from_millis(500));
            if let ClipboardContent::Text(text) = content {
                self.clipboard.borrow_mut().set_text(text)?;
            }
        }

        Ok(())
    }

    pub fn get_current(&self) -> Result<ClipboardContent, ClipboardError> {
        self.clipboard
            .borrow_mut()
            .get_text()
            .map(ClipboardContent::Text)
            .map_err(|e| ClipboardError::AccessError(e.to_string()))
    }
}

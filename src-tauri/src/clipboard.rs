use arboard::Clipboard;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClipboardError {
    #[error("Failed to access clipboard: {0}")]
    AccessError(#[from] arboard::Error),
    #[error("Failed to paste text: {0}")]
    PasteError(#[from] enigo::Error),
}

#[derive(Debug, Clone)]
pub enum ClipboardContent {
    Text(String),
    Image(Vec<u8>),
    Files(Vec<String>),
}

#[derive(Debug)]
pub struct ClipboardManager {
    saved_content: Option<ClipboardContent>,
    clipboard: Clipboard,
}

impl ClipboardManager {
    pub fn new() -> Result<Self, ClipboardError> {
        Ok(Self {
            saved_content: None,
            clipboard: Clipboard::new()?,
        })
    }

    pub fn save(&mut self) -> Result<(), ClipboardError> {
        // Try to get text content first
        if let Ok(text) = self.clipboard.get_text() {
            self.saved_content = Some(ClipboardContent::Text(text));
        } else {
            // TODO: Handle other clipboard content types (images, files)
            self.saved_content = None;
        }

        Ok(())
    }

    pub fn restore(&mut self) -> Result<(), ClipboardError> {
        if let Some(content) = &self.saved_content {
            match content {
                ClipboardContent::Text(text) => {
                    self.clipboard.set_text(text.clone())?;
                }
                // TODO: Handle other clipboard content types
                _ => (),
            }
        }

        Ok(())
    }

    pub fn paste_text(&self, text: &str) -> Result<(), ClipboardError> {
        // Save current clipboard content temporarily
        let current_content = if let Ok(current_text) = self.clipboard.get_text() {
            Some(ClipboardContent::Text(current_text))
        } else {
            None
        };

        // Set the new text to clipboard
        self.clipboard.set_text(text.to_string())?;

        // Simulate Ctrl+V to paste
        let mut enigo = enigo::Enigo::new();
        enigo.key_down(enigo::Key::Control);
        enigo.key_click(enigo::Key::Layout('v'));
        enigo.key_up(enigo::Key::Control);

        // Restore original clipboard content after delay
        if let Some(content) = current_content {
            std::thread::sleep(std::time::Duration::from_millis(500));
            match content {
                ClipboardContent::Text(text) => {
                    self.clipboard.set_text(text)?;
                }
                _ => (),
            }
        }

        Ok(())
    }

    pub fn get_current(&self) -> Result<ClipboardContent, ClipboardError> {
        if let Ok(text) = self.clipboard.get_text() {
            Ok(ClipboardContent::Text(text))
        } else {
            Err(ClipboardError::AccessError(arboard::Error::Other {
                description: "No text content on clipboard".to_string(),
            }))
        }
    }
}

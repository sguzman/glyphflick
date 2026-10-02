use std::fmt;
use std::process::{Command, Stdio};

pub trait ClipboardBackend {
    fn copy(&mut self, text: &str) -> Result<(), ClipboardError>;
}

#[derive(Default)]
pub struct WlCopyClipboard;

impl ClipboardBackend for WlCopyClipboard {
    fn copy(&mut self, text: &str) -> Result<(), ClipboardError> {
        let status = Command::new("wl-copy")
            .args(["--type", "text/plain;charset=utf-8", "--", text])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map_err(|error| ClipboardError::new(format!("could not start wl-copy: {error}")))?;

        if status.success() {
            Ok(())
        } else {
            Err(ClipboardError::new(format!(
                "wl-copy exited with status {status}"
            )))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClipboardError {
    message: String,
}

impl ClipboardError {
    fn new(message: String) -> Self {
        Self { message }
    }

    #[cfg(test)]
    pub fn test(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for ClipboardError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ClipboardError {}

#[cfg(test)]
mod tests {
    use super::{ClipboardBackend, WlCopyClipboard};

    pub const WAYLAND_QA_TEXT: &str = "glyphflick-wayland-clipboard-qa";

    #[test]
    #[ignore = "requires a live Wayland session and wl-copy"]
    fn real_wayland_clipboard_establishes_selection() {
        let mut clipboard = WlCopyClipboard;
        clipboard
            .copy(WAYLAND_QA_TEXT)
            .expect("wl-copy backend failed on live Wayland session");
    }
}

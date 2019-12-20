use serde::Serialize;

/// Represents a thumb.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[must_use]
pub struct Thumb<'a> {
    #[serde(rename = "thumb_url")]
    url: &'a str,
    #[serde(rename = "thumb_width", skip_serializing_if = "Option::is_none")]
    width: Option<usize>,
    #[serde(rename = "thumb_height", skip_serializing_if = "Option::is_none")]
    height: Option<usize>,
}

impl<'a> Thumb<'a> {
    /// Constructs a `Thumb`.
    pub const fn new(url: &'a str) -> Self {
        Self {
            url,
            width: None,
            height: None,
        }
    }

    /// Configures the width of the thumb.
    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Configures the height of the thumb.
    pub fn height(mut self, height: usize) -> Self {
        self.height = Some(height);
        self
    }
}

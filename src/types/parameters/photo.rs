use serde::Serialize;

/// Represents a photo to be sent as an invoice preview.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
pub struct Photo<'a> {
    #[serde(rename = "photo_url")]
    url: &'a str,
    #[serde(skip_serializing_if = "Option::is_none", rename = "photo_size")]
    size: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "photo_width")]
    width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "photo_height")]
    height: Option<usize>,
}

impl<'a> Photo<'a> {
    /// Constructs a `Photo`.
    pub const fn new(url: &'a str) -> Self {
        Self {
            url,
            size: None,
            width: None,
            height: None,
        }
    }

    /// Configures the size of the photo.
    pub fn size(mut self, size: usize) -> Self {
        self.size = Some(size);
        self
    }

    /// Configures the width of the photo.
    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Configures the height of the photo.
    pub fn height(mut self, height: usize) -> Self {
        self.height = Some(height);
        self
    }
}

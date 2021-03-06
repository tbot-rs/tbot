use serde::Serialize;

/// Represents a photo to be sent as an invoice preview.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[must_use]
pub struct Photo {
    #[serde(rename = "photo_url")]
    url: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "photo_size")]
    size: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "photo_width")]
    width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "photo_height")]
    height: Option<usize>,
}

impl Photo {
    /// Constructs a `Photo`.
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            size: None,
            width: None,
            height: None,
        }
    }

    /// Configures the size of the photo.
    pub const fn size(mut self, size: usize) -> Self {
        self.size = Some(size);
        self
    }

    /// Configures the width of the photo.
    pub const fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Configures the height of the photo.
    pub const fn height(mut self, height: usize) -> Self {
        self.height = Some(height);
        self
    }
}

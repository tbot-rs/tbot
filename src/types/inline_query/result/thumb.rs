use crate::types::InteriorBorrow;
use serde::Serialize;
use std::borrow::Cow;

/// Represents a thumb.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[must_use]
pub struct Thumb<'a> {
    #[serde(rename = "thumb_url")]
    url: Cow<'a, str>,
    #[serde(rename = "thumb_width", skip_serializing_if = "Option::is_none")]
    width: Option<usize>,
    #[serde(rename = "thumb_height", skip_serializing_if = "Option::is_none")]
    height: Option<usize>,
}

impl<'a> Thumb<'a> {
    /// Constructs a `Thumb`.
    pub fn new(url: impl Into<Cow<'a, str>>) -> Self {
        Self {
            url: url.into(),
            width: None,
            height: None,
        }
    }

    /// Configures the width of the thumb.
    pub const fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Configures the height of the thumb.
    pub const fn height(mut self, height: usize) -> Self {
        self.height = Some(height);
        self
    }
}

impl<'a> InteriorBorrow<'a> for Thumb<'a> {
    fn borrow_inside(&'a self) -> Self {
        Self {
            url: self.url.borrow_inside(),
            ..*self
        }
    }
}

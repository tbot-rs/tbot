use super::{InputFile, WithName};
use serde::{ser::Serializer, Serialize};

/// Represents a thumb to be sent.
///
/// Note that a thumb cannot be sent via either a file ID or a URL.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[must_use]
pub struct Thumb(pub(crate) InputFile);

impl Thumb {
    /// Constructs a `Thumb`.
    pub fn with_bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self(InputFile::File {
            filename: "thumb.jpg".into(),
            bytes: bytes.into(),
        })
    }

    pub(super) const fn with_name<'a>(&'a self, name: &'a str) -> WithName<'a> {
        self.0.with_name(name)
    }
}

impl Serialize for Thumb {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer, "thumb")
    }
}

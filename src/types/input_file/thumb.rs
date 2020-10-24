use super::{InputFile, WithName};
use serde::{ser::Serializer, Serialize};

/// Represents a thumb to be sent.
///
/// Note that a thumb cannot be sent via either a file ID or a URL.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[must_use]
pub struct Thumb<'a>(pub(crate) InputFile<'a>);

impl<'a> Thumb<'a> {
    /// Constructs a `Thumb`.
    pub fn with_bytes(bytes: &'a [u8]) -> Self {
        Thumb(InputFile::File {
            filename: "thumb.jpg",
            bytes,
        })
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "this method is renamed to `with_bytes`"
    )]
    pub fn new(bytes: &'a [u8]) -> Self {
        Self::with_bytes(bytes)
    }

    pub(crate) const fn with_name(self, name: &'a str) -> WithName<'a> {
        self.0.with_name(name)
    }

    pub(crate) fn serialize<S>(
        &self,
        serializer: S,
        name: &str,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer, name)
    }
}

impl<'a> Serialize for Thumb<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.serialize(serializer, "thumb")
    }
}

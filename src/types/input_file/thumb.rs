use super::{InputFile, WithName};
use serde::{ser::Serializer, Serialize};
use std::borrow::Cow;

/// Represents a thumb to be sent.
///
/// Note that a thumb cannot be sent via either a file ID or a URL.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[must_use]
pub struct Thumb<'a>(pub(crate) InputFile<'a>);

impl<'a> Thumb<'a> {
    /// Constructs a `Thumb`.
    pub fn new(bytes: impl Into<Cow<'a, [u8]>>) -> Self {
        Thumb(InputFile::File {
            filename: "thumb.jpg".into(),
            bytes: bytes.into(),
        })
    }

    pub(crate) fn with_name(self, name: impl Into<Cow<'a, str>>) -> WithName<'a> {
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

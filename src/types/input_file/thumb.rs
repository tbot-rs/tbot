use super::{InputFile, WithName};
use crate::types::value::Bytes;
use serde::{ser::Serializer, Serialize};

/// Represents a thumb to be sent.
///
/// Note that a thumb cannot be sent via either a file ID or a URL.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Thumb<'a>(pub(crate) InputFile<'a>);

impl<'a> Thumb<'a> {
    /// Constructs a `Thumb`.
    pub fn new(bytes: impl Into<Bytes<'a>>) -> Self {
        Thumb(InputFile::File {
            filename: "thumb.jpg".into(),
            bytes: bytes.into(),
        })
    }

    pub(crate) fn borrow_with_name(&'a self, name: &'a str) -> WithName<'a> {
        self.0.borrow_with_name(name)
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

impl Serialize for Thumb<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.serialize(serializer, "thumb")
    }
}

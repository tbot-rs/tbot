use super::*;

/// Represents a thumb to be sent.
///
/// Note that a thumb cannot be sent via either a file ID or a URL.
pub struct Thumb<'a>(pub(crate) InputFile<'a>);

impl<'a> Thumb<'a> {
    /// Constructs a `Thumb`.
    pub fn new(bytes: &'a [u8]) -> Self {
        Thumb(InputFile::File {
            name: "thumb".into(),
            filename: "thumb.jpg",
            bytes,
        })
    }
}

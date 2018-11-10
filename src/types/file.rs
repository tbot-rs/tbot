/// Represents a file to be sent.
#[derive(Debug, PartialEq, Clone)]
pub enum File<'a> {
    /// Represents a file to be uploaded.
    File {
        /// Represents the file's name.
        name: &'a str,
        /// Represents the file's contents.
        bytes: &'a [u8],
    },
    /// Represents a file to be downloaded from a remote resource by Telegram.
    Url(&'a str),
    /// Represents the ID of a file already existing on Telegram's servers.
    Id(&'a str),
}

impl<'a> File<'a> {
    /// Returns `true` if `self` is `File::File`.
    pub fn is_file(&self) -> bool {
        match self {
            File::File {
                ..
            } => true,
            _ => false,
        }
    }
}

impl<'a> serde::Serialize for File<'a> {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match self {
            File::File {
                name,
                ..
            } => serializer.serialize_str(&format!("attach://{}", name)),
            File::Url(file) | File::Id(file) => serializer.serialize_str(file),
        }
    }
}

/// Represents a file to be sent.
#[derive(Debug, PartialEq, Clone)]
pub enum File<'a> {
    File {
        name: &'a str,
        bytes: &'a [u8],
    },
    Url(&'a str),
    Id(&'a str),
}

impl<'a> File<'a> {
    /// Returns `true` if `self` is `File::File`.
    pub fn is_file(&self) -> bool {
        match self {
            File::File { .. } => true,
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
            File::File { name, .. } => {
                serializer.serialize_str(&format!("attach://{}", name))
            },
            File::Url(file) | File::Id(file) => {
                serializer.serialize_str(file)
            }
        }
    }
}

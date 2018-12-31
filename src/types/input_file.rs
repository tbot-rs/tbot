use rand::{distributions::Alphanumeric, rngs::SmallRng, FromEntropy, Rng};

/// Represents a file.
#[derive(Debug, PartialEq, Clone)]
pub struct File<'a> {
    pub(crate) name: String,
    pub(crate) filename: &'a str,
    pub(crate) bytes: &'a [u8],
}

/// Represents a file to be sent.
#[derive(Debug, PartialEq, Clone)]
pub enum InputFile<'a> {
    /// Represents a file to be uploaded.
    File(File<'a>),
    /// Represents a file to be downloaded from a remote resource by Telegram.
    Url(&'a str),
    /// Represents the ID of a file already existing on Telegram's servers.
    Id(&'a str),
}

fn random_name() -> String {
    let mut rng = SmallRng::from_entropy();
    let ascii = rng.sample_iter(&Alphanumeric);
    ascii.take(20).collect()
}

impl<'a> InputFile<'a> {
    /// Generates a file to be sent as a file
    pub fn photo(bytes: &'a [u8]) -> Self {
        InputFile::File(File {
            name: format!("photo_{}", random_name()),
            filename: "photo.jpg",
            bytes,
        })
    }
}

impl<'a> serde::Serialize for InputFile<'a> {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match self {
            InputFile::File(file) => {
                serializer.serialize_str(&format!("attach://{}", file.name))
            }
            InputFile::Url(file) | InputFile::Id(file) => {
                serializer.serialize_str(file)
            }
        }
    }
}

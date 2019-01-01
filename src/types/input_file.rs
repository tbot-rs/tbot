use rand::{distributions::Alphanumeric, rngs::SmallRng, FromEntropy, Rng};

mod photo;

pub use self::photo::*;

pub(crate) enum InputFile<'a> {
    File {
        name: String,
        filename: &'a str,
        bytes: &'a [u8],
    },
    Url(&'a str),
    Id(&'a str),
}

fn random_name() -> String {
    let mut rng = SmallRng::from_entropy();
    let ascii = rng.sample_iter(&Alphanumeric);
    ascii.take(20).collect()
}

impl<'a> serde::Serialize for InputFile<'a> {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match self {
            InputFile::File {
                name,
                ..
            } => serializer.serialize_str(&format!("attach://{}", name)),
            InputFile::Url(file) | InputFile::Id(file) => {
                serializer.serialize_str(file)
            }
        }
    }
}

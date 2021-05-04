use is_macro::Is;
use serde::Serialize;

pub mod data;
pub mod file;
pub mod front_side;
pub mod reverse_side;
pub mod selfie;
pub mod translation_file;
pub mod unspecified;

pub use {
    data::Data,
    file::{File, Files},
    front_side::FrontSide,
    reverse_side::ReverseSide,
    selfie::Selfie,
    translation_file::{TranslationFile, TranslationFiles},
    unspecified::Unspecified,
};

/// Reperesents possible sources of an error.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Is)]
#[serde(rename_all = "snake_case", tag = "source")]
#[non_exhaustive]
#[must_use]
pub enum Source {
    /// An error with data.
    Data(Data),
    /// An error with a front side.
    FrontSide(FrontSide),
    /// An error with a reverse side.
    ReverseSide(ReverseSide),
    /// An error with a selfie.
    Selfie(Selfie),
    /// An error with a file.
    File(File),
    /// An error with several files.
    Files(Files),
    /// An error with a translation file.
    TranslationFile(TranslationFile),
    /// An error with translation files.
    TranslationFiles(TranslationFiles),
    /// An unspecified error.
    Unspecified(Unspecified),
}

impl From<Data> for Source {
    fn from(source: Data) -> Self {
        Self::Data(source)
    }
}

impl From<FrontSide> for Source {
    fn from(source: FrontSide) -> Self {
        Self::FrontSide(source)
    }
}

impl From<ReverseSide> for Source {
    fn from(source: ReverseSide) -> Self {
        Self::ReverseSide(source)
    }
}

impl From<Selfie> for Source {
    fn from(source: Selfie) -> Self {
        Self::Selfie(source)
    }
}

impl From<File> for Source {
    fn from(source: File) -> Self {
        Self::File(source)
    }
}

impl From<Files> for Source {
    fn from(source: Files) -> Self {
        Self::Files(source)
    }
}

impl From<TranslationFile> for Source {
    fn from(source: TranslationFile) -> Self {
        Self::TranslationFile(source)
    }
}

impl From<TranslationFiles> for Source {
    fn from(source: TranslationFiles) -> Self {
        Self::TranslationFiles(source)
    }
}

impl From<Unspecified> for Source {
    fn from(source: Unspecified) -> Self {
        Self::Unspecified(source)
    }
}

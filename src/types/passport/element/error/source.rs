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
pub enum Source<'a> {
    /// An error with data.
    Data(Data<'a>),
    /// An error with a front side.
    FrontSide(FrontSide<'a>),
    /// An error with a reverse side.
    ReverseSide(ReverseSide<'a>),
    /// An error with a selfie.
    Selfie(Selfie<'a>),
    /// An error with a file.
    File(File<'a>),
    /// An error with several files.
    Files(Files<'a>),
    /// An error with a translation file.
    TranslationFile(TranslationFile<'a>),
    /// An error with translation files.
    TranslationFiles(TranslationFiles<'a>),
    /// An unspecified error.
    Unspecified(Unspecified<'a>),
}

impl<'a> From<Data<'a>> for Source<'a> {
    fn from(source: Data<'a>) -> Self {
        Self::Data(source)
    }
}

impl<'a> From<FrontSide<'a>> for Source<'a> {
    fn from(source: FrontSide<'a>) -> Self {
        Self::FrontSide(source)
    }
}

impl<'a> From<ReverseSide<'a>> for Source<'a> {
    fn from(source: ReverseSide<'a>) -> Self {
        Self::ReverseSide(source)
    }
}

impl<'a> From<Selfie<'a>> for Source<'a> {
    fn from(source: Selfie<'a>) -> Self {
        Self::Selfie(source)
    }
}

impl<'a> From<File<'a>> for Source<'a> {
    fn from(source: File<'a>) -> Self {
        Self::File(source)
    }
}

impl<'a> From<Files<'a>> for Source<'a> {
    fn from(source: Files<'a>) -> Self {
        Self::Files(source)
    }
}

impl<'a> From<TranslationFile<'a>> for Source<'a> {
    fn from(source: TranslationFile<'a>) -> Self {
        Self::TranslationFile(source)
    }
}

impl<'a> From<TranslationFiles<'a>> for Source<'a> {
    fn from(source: TranslationFiles<'a>) -> Self {
        Self::TranslationFiles(source)
    }
}

impl<'a> From<Unspecified<'a>> for Source<'a> {
    fn from(source: Unspecified<'a>) -> Self {
        Self::Unspecified(source)
    }
}

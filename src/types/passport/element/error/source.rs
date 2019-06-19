use serde::Serialize;

mod data;
mod file;
mod front_side;
mod reverse_side;
mod selfie;
mod translation_file;
mod unspecified;

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
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[serde(rename_all = "snake_case", tag = "source")]
// todo: #[non_exhaustive]
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

impl<'a> Source<'a> {
    /// Checks if `self` is `Data`.
    pub fn is_data(self) -> bool {
        match self {
            Source::Data {
                ..
            } => true,
            _ => false,
        }
    }

    /// Checks if `self` is `FrontSide`.
    pub fn is_front_side(self) -> bool {
        match self {
            Source::FrontSide {
                ..
            } => true,
            _ => false,
        }
    }

    /// Checks if `self` is `ReverseSide`.
    pub fn is_reverse_side(self) -> bool {
        match self {
            Source::ReverseSide {
                ..
            } => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Selfie`.
    pub fn is_selfie(self) -> bool {
        match self {
            Source::Selfie {
                ..
            } => true,
            _ => false,
        }
    }

    /// Checks if `self` is `File`.
    pub fn is_file(self) -> bool {
        match self {
            Source::File {
                ..
            } => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Files`.
    pub fn is_files(self) -> bool {
        match self {
            Source::Files {
                ..
            } => true,
            _ => false,
        }
    }

    /// Checks if `self` is `TranslationFile`.
    pub fn is_translation_file(self) -> bool {
        match self {
            Source::TranslationFile {
                ..
            } => true,
            _ => false,
        }
    }

    /// Checks if `self` is `TranslationFiles`.
    pub fn is_translation_files(self) -> bool {
        match self {
            Source::TranslationFiles {
                ..
            } => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Unspecified`.
    pub fn is_unspecified(self) -> bool {
        match self {
            Source::Unspecified {
                ..
            } => true,
            _ => false,
        }
    }
}

impl<'a> From<Data<'a>> for Source<'a> {
    fn from(source: Data<'a>) -> Self {
        Source::Data(source)
    }
}

impl<'a> From<FrontSide<'a>> for Source<'a> {
    fn from(source: FrontSide<'a>) -> Self {
        Source::FrontSide(source)
    }
}

impl<'a> From<ReverseSide<'a>> for Source<'a> {
    fn from(source: ReverseSide<'a>) -> Self {
        Source::ReverseSide(source)
    }
}

impl<'a> From<Selfie<'a>> for Source<'a> {
    fn from(source: Selfie<'a>) -> Self {
        Source::Selfie(source)
    }
}

impl<'a> From<File<'a>> for Source<'a> {
    fn from(source: File<'a>) -> Self {
        Source::File(source)
    }
}

impl<'a> From<Files<'a>> for Source<'a> {
    fn from(source: Files<'a>) -> Self {
        Source::Files(source)
    }
}

impl<'a> From<TranslationFile<'a>> for Source<'a> {
    fn from(source: TranslationFile<'a>) -> Self {
        Source::TranslationFile(source)
    }
}

impl<'a> From<TranslationFiles<'a>> for Source<'a> {
    fn from(source: TranslationFiles<'a>) -> Self {
        Source::TranslationFiles(source)
    }
}

impl<'a> From<Unspecified<'a>> for Source<'a> {
    fn from(source: Unspecified<'a>) -> Self {
        Source::Unspecified(source)
    }
}

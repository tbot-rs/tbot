//! Types related to front side errors.

use is_macro::Is;
use serde::Serialize;

/// Represents possible element kinds for front side error.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Is)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
#[must_use]
pub enum Kind {
    /// An error in the user's passport.
    Passport,
    /// An error in the user's driver license.
    DriverLicense,
    /// An error in the user's identity card.
    IdentityCard,
    /// An error in the user's internal passport.
    InternalPassport,
}

/// Represents a [`PassportElementErrorFrontSide`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#passportelementerrorfrontside
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[must_use]
pub struct FrontSide {
    #[serde(rename = "type")]
    kind: Kind,
    file_hash: String,
}

impl FrontSide {
    /// Constructs a new `FrontSide`.
    pub fn new(kind: Kind, file_hash: impl Into<String>) -> Self {
        Self {
            kind,
            file_hash: file_hash.into(),
        }
    }
}

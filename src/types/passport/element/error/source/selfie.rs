//! Types related to selfie errors.

use serde::Serialize;

/// Represents possible element kinds for selfie error.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
// todo: #[non_exhaustive]
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

impl Kind {
    /// Checks if `self` is `Passport`.
    pub fn is_passport(self) -> bool {
        self == Kind::Passport
    }

    /// Checks if `self` is `DriverLicense`.
    pub fn is_driver_license(self) -> bool {
        self == Kind::DriverLicense
    }

    /// Checks if `self` is `IdentityCard`.
    pub fn is_identity_card(self) -> bool {
        self == Kind::IdentityCard
    }

    /// Checks if `self` is `InternalPassport`.
    pub fn is_internal_passport(self) -> bool {
        self == Kind::InternalPassport
    }
}

/// Represents a [`PassportElementErrorSelfie`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#passportelementerrorselfie
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
pub struct Selfie<'a> {
    #[serde(rename = "type")]
    kind: Kind,
    file_hash: &'a str,
}

impl<'a> Selfie<'a> {
    /// Constructs a new `Selfie`.
    pub const fn new(kind: Kind, file_hash: &'a str) -> Self {
        Self { kind, file_hash }
    }
}

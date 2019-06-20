//! Types related to reverse side errors.

use serde::Serialize;

/// Represents possible element kinds for reverse side error.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
// todo: #[non_exhaustive]
pub enum Kind {
    /// An error in the user's driver license.
    DriverLicense,
    /// An error in the user's identity card.
    IdentityCard,
}

impl Kind {
    /// Checks if `self` is `DriverLicense`.
    pub fn is_driver_license(self) -> bool {
        self == Kind::DriverLicense
    }

    /// Checks if `self` is `IdentityCard`.
    pub fn is_identity_card(self) -> bool {
        self == Kind::IdentityCard
    }
}

/// Represents a [`PassportElementErrorReverseSide`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#passportelementerrorreverseside
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
pub struct ReverseSide<'a> {
    #[serde(rename = "type")]
    kind: Kind,
    file_hash: &'a str,
}

impl<'a> ReverseSide<'a> {
    /// Constructs a new `ReverseSide`.
    pub const fn new(kind: Kind, file_hash: &'a str) -> Self {
        Self {
            kind,
            file_hash,
        }
    }
}

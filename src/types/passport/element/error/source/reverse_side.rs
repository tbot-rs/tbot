//! Types related to reverse side errors.

use is_macro::Is;
use serde::Serialize;

/// Represents possible element kinds for reverse side error.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Is)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
#[must_use]
pub enum Kind {
    /// An error in the user's driver license.
    DriverLicense,
    /// An error in the user's identity card.
    IdentityCard,
}

/// Represents a [`PassportElementErrorReverseSide`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#passportelementerrorreverseside
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[must_use]
pub struct ReverseSide<'a> {
    #[serde(rename = "type")]
    kind: Kind,
    file_hash: &'a str,
}

impl<'a> ReverseSide<'a> {
    /// Constructs a new `ReverseSide`.
    pub const fn new(kind: Kind, file_hash: &'a str) -> Self {
        Self { kind, file_hash }
    }
}

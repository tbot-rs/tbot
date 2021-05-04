//! Types related to data passport errors.

use is_macro::Is;
use serde::Serialize;

/// Represents possible element kinds for data error.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Is)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
#[must_use]
pub enum Kind {
    /// An error in the user's personal details.
    PersonalDetails,
    /// An error in the user's passport.
    Passport,
    /// An error in the user's driver license.
    DriverLicense,
    /// An error in the user's identity card.
    IdentityCard,
    /// An error in the user's internal passport.
    InternalPassport,
    /// An error in the user's address.
    Address,
}

/// Represents a [`PassportElementErrorDataField`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#passportelementerrordatafield
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[must_use]
pub struct Data {
    #[serde(rename = "type")]
    kind: Kind,
    field_name: String,
    data_hash: String,
}

impl Data {
    /// Constructs a new `Data`.
    pub fn new(
        kind: Kind,
        field_name: impl Into<String>,
        data_hash: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            field_name: field_name.into(),
            data_hash: data_hash.into(),
        }
    }
}

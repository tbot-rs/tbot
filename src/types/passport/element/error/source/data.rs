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
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[must_use]
pub struct Data<'a> {
    #[serde(rename = "type")]
    kind: Kind,
    field_name: &'a str,
    data_hash: &'a str,
}

impl<'a> Data<'a> {
    /// Constructs a new `Data`.
    pub const fn new(
        kind: Kind,
        field_name: &'a str,
        data_hash: &'a str,
    ) -> Self {
        Self {
            kind,
            field_name,
            data_hash,
        }
    }
}

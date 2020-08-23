//! Types related to data passport errors.

use crate::types::InteriorBorrow;
use is_macro::Is;
use serde::Serialize;
use std::borrow::Cow;

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
pub struct Data<'a> {
    #[serde(rename = "type")]
    kind: Kind,
    field_name: Cow<'a, str>,
    data_hash: Cow<'a, str>,
}

impl<'a> Data<'a> {
    /// Constructs a new `Data`.
    pub fn new(
        kind: Kind,
        field_name: impl Into<Cow<'a, str>>,
        data_hash: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            kind,
            field_name: field_name.into(),
            data_hash: data_hash.into(),
        }
    }
}

impl<'a> InteriorBorrow<'a> for Data<'a> {
    fn borrow_inside(&'a self) -> Self {
        Self {
            field_name: self.field_name.borrow_inside(),
            data_hash: self.data_hash.borrow_inside(),
            ..*self
        }
    }
}

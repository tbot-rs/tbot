//! Types related to unspecified errors.

use is_macro::Is;
use serde::Serialize;
use std::borrow::Cow;

/// Represents possible element kinds for unspecified error.
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
    /// An error in the user's utility bill.
    UtilityBill,
    /// An error in the user's bank statement.
    BankStatement,
    /// An error in the user's rental agreement.
    RentalAgreement,
    /// An error in the user's passport registration.
    PassportRegistration,
    /// An error in the user's temporary registration.
    TemporaryRegistration,
    /// An error in the user's phone number.
    PhoneNumber,
    /// An error in the user's email.
    Email,
}

/// Represents a [`PassportElementErrorUnspecified`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#passportelementerrorunspecified
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[must_use]
pub struct Unspecified<'a> {
    #[serde(rename = "type")]
    kind: Kind,
    element_hash: Cow<'a, str>,
}

impl<'a> Unspecified<'a> {
    /// Constructs a new `Unspecified`.
    pub fn new(kind: Kind, element_hash: impl Into<Cow<'a, str>>) -> Self {
        Self { kind, element_hash: element_hash.into() }
    }
}

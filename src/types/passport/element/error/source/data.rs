//! Types related to data passport errors.

use serde::Serialize;

/// Represents possible element kinds for data error.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
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

impl Kind {
    /// Checks if `self` is `PersonalDetails`.
    #[must_use]
    pub fn is_personal_details(self) -> bool {
        self == Self::PersonalDetails
    }

    /// Checks if `self` is `Passport`.
    #[must_use]
    pub fn is_passport(self) -> bool {
        self == Self::Passport
    }

    /// Checks if `self` is `DriverLicense`.
    #[must_use]
    pub fn is_driver_license(self) -> bool {
        self == Self::DriverLicense
    }

    /// Checks if `self` is `IdentityCard`.
    #[must_use]
    pub fn is_identity_card(self) -> bool {
        self == Self::IdentityCard
    }

    /// Checks if `self` is `InternalPassport`.
    #[must_use]
    pub fn is_internal_passport(self) -> bool {
        self == Self::InternalPassport
    }

    /// Checks if `self` is `Address`.
    #[must_use]
    pub fn is_address(self) -> bool {
        self == Self::Address
    }
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

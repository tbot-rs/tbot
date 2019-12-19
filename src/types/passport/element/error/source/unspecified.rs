//! Types related to unspecified errors.

use serde::Serialize;

/// Represents possible element kinds for unspecified error.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
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
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
pub struct Unspecified<'a> {
    #[serde(rename = "type")]
    kind: Kind,
    element_hash: &'a str,
}

impl Kind {
    /// Checks if `self` is `PersonalDetails`.
    pub fn is_personal_details(self) -> bool {
        self == Self::PersonalDetails
    }

    /// Checks if `self` is `Passport`.
    pub fn is_passport(self) -> bool {
        self == Self::Passport
    }

    /// Checks if `self` is ` DriverLicense`.
    pub fn is_driver_license(self) -> bool {
        self == Self::DriverLicense
    }

    /// Checks if `self` is `IdentityCard`.
    pub fn is_identity_card(self) -> bool {
        self == Self::IdentityCard
    }

    /// Checks if `self` is `InternalPassport`.
    pub fn is_internal_passport(self) -> bool {
        self == Self::InternalPassport
    }

    /// Checks if `self` is `Address`.
    pub fn is_address(self) -> bool {
        self == Self::Address
    }

    /// Checks if `self` is `UtilityBill`.
    pub fn is_utility_bill(self) -> bool {
        self == Self::UtilityBill
    }

    /// Checks if `self` is `BankStatement`.
    pub fn is_bank_statement(self) -> bool {
        self == Self::BankStatement
    }

    /// Checks if `self` is `RentalAgreement`.
    pub fn is_rental_agreement(self) -> bool {
        self == Self::RentalAgreement
    }

    /// Checks if `self` is `PassportRegistration`.
    pub fn is_passport_registration(self) -> bool {
        self == Self::PassportRegistration
    }

    /// Checks if `self` is `TemporaryRegistration`.
    pub fn is_temporary_registration(self) -> bool {
        self == Self::TemporaryRegistration
    }

    /// Checks if `self` is `PhoneNumber`.
    pub fn is_phone_number(self) -> bool {
        self == Self::PhoneNumber
    }

    /// Checks if `self` is `Email`.
    pub fn is_email(self) -> bool {
        self == Self::Email
    }
}

impl<'a> Unspecified<'a> {
    /// Constructs a new `Unspecified`.
    pub const fn new(kind: Kind, element_hash: &'a str) -> Self {
        Self { kind, element_hash }
    }
}

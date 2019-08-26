//! Types related to unspecified errors.

use serde::Serialize;

/// Represents possible element kinds for unspecified error.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
// todo: #[non_exhaustive]
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
        self == Kind::PersonalDetails
    }

    /// Checks if `self` is `Passport`.
    pub fn is_passport(self) -> bool {
        self == Kind::Passport
    }

    /// Checks if `self` is ` DriverLicense`.
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

    /// Checks if `self` is `Address`.
    pub fn is_address(self) -> bool {
        self == Kind::Address
    }

    /// Checks if `self` is `UtilityBill`.
    pub fn is_utility_bill(self) -> bool {
        self == Kind::UtilityBill
    }

    /// Checks if `self` is `BankStatement`.
    pub fn is_bank_statement(self) -> bool {
        self == Kind::BankStatement
    }

    /// Checks if `self` is `RentalAgreement`.
    pub fn is_rental_agreement(self) -> bool {
        self == Kind::RentalAgreement
    }

    /// Checks if `self` is `PassportRegistration`.
    pub fn is_passport_registration(self) -> bool {
        self == Kind::PassportRegistration
    }

    /// Checks if `self` is `TemporaryRegistration`.
    pub fn is_temporary_registration(self) -> bool {
        self == Kind::TemporaryRegistration
    }

    /// Checks if `self` is `PhoneNumber`.
    pub fn is_phone_number(self) -> bool {
        self == Kind::PhoneNumber
    }

    /// Checks if `self` is `Email`.
    pub fn is_email(self) -> bool {
        self == Kind::Email
    }
}

impl<'a> Unspecified<'a> {
    /// Constructs a new `Unspecified`.
    pub const fn new(kind: Kind, element_hash: &'a str) -> Self {
        Self { kind, element_hash }
    }
}

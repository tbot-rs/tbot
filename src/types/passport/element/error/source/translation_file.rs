//! Types related to translation file errors.

use serde::Serialize;

/// Represents possible element kinds for translation error.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
#[must_use]
pub enum Kind {
    /// An error in the user's passport.
    Passport,
    /// An error in the user's driver license.
    DriverLicense,
    /// An error in the user's identity card.
    IdentityCard,
    /// An error in the user's internal passport.
    InternalPassport,
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
}

impl Kind {
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

    /// Checks if `self` is `UtilityBill`.
    #[must_use]
    pub fn is_utility_bill(self) -> bool {
        self == Self::UtilityBill
    }

    /// Checks if `self` is `BankStatement`.
    #[must_use]
    pub fn is_bank_statement(self) -> bool {
        self == Self::BankStatement
    }

    /// Checks if `self` is `RentalAgreement`.
    #[must_use]
    pub fn is_rental_agreement(self) -> bool {
        self == Self::RentalAgreement
    }

    /// Checks if `self` is `PassportRegistration`.
    #[must_use]
    pub fn is_passport_registration(self) -> bool {
        self == Self::PassportRegistration
    }

    /// Checks if `self` is `TemporaryRegistration`.
    #[must_use]
    pub fn is_temporary_registration(self) -> bool {
        self == Self::TemporaryRegistration
    }
}

/// Represents a [`PassportElementErrorTranslationFile`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#passportelementerrortranslationfile
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[must_use]
pub struct TranslationFile<'a> {
    #[serde(rename = "type")]
    kind: Kind,
    file_hash: &'a str,
}

impl<'a> TranslationFile<'a> {
    /// Constructs a new `TranslationFile`.
    pub const fn new(kind: Kind, file_hash: &'a str) -> Self {
        Self { kind, file_hash }
    }
}

/// Represents a [`PassportElementErrorTranslationFiles`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#passportelementerrortranslationfiles
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[must_use]
pub struct TranslationFiles<'a> {
    #[serde(rename = "type")]
    kind: Kind,
    file_hashes: &'a [&'a str],
}

impl<'a> TranslationFiles<'a> {
    /// Constructs new `TranslationFiles`.
    pub const fn new(kind: Kind, file_hashes: &'a [&'a str]) -> Self {
        Self { kind, file_hashes }
    }
}

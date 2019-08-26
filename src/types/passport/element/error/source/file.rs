//! Types related to file passport errors.

use serde::Serialize;

/// Represents possible element kinds for file error.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
// todo: #[non_exhaustive]
pub enum Kind {
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
}

/// Represents a [`PassportElementErrorFile`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#passportelementerrorfile
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
pub struct File<'a> {
    #[serde(rename = "type")]
    kind: Kind,
    file_hash: &'a str,
}

impl<'a> File<'a> {
    /// Constructs a new `File`.
    pub const fn new(kind: Kind, file_hash: &'a str) -> Self {
        Self { kind, file_hash }
    }
}

/// Represents a [`PassportElementErrorFiles`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#passportelementerrorfiles
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
pub struct Files<'a> {
    #[serde(rename = "type")]
    kind: Kind,
    file_hashes: &'a [&'a str],
}

impl<'a> Files<'a> {
    /// Constructs neile`.
    pub const fn new(kind: Kind, file_hashes: &'a [&'a str]) -> Self {
        Self { kind, file_hashes }
    }
}

//! Types related to file passport errors.

use crate::types::InteriorBorrow;
use is_macro::Is;
use serde::Serialize;
use std::borrow::Cow;

/// Represents possible element kinds for file error.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Is)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
#[must_use]
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

/// Represents a [`PassportElementErrorFile`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#passportelementerrorfile
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[must_use]
pub struct File<'a> {
    #[serde(rename = "type")]
    kind: Kind,
    file_hash: Cow<'a, str>,
}

impl<'a> File<'a> {
    /// Constructs a new `File`.
    pub fn new(kind: Kind, file_hash: impl Into<Cow<'a, str>>) -> Self {
        Self {
            kind,
            file_hash: file_hash.into(),
        }
    }
}

impl<'a> InteriorBorrow<'a> for File<'a> {
    fn borrow_inside(&'a self) -> Self {
        Self {
            file_hash: self.file_hash.borrow_inside(),
            ..*self
        }
    }
}

/// Represents a [`PassportElementErrorFiles`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#passportelementerrorfiles
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[must_use]
pub struct Files<'a> {
    #[serde(rename = "type")]
    kind: Kind,
    file_hashes: Vec<Cow<'a, str>>,
}

impl<'a> Files<'a> {
    /// Constructs new `Files`.
    pub fn new<F>(kind: Kind, file_hashes: F) -> Self
    where
        F: IntoIterator,
        F::Item: Into<Cow<'a, str>>,
    {
        Self {
            kind,
            file_hashes: file_hashes.into_iter().map(Into::into).collect(),
        }
    }
}

impl<'a> InteriorBorrow<'a> for Files<'a> {
    fn borrow_inside(&'a self) -> Self {
        Self {
            file_hashes: self.file_hashes.borrow_inside(),
            ..*self
        }
    }
}

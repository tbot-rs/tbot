use crate::types::passport;
use is_macro::Is;

/// Represents different kinds of [`Element`].
///
/// [`Element`]: ./struct.Element.html
#[derive(Debug, PartialEq, Eq, Clone, Hash, Is)]
#[non_exhaustive]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum Kind {
    /// The user's personal details.
    PersonalDetails(String),
    /// The user's passport.
    #[non_exhaustive]
    Passport {
        /// Data related to the passport.
        data: String,
        /// The front size of the passport.
        front_side: passport::File,
        /// The user's selfie with the passport.
        selfie: passport::File,
        /// Translated versions of the passport.
        translation: Vec<passport::File>,
    },
    /// The user's driver license.
    #[non_exhaustive]
    DriverLicense {
        /// Data related to the license.
        data: String,
        /// The front side of the license.
        front_side: passport::File,
        /// The reverse side of the license.
        reverse_side: passport::File,
        /// The user's selfie with the license.
        selfie: passport::File,
        /// Translated versions of the license.
        translation: Vec<passport::File>,
    },
    /// The user's identity card.
    #[non_exhaustive]
    IdentityCard {
        /// Data related to the identity card.
        data: String,
        /// The front side of the identity card.
        front_side: passport::File,
        /// The reverse side of the identity card.
        reverse_side: passport::File,
        /// The user's selfie with the license.
        selfie: passport::File,
        /// Translated versions of the identity card.
        translation: Vec<passport::File>,
    },
    /// The user's internal passport.
    #[non_exhaustive]
    InternalPassport {
        /// Data related to the passport.
        data: String,
        /// The front side of the passport.
        front_side: passport::File,
        /// The user's selfie with the passport.
        selfie: passport::File,
        /// Translated versions of the passport.
        translation: Vec<passport::File>,
    },
    /// The user's address.
    Address(String),
    /// The user's utility bill.
    #[non_exhaustive]
    UtilityBill {
        /// Photos of the bill.
        files: Vec<passport::File>,
        /// Translated versions of the bill.
        translation: Vec<passport::File>,
    },
    /// The user's bank statement.
    #[non_exhaustive]
    BankStatement {
        /// Photos of the statement.
        files: Vec<passport::File>,
        /// Translated versions of the statement.
        translation: Vec<passport::File>,
    },
    /// The user's rental agreement.
    #[non_exhaustive]
    RentalAgreement {
        /// Photos of the agreement.
        files: Vec<passport::File>,
        /// Translated versions of the agreement.
        translation: Vec<passport::File>,
    },
    /// The user's passport registration.
    #[non_exhaustive]
    PassportRegistration {
        /// Photos of the registration.
        files: Vec<passport::File>,
        /// Translated versions of the registration.
        translation: Vec<passport::File>,
    },
    /// The user's temporary registration.
    #[non_exhaustive]
    TemporaryRegistration {
        /// Photos of the registration.
        files: Vec<passport::File>,
        /// Translated versions of the registration.
        translation: Vec<passport::File>,
    },
    /// The user's phone number.
    PhoneNumber(String),
    /// The user's email.
    Email(String),
}

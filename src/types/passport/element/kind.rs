use crate::types::passport;

/// Represents different kinds of [`Element`].
///
/// [`Element`]: ./struct.Element.html
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[non_exhaustive]
#[must_use]
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

impl Kind {
    /// Checks if `self` is `PersonalDetails`.
    #[must_use]
    pub fn is_personal_details(&self) -> bool {
        match self {
            Self::PersonalDetails(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Passport`.
    #[must_use]
    pub fn is_passport(&self) -> bool {
        match self {
            Self::Passport { .. } => true,
            _ => false,
        }
    }

    /// Checks if `self` is `DriverLicense`.
    #[must_use]
    pub fn is_driver_license(&self) -> bool {
        match self {
            Self::DriverLicense { .. } => true,
            _ => false,
        }
    }

    /// Checks if `self` is `IdentityCard`.
    #[must_use]
    pub fn is_identity_card(&self) -> bool {
        match self {
            Self::IdentityCard { .. } => true,
            _ => false,
        }
    }

    /// Checks if `self` is `InternalPassport`.
    #[must_use]
    pub fn is_internal_passport(&self) -> bool {
        match self {
            Self::InternalPassport { .. } => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Address`.
    #[must_use]
    pub fn is_address(&self) -> bool {
        match self {
            Self::Address(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `UtilityBill`.
    #[must_use]
    pub fn is_utility_bill(&self) -> bool {
        match self {
            Self::UtilityBill { .. } => true,
            _ => false,
        }
    }

    /// Checks if `self` is `BankStatement`.
    #[must_use]
    pub fn is_bank_statement(&self) -> bool {
        match self {
            Self::BankStatement { .. } => true,
            _ => false,
        }
    }

    /// Checks if `self` is `RentalAgreement`.
    #[must_use]
    pub fn is_rental_agreement(&self) -> bool {
        match self {
            Self::RentalAgreement { .. } => true,
            _ => false,
        }
    }

    /// Checks if `self` is `PassportRegistration`.
    #[must_use]
    pub fn is_passport_registration(&self) -> bool {
        match self {
            Self::PassportRegistration { .. } => true,
            _ => false,
        }
    }

    /// Checks if `self` is `TemporaryRegistration`.
    #[must_use]
    pub fn is_temporary_registration(&self) -> bool {
        match self {
            Self::TemporaryRegistration { .. } => true,
            _ => false,
        }
    }

    /// Checks if `self` is `PhoneNumber`.
    #[must_use]
    pub fn is_phone_number(&self) -> bool {
        match self {
            Self::PhoneNumber(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Email`.
    #[must_use]
    pub fn is_email(&self) -> bool {
        match self {
            Self::Email(..) => true,
            _ => false,
        }
    }
}

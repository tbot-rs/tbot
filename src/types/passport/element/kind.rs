use crate::types::passport;

/// Represents different kinds of [`Element`].
///
/// [`Element`]: ./struct.Element.html
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
// todo: #[non_exhaustive]
pub enum Kind {
    /// THe user's personal details.
    PersonalDetails(String),
    /// The user's passport.
    // todo: #[non_exhaustive]
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
    // todo: #[non_exhaustive]
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
    // todo: #[non_exhaustive]
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
    // todo: #[non_exhaustive]
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
    // todo: #[non_exhaustive]
    UtilityBill {
        /// Photos of the bill.
        files: Vec<passport::File>,
        /// Translated versions of the bill.
        translation: Vec<passport::File>,
    },
    /// The user's bank statement.
    // todo: #[non_exhaustive]
    BankStatement {
        /// Photos of the statement.
        files: Vec<passport::File>,
        /// Translated versions of the statement.
        translation: Vec<passport::File>,
    },
    /// The user's rental agreement.
    // todo: #[non_exhaustive]
    RentalAgreement {
        /// Photos of the agreement.
        files: Vec<passport::File>,
        /// Translated versions of the agreement.
        translation: Vec<passport::File>,
    },
    /// The user's passport registration.
    // todo: #[non_exhaustive]
    PassportRegistration {
        /// Photos of the registration.
        files: Vec<passport::File>,
        /// Translated versions of the registration.
        translation: Vec<passport::File>,
    },
    /// The user's temporary registration.
    // todo: #[non_exhaustive]
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
    pub fn is_personal_details(&self) -> bool {
        match self {
            Kind::PersonalDetails(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Passport`.
    pub fn is_passport(&self) -> bool {
        match self {
            Kind::Passport {
                ..
            } => true,
            _ => false,
        }
    }

    /// Checks if `self` is `DriverLicense`.
    pub fn is_driver_license(&self) -> bool {
        match self {
            Kind::DriverLicense {
                ..
            } => true,
            _ => false,
        }
    }

    /// Checks if `self` is `IdentityCard`.
    pub fn is_identity_card(&self) -> bool {
        match self {
            Kind::IdentityCard {
                ..
            } => true,
            _ => false,
        }
    }

    /// Checks if `self` is `InternalPassport`.
    pub fn is_internal_passport(&self) -> bool {
        match self {
            Kind::InternalPassport {
                ..
            } => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Address`.
    pub fn is_address(&self) -> bool {
        match self {
            Kind::Address(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `UtilityBill`.
    pub fn is_utility_bill(&self) -> bool {
        match self {
            Kind::UtilityBill {
                ..
            } => true,
            _ => false,
        }
    }

    /// Checks if `self` is `BankStatement`.
    pub fn is_bank_statement(&self) -> bool {
        match self {
            Kind::BankStatement {
                ..
            } => true,
            _ => false,
        }
    }

    /// Checks if `self` is `RentalAgreement`.
    pub fn is_rental_agreement(&self) -> bool {
        match self {
            Kind::RentalAgreement {
                ..
            } => true,
            _ => false,
        }
    }

    /// Checks if `self` is `PassportRegistration`.
    pub fn is_passport_registration(&self) -> bool {
        match self {
            Kind::PassportRegistration {
                ..
            } => true,
            _ => false,
        }
    }

    /// Checks if `self` is `TemporaryRegistration`.
    pub fn is_temporary_registration(&self) -> bool {
        match self {
            Kind::TemporaryRegistration {
                ..
            } => true,
            _ => false,
        }
    }

    /// Checks if `self` is `PhoneNumber`.
    pub fn is_phone_number(&self) -> bool {
        match self {
            Kind::PhoneNumber(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Email`.
    pub fn is_email(&self) -> bool {
        match self {
            Kind::Email(..) => true,
            _ => false,
        }
    }
}

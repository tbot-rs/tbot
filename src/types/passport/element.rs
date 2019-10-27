//! Types related to passport elements.

use serde::de::{
    self, Deserialize, Deserializer, IgnoredAny, MapAccess, Visitor,
};
use std::fmt::{self, Formatter};

pub mod error;
mod kind;

pub use {error::Error, kind::Kind};

/// Represents an [`EncryptedPassportElement`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#encryptedpassportelement
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
// todo: #[non_exhaustive]
pub struct Element {
    /// The kind of the element.
    pub kind: Kind,
    /// The hash of the element.
    pub hash: String,
}

const TYPE: &str = "type";
const DATA: &str = "data";
const PHONE_NUMBER: &str = "phone_number";
const EMAIL: &str = "email";
const FILES: &str = "files";
const FRONT_SIDE: &str = "front_side";
const REVERSE_SIDE: &str = "reverse_side";
const SELFIE: &str = "selfie";
const TRANSLATION: &str = "translation";
const HASH: &str = "hash";

const PERSONAL_DETAILS: &str = "personal_details";
const PASSPORT: &str = "passport";
const DRIVER_LICENSE: &str = "driver_license";
const IDENTITY_CARD: &str = "identity_card";
const INTERNAL_PASSPORT: &str = "internal_passport";
const ADDRESS: &str = "address";
const UTILITY_BILL: &str = "utility_bill";
const BANK_STATEMENT: &str = "bank_statement";
const RENTAL_AGREEMENT: &str = "rental_agreement";
const PASSPORT_REGISTRATION: &str = "passport_registration";
const TEMPORARY_REGISTRATION: &str = "temporary_registration";
// phone_number and email already declared

struct ElementVisitor;

impl<'v> Visitor<'v> for ElementVisitor {
    type Value = Element;

    fn expecting(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "struct Element")
    }

    #[allow(clippy::too_many_lines)]
    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'v>,
    {
        let mut kind = None;
        let mut data = None;
        let mut phone_number = None;
        let mut email = None;
        let mut files = None;
        let mut front_side = None;
        let mut reverse_side = None;
        let mut selfie = None;
        let mut translation = None;
        let mut hash = None;

        while let Some(key) = map.next_key()? {
            match key {
                TYPE => kind = Some(map.next_value()?),
                DATA => data = Some(map.next_value()?),
                PHONE_NUMBER => phone_number = Some(map.next_value()?),
                EMAIL => email = Some(map.next_value()?),
                FILES => files = Some(map.next_value()?),
                FRONT_SIDE => front_side = Some(map.next_value()?),
                REVERSE_SIDE => reverse_side = Some(map.next_value()?),
                SELFIE => selfie = Some(map.next_value()?),
                TRANSLATION => translation = Some(map.next_value()?),
                HASH => hash = Some(map.next_value()?),
                _ => {
                    let _ = map.next_value::<IgnoredAny>()?;
                }
            }
        }

        let kind = kind.ok_or_else(|| de::Error::missing_field(TYPE))?;
        let data = data.ok_or_else(|| de::Error::missing_field(DATA));
        let phone_number =
            phone_number.ok_or_else(|| de::Error::missing_field(PHONE_NUMBER));
        let email = email.ok_or_else(|| de::Error::missing_field(EMAIL));
        let files = files.ok_or_else(|| de::Error::missing_field(FILES));
        let front_side =
            front_side.ok_or_else(|| de::Error::missing_field(FRONT_SIDE));
        let reverse_side =
            reverse_side.ok_or_else(|| de::Error::missing_field(REVERSE_SIDE));
        let selfie = selfie.ok_or_else(|| de::Error::missing_field(SELFIE));
        let translation =
            translation.ok_or_else(|| de::Error::missing_field(TRANSLATION));
        let hash = hash.ok_or_else(|| de::Error::missing_field(HASH))?;

        let kind = match kind {
            PERSONAL_DETAILS => Kind::PersonalDetails(data?),
            PASSPORT => Kind::Passport {
                data: data?,
                front_side: front_side?,
                selfie: selfie?,
                translation: translation?,
            },
            DRIVER_LICENSE => Kind::DriverLicense {
                data: data?,
                front_side: front_side?,
                reverse_side: reverse_side?,
                selfie: selfie?,
                translation: translation?,
            },
            IDENTITY_CARD => Kind::IdentityCard {
                data: data?,
                front_side: front_side?,
                reverse_side: reverse_side?,
                selfie: selfie?,
                translation: translation?,
            },
            INTERNAL_PASSPORT => Kind::InternalPassport {
                data: data?,
                front_side: front_side?,
                selfie: selfie?,
                translation: translation?,
            },
            ADDRESS => Kind::Address(data?),
            UTILITY_BILL => Kind::UtilityBill {
                files: files?,
                translation: translation?,
            },
            BANK_STATEMENT => Kind::BankStatement {
                files: files?,
                translation: translation?,
            },
            RENTAL_AGREEMENT => Kind::RentalAgreement {
                files: files?,
                translation: translation?,
            },
            PASSPORT_REGISTRATION => Kind::PassportRegistration {
                files: files?,
                translation: translation?,
            },
            TEMPORARY_REGISTRATION => Kind::TemporaryRegistration {
                files: files?,
                translation: translation?,
            },
            PHONE_NUMBER => Kind::PhoneNumber(phone_number?),
            EMAIL => Kind::Email(email?),
            kind => {
                return Err(de::Error::unknown_variant(
                    kind,
                    &[
                        PERSONAL_DETAILS,
                        PASSPORT,
                        DRIVER_LICENSE,
                        IDENTITY_CARD,
                        INTERNAL_PASSPORT,
                        ADDRESS,
                        UTILITY_BILL,
                        BANK_STATEMENT,
                        RENTAL_AGREEMENT,
                        PASSPORT_REGISTRATION,
                        TEMPORARY_REGISTRATION,
                    ],
                ))
            }
        };

        Ok(Element { kind, hash })
    }
}

impl<'de> Deserialize<'de> for Element {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "Element",
            &[
                TYPE,
                DATA,
                PHONE_NUMBER,
                EMAIL,
                FILES,
                FRONT_SIDE,
                REVERSE_SIDE,
                SELFIE,
                TRANSLATION,
                HASH,
            ],
            ElementVisitor,
        )
    }
}

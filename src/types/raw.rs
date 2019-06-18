// There are many types that will be changed, unrawed and documented only later.
#![allow(missing_docs)]

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,
    pub credentails: EncryptedCredentails,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct PassportFile {
    pub file_id: String,
    pub file_size: u32,
    pub file_date: i64,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EncryptedPassportElementType {
    PersonalDetails,
    Passport,
    DriverLicense,
    IdentityCard,
    InternalPassport,
    Address,
    UtilityBill,
    BankStatement,
    RentalAgreement,
    PassportRegistration,
    TemporaryRegistration,
    PhoneNumber,
    Email,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct EncryptedPassportElement {
    pub element_type: EncryptedPassportElementType,
    pub data: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub files: Option<Vec<PassportFile>>,
    pub front_side: Option<PassportFile>,
    pub reverse_side: Option<PassportFile>,
    pub selfie: Option<PassportFile>,
    pub translation: Option<Vec<PassportFile>>,
    pub hash: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct EncryptedCredentails {
    pub data: String,
    pub hash: String,
    pub secret: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub enum PassportElementError {
    DataField {
        field_type: String,
        field_name: EncryptedPassportElementType,
        data_hash: String,
        message: String,
    },
    FrontSide {
        field_type: EncryptedPassportElementType,
        file_hash: String,
        message: String,
    },
    ReverseSide {
        field_type: EncryptedPassportElementType,
        file_hash: String,
        message: String,
    },
    Selfie {
        field_type: EncryptedPassportElementType,
        file_hash: String,
        message: String,
    },
    File {
        field_type: EncryptedPassportElementType,
        file_hash: String,
        message: String,
    },
    Files {
        field_type: EncryptedPassportElementType,
        file_hashes: Vec<String>,
        message: String,
    },
    TranslationFile {
        field_type: EncryptedPassportElementType,
        file_hash: String,
        message: String,
    },
    TranslationFiles {
        field_type: EncryptedPassportElementType,
        file_hashes: Vec<String>,
        message: String,
    },
    Unspecified {
        field_type: EncryptedPassportElementType,
        element_hash: String,
        message: String,
    },
}

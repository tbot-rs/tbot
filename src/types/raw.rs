// There are many types that will be changed, unrawed and documented only later.
#![allow(missing_docs)]

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct ChatMember {
    pub user: User,
    pub status: String,
    pub until_date: Option<i64>,
    pub can_be_edited: Option<bool>,
    pub can_change_info: Option<bool>,
    pub can_post_messages: Option<bool>,
    pub can_edit_messages: Option<bool>,
    pub can_delete_messages: Option<bool>,
    pub can_invite_users: Option<bool>,
    pub can_restrict_members: Option<bool>,
    pub can_pin_messages: Option<bool>,
    pub can_promote_members: Option<bool>,
    pub can_send_messages: Option<bool>,
    pub can_send_media_messages: Option<bool>,
    pub can_send_other_messages: Option<bool>,
    pub can_add_web_page_previews: Option<bool>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Chat {
    pub id: i64,
    #[serde(rename = "type")]
    pub chat_type: ChatTypes,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub all_members_are_administrators: Option<bool>,
    pub photo: Option<ChatPhoto>,
    pub description: Option<String>,
    pub invite_link: Option<String>,
    pub pinned_message: Option<Box<Message>>,
    pub sticker_set_name: Option<String>,
    pub can_set_sticker_set: Option<bool>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct SuccessfulPayment {
    pub currency: String,
    pub total_amount: u32,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<OrderInfo>,
    pub telegram_payment_charge_id: String,
    pub provider_payment_charge_id: String,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<Message>,
    pub inline_message_id: Option<String>,
    pub chat_instance: String,
    pub data: Option<String>,
    pub game_short_name: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct LabeledPrice {
    pub label: String,
    pub amount: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct ShippingAddress {
    pub country_code: String,
    pub state: String,
    pub city: String,
    pub street_line1: String,
    pub street_line2: String,
    pub post_code: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct OrderInfo {
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub shipping_address: Option<ShippingAddress>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct ShippingOption {
    pub id: String,
    pub title: String,
    pub prices: Vec<LabeledPrice>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct ShippingQuery {
    pub id: String,
    pub from: User,
    pub invoice_payload: String,
    pub shipping_address: ShippingAddress,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct PreCheckoutQuery {
    pub id: String,
    pub from: User,
    pub currency: String,
    pub total_amount: u32,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<OrderInfo>,
}

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

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct GameHighScore {
    pub position: u32,
    pub user: User,
    pub score: i32,
}

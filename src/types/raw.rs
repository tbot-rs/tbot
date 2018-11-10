// There are many types that will be changed, unrawed and documented only later.
#![allow(missing_docs)]

use super::*;

#[derive(Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ChatTypes {
    Private,
    Group,
    Supergroup,
    Channel,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
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

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct ChatPhoto {
    pub small_file_id: String,
    pub big_file_id: String,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
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

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Message {
    pub message_id: i64,
    pub from: User,
    pub date: i64,
    pub chat: Chat,
    pub forward_from: Option<User>,
    pub forward_from_chat: Option<Chat>,
    pub forward_from_message_id: Option<i64>,
    pub forward_signature: Option<String>,
    pub forward_date: Option<i64>,
    pub reply_to_message: Option<Box<Message>>,
    pub edit_date: Option<i64>,
    pub media_group_id: Option<String>,
    pub author_signature: Option<String>,
    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub audio: Option<Audio>,
    pub document: Option<Document>,
    pub animation: Option<Animation>,
    pub game: Option<Game>,
    pub photo: Option<Vec<PhotoSize>>,
    pub sticker: Option<Sticker>,
    pub video: Option<Video>,
    pub voice: Option<Voice>,
    pub video_note: Option<VideoNote>,
    pub caption: Option<String>,
    pub contact: Option<Contact>,
    pub location: Option<Location>,
    pub venue: Option<Venue>,
    pub new_chat_members: Option<Vec<User>>,
    pub left_chat_member: Option<User>,
    pub new_chat_title: Option<String>,
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    pub delete_chat_photo: Option<bool>,
    pub group_chat_created: Option<bool>,
    pub supergroup_chat_created: Option<bool>,
    pub channel_chat_created: Option<bool>,
    pub migrate_to_chat_id: Option<i64>,
    pub migrate_from_chat_id: Option<i64>,
    pub pinned_message: Option<Box<Message>>,
    pub invoice: Option<Invoice>,
    pub successful_payment: Option<SuccessfulPayment>,
    pub conneted_website: Option<String>,
    pub passport_data: Option<PassportData>,
}

#[derive(Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum MessageEntityTypes {
    Mention,
    Hashtag,
    Cashtag,
    BotCommand,
    Url,
    Email,
    PhoneNumber,
    Bold,
    Italic,
    Code,
    Pre,
    TextLink,
    TextMention,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct MessageEntity {
    #[serde(rename = "type")]
    pub entity_type: MessageEntityTypes,
    pub offset: i64,
    pub length: i64,
    pub url: Option<String>,
    pub user: Option<User>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PhotoSize {
    pub file_id: String,
    pub width: i64,
    pub height: i64,
    pub file_size: Option<i64>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Audio {
    pub file_id: String,
    pub duration: i64,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
    pub thumb: Option<PhotoSize>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Document {
    pub file_id: String,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Video {
    pub file_id: String,
    pub width: i64,
    pub height: i64,
    pub duration: i64,
    pub thumb: Option<PhotoSize>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}
#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Animation {
    pub file_id: String,
    pub width: i64,
    pub height: i64,
    pub duration: i64,
    pub thumb: Option<PhotoSize>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Voice {
    pub file_id: String,
    pub duration: i64,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct VideoNote {
    pub file_id: String,
    pub length: i64,
    pub duration: i64,
    pub thumb: Option<PhotoSize>,
    pub file_size: Option<i64>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub user_id: Option<i64>,
    pub vcard: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Venue {
    pub location: Location,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub foursquare_type: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct UserProfilePhotos {
    pub total_count: i64,
    pub photos: Vec<Vec<PhotoSize>>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct File {
    pub file_id: String,
    pub file_size: Option<i64>,
    pub file_path: Option<String>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<KeyboardButton>>,
    pub resize_keyboard: Option<bool>,
    pub one_time_keyboard: Option<bool>,
    pub selective: Option<bool>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct KeyboardButton {
    pub text: String,
    pub request_contact: Option<bool>,
    pub request_location: Option<bool>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct ReplyKeyboardRemove {
    pub remove_keyboard: bool,
    pub selective: Option<bool>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct InlineKeyboardButton {
    pub text: String,
    pub url: Option<String>,
    pub callback_data: Option<String>,
    pub switch_inline_query: Option<String>,
    pub switch_inline_query_current_chat: Option<String>,
    pub callback_game: Option<CallbackGame>,
    pub pay: Option<bool>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<Message>,
    pub inline_message_id: Option<String>,
    pub chat_instance: String,
    pub data: Option<String>,
    pub game_short_name: Option<String>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct ForceReply {
    pub force_reply: bool,
    pub selective: Option<bool>,
}

// TODO: Manual serialization or look up how to choose the right `type` value
// based on the variant.
#[derive(Debug, PartialEq, Clone)]
pub enum InputMedia {
    Photo {
        media: String,
        caption: Option<String>,
        parse_mode: Option<ParseMode>,
    },
    Video {
        media: String,
        thumb: Option<String>,
        caption: Option<String>,
        parse_mode: Option<ParseMode>,
        width: Option<i64>,
        height: Option<i64>,
        duration: Option<i64>,
        supports_streaming: Option<bool>,
    },
    Animation {
        media: String,
        thumb: Option<String>,
        caption: Option<String>,
        parse_mode: Option<ParseMode>,
        width: Option<i64>,
        height: Option<i64>,
        duration: Option<i64>,
    },
    Audio {
        media: String,
        thumb: Option<String>,
        caption: Option<String>,
        parse_mode: Option<ParseMode>,
        duration: Option<i64>,
        performer: Option<String>,
        title: Option<String>,
    },
    Document {
        media: String,
        thumb: Option<String>,
        caption: Option<String>,
        parse_mode: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Sticker {
    pub file_id: String,
    pub width: i64,
    pub heigth: i64,
    pub thumb: Option<PhotoSize>,
    pub emoji: Option<String>,
    pub set_name: Option<String>,
    pub mask_position: Option<MaskPosition>,
    pub file_size: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MaskPositionPoint {
    Forehead,
    Eyes,
    Mouth,
    Chin,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MaskPosition {
    pub point: MaskPositionPoint,
    pub x_shift: f64,
    pub y_shift: f64,
    pub scale: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct StickerSet {
    pub name: String,
    pub title: String,
    pub contains_masks: bool,
    pub stickers: Vec<Sticker>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct LabeledPrice {
    pub label: String,
    pub amount: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Invoice {
    pub title: String,
    pub description: String,
    pub start_parameter: String,
    pub currency: String,
    pub total_amount: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ShippingAddress {
    pub country_code: String,
    pub state: String,
    pub city: String,
    pub street_line1: String,
    pub street_line2: String,
    pub post_code: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct OrderInfo {
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub shipping_address: Option<ShippingAddress>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ShippingOption {
    pub id: String,
    pub title: String,
    pub prices: Vec<LabeledPrice>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct SuccessfulPayment {
    pub currency: String,
    pub total_amount: u64,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<OrderInfo>,
    pub telegram_payment_charge_id: String,
    pub provider_payment_charge_id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ShippingQuery {
    pub id: String,
    pub from: User,
    pub invoice_payload: String,
    pub shipping_address: ShippingAddress,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PreCheckoutQuery {
    pub id: String,
    pub from: User,
    pub currency: String,
    pub total_amount: u64,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<OrderInfo>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,
    pub credentails: EncryptedCredentails,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct PassportFile {
    pub file_id: String,
    pub file_size: u64,
    pub file_date: u64,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
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

#[derive(Deserialize, Debug, PartialEq, Clone)]
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

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct EncryptedCredentails {
    pub data: String,
    pub hash: String,
    pub secret: String,
}

//manual Deserialize source
#[derive(Deserialize, Debug, PartialEq, Clone)]
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

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Game {
    pub title: String,
    pub description: String,
    pub photo: Vec<PhotoSize>,
    pub text: Option<String>,
    pub text_entities: Option<Vec<MessageEntity>>,
    pub animation: Option<Animation>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct GameHighScore {
    pub position: u64,
    pub user: User,
    pub score: u64,
}

pub enum Keyboard<'o, 'i: 'o, 'b: 'i> {
    Inline(InlineKeyboard<'o, 'i, 'b>),
    ReplyMarkup(ReplyKeyboard<'o, 'i, 'b>),
    ReplyRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}

impl<'o, 'i: 'o, 'b: 'i> serde::Serialize for Keyboard<'o, 'i, 'b> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Keyboard::Inline(keyboard) => keyboard.serialize(s),
            Keyboard::ReplyMarkup(keyboard) => keyboard.serialize(s),
            Keyboard::ReplyRemove(keyboard) => keyboard.serialize(s),
            Keyboard::ForceReply(keyboard) => keyboard.serialize(s),
        }
    }
}

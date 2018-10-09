
#[derive(Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum ChatTypes {
    Private,
    Group,
    Supergroup,
    Channel,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct User {
    id: i64,
    is_bot: bool,
    first_name: String,
    last_name: Option<String>,
    username: Option<String>,
    language_code: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct ChatMember {
    user: User,
    status: String,
    until_date: Option<i64>,
    can_be_edited: Option<bool>,
    can_change_info: Option<bool>,
    can_post_messages: Option<bool>,
    can_edit_messages: Option<bool>,
    can_delete_messages: Option<bool>,
    can_invite_users: Option<bool>,
    can_restrict_members: Option<bool>,
    can_pin_messages: Option<bool>,
    can_promote_members: Option<bool>,
    can_send_messages: Option<bool>,
    can_send_media_messages: Option<bool>,
    can_send_other_messages: Option<bool>,
    can_add_web_page_previews: Option<bool>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct ChatPhoto {
    small_file_id: String,
    big_file_id: String,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Chat {
    id: i64,
    // from `type`
    chat_type: ChatTypes,
    title: Option<String>,
    username: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    all_members_are_administrators: Option<bool>,
    photo: Option<ChatPhoto>,
    description: Option<String>,
    invite_link: Option<String>,
    pinned_message: Option<Box<Message>>,
    sticker_set_name: Option<String>,
    can_set_sticker_set: Option<bool>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Message {
    message_id: i64,
    from: User,
    date: i64,
    chat: Chat,
    forward_from: Option<User>,
    forward_from_chat: Option<Chat>,
    forward_from_message_id: Option<i64>,
    forward_signature: Option<String>,
    forward_date: Option<i64>,
    reply_to_message: Option<Box<Message>>,
    edit_date: Option<i64>,
    media_group_id: Option<String>,
    author_signature: Option<String>,
    text: Option<String>,
    entities: Option<Vec<MessageEntity>>,
    caption_entities: Option<Vec<MessageEntity>>,
    audio: Option<Audio>,
    document: Option<Document>,
    animation: Option<Animation>,
    game: Option<Game>,
    photo: Option<Vec<PhotoSize>>,
    sticker: Option<Sticker>,
    video: Option<Video>,
    voice: Option<Voice>,
    video_note: Option<VideoNote>,
    caption: Option<String>,
    contact: Option<Contact>,
    location: Option<Location>,
    venue: Option<Venue>,
    new_chat_members: Option<Vec<User>>,
    left_chat_member: Option<User>,
    new_chat_title: Option<String>,
    new_chat_photo: Option<Vec<PhotoSize>>,
    delete_chat_photo: Option<bool>,
    group_chat_created: Option<bool>,
    supergroup_chat_created: Option<bool>,
    channel_chat_created: Option<bool>,
    migrate_to_chat_id: Option<i64>,
    migrate_from_chat_id: Option<i64>,
    pinned_message: Option<Box<Message>>,
    invoice: Option<Invoice>,
    successful_payment: Option<SuccessfulPayment>,
    conneted_website: Option<String>,
    passport_data: Option<PassportData>,
}

#[derive(Deserialize, Debug, PartialEq, Clone, Copy)]
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
    // from `type`
    entity_type: MessageEntityTypes,
    offset: i64,
    length: i64,
    url: Option<String>,
    user: Option<User>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PhotoSize {
    file_id: String,
    width: i64,
    height: i64,
    file_size: Option<i64>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Audio {
    file_id: String,
    duration: i64,
    performer: Option<String>,
    title: Option<String>,
    mime_type: Option<String>,
    file_size: Option<i64>,
    thumb: Option<PhotoSize>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Document {
    file_id: String,
    thumb: Option<PhotoSize>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<i64>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Video {
    file_id: String,
    width: i64,
    height: i64,
    duration: i64,
    thumb: Option<PhotoSize>,
    mime_type: Option<String>,
    file_size: Option<i64>
}
#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Animation {
    file_id: String,
    width: i64,
    height: i64,
    duration: i64,
    thumb: Option<PhotoSize>,
    mime_type: Option<String>,
    file_size: Option<i64>
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Voice {
    file_id: String,
    duration: i64,
    mime_type: Option<String>,
    file_size: Option<i64>
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct VideoNote {
    file_id: String,
    length: i64,
    duration: i64,
    thumb: Option<PhotoSize>,
    file_size: Option<i64>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Contact {
    phone_number: String,
    first_name: String,
    last_name: Option<String>,
    user_id: Option<i64>,
    vcard: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Location {
    longitude: f64,
    latitude: f64,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Venue {
    location: Location,
    title: String,
    address: String,
    foursquare_id: Option<String>,
    foursquare_type: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct UserProfilePhotos {
    total_count: i64,
    photos: Vec<Vec<PhotoSize>>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct File {
    file_id: String,
    file_size: Option<i64>,
    file_path: Option<String>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct ReplyKeyboardMarkup {
    keyboard: Vec<Vec<KeyboardButton>>,
    resize_keyboard: Option<bool>,
    one_time_keyboard: Option<bool>,
    selective: Option<bool>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct KeyboardButton {
    text: String,
    request_contact: Option<bool>,
    request_location: Option<bool>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct ReplyKeyboardRemove {
    remove_keyboard: bool,
    selective: Option<bool>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct InlineKeyboardMarkup {
    inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct InlineKeyboardButton {
    text: String,
    url: Option<String>,
    callback_data: Option<String>,
    switch_inline_query: Option<String>,
    switch_inline_query_current_chat: Option<String>,
    callback_game: Option<CallbackGame>,
    pay: Option<bool>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct CallbackQuery {
    id: String,
    from: User,
    message: Option<Message>,
    inline_message_id: Option<String>,
    chat_instance: String,
    data: Option<String>,
    game_short_name: Option<String>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct ForceReply {
    force_reply: bool,
    selective: Option<bool>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct ResponseParameters {
    migrate_to_chat_id: Option<i64>,
    retry_after: Option<i64>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum ParseModes {
    Markdown,
    // must serialized to `HTML`
    Html,
}

// TODO: Manual serialization or look up how to choose the right `type` value
// based on the variant.
#[derive(Debug, PartialEq, Clone)]
pub enum InputMedia {
    Photo {
        media: String,
        caption: Option<String>,
        parse_mode: Option<ParseModes>,
    },
    Video {
        media: String,
        thumb: Option<String>,
        caption: Option<String>,
        parse_mode: Option<ParseModes>,
        width: Option<i64>,
        height: Option<i64>,
        duration: Option<i64>,
        supports_streaming: Option<bool>,
    },
    Animation {
        media: String,
        thumb: Option<String>,
        caption: Option<String>,
        parse_mode: Option<ParseModes>,
        width: Option<i64>,
        height: Option<i64>,
        duration: Option<i64>,
    },
    Audio {
        media: String,
        thumb: Option<String>,
        caption: Option<String>,
        parse_mode: Option<ParseModes>,
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
    file_id: String,
    width: i64,
    heigth: i64,
    thumb: Option<PhotoSize>,
    emoji: Option<String>,
    set_name: Option<String>,
    mask_position: Option<MaskPosition>,
    file_size: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum MaskPositionPoint {
    Forehead,
    Eyes,
    Mouth,
    Chin,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MaskPosition {
    point: MaskPositionPoint,
    x_shift: f64,
    y_shift: f64,
    scale: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct StickerSet {
    name: String,
    title: String,
    contains_masks: bool,
    stickers: Vec<Sticker>,
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct LabeledPrice {
    label: String,
    amount: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Invoice {
    title: String,
    description: String,
    start_parameter: String,
    currency: String, //TODO: Maybe change to CurrencyCode later
    total_amount: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ShippingAddress {
    country_code: String,
    state: String,
    city: String,
    street_line1: String,
    street_line2: String,
    post_code: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct OrderInfo {
    name: Option<String>,
    phone_number: Option<String>,
    email: Option<String>,
    shipping_address: Option<ShippingAddress>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ShippingOption {
    id: String,
    title: String,
    prices: Vec<LabeledPrice>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct SuccessfulPayment {
    currency: String,
    total_amount: u64,
    invoice_payload: String,
    shipping_option_id: Option<String>,
    order_info: Option<OrderInfo>,
    telegram_payment_charge_id: String,
    provider_payment_charge_id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ShippingQuery {
    id: String,
    from: User,
    invoice_payload: String,
    shipping_address: ShippingAddress,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PreCheckoutQuery {
    id: String,
    from: User,
    currency: String,
    total_amount: u64,
    invoice_payload: String,
    shipping_option_id: Option<String>,
    order_info: Option<OrderInfo>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct PassportData {
    data: Vec<EncryptedPassportElement>,
    credentails: EncryptedCredentails,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct PassportFile {
    file_id: String,
    file_size: u64,
    file_date: u64,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
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
    element_type: EncryptedPassportElementType,
    data: Option<String>,
    phone_number: Option<String>,
    email: Option<String>,
    files: Option<Vec<PassportFile>>,
    front_side: Option<PassportFile>,
    reverse_side: Option<PassportFile>,
    selfie: Option<PassportFile>,
    translation: Option<Vec<PassportFile>>,
    hash: String,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct EncryptedCredentails {
    data: String,
    hash: String,
    secret: String,
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

/// A placeholder, currently holds no information, according to the [API docs].
/// [API docs]: https://core.telegram.org/bots/api#callbackgame
#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct CallbackGame;

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Game {
    title: String,
    description: String,
    photo: Vec<PhotoSize>,
    text: Option<String>,
    text_entities: Option<Vec<MessageEntity>>,
    animation: Option<Animation>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct GameHighScore {
    position: u64,
    user: User,
    score: u64,
}

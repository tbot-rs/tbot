//! Types for interacting with the API.

mod animation;
mod audio;
mod bot_command;
pub mod callback;
pub mod chat;
mod chosen_inline_result;
mod contact;
pub mod dice;
mod document;
pub mod file;
pub mod game;
pub mod inline_message_id;
pub mod inline_query;
pub mod input_file;
pub mod input_message_content;
mod invoice;
pub mod keyboard;
mod labeled_price;
mod location;
mod login_url;
pub mod message;
mod order_info;
pub mod parameters;
pub mod passport;
mod photo_size;
pub mod poll;
pub mod pre_checkout_query;
pub mod shipping;
pub mod sticker;
mod successful_payment;
pub mod update;
pub mod user;
mod venue;
mod video;
mod video_note;
mod voice;
pub mod webhook_info;

pub use {
    animation::Animation, audio::Audio, bot_command::BotCommand, chat::Chat,
    chosen_inline_result::ChosenInlineResult, contact::Contact, dice::Dice,
    document::Document, file::File, game::Game,
    inline_message_id::InlineMessageId, inline_query::InlineQuery,
    input_message_content::InputMessageContent, invoice::Invoice,
    labeled_price::LabeledPrice, location::Location, login_url::LoginUrl,
    message::Message, order_info::OrderInfo, photo_size::PhotoSize, poll::Poll,
    pre_checkout_query::PreCheckoutQuery, sticker::Sticker,
    successful_payment::SuccessfulPayment, update::Update, user::User,
    venue::Venue, video::Video, video_note::VideoNote, voice::Voice,
    webhook_info::WebhookInfo,
};

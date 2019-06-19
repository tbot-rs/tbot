//! Types for interacting with the API.

use super::*;

mod animation;
mod audio;
pub mod callback;
pub mod chat;
mod chosen_inline_result;
mod contact;
mod document;
mod file;
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
mod photo_size;
pub mod poll;
pub mod pre_checkout_query;
pub mod raw;
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
    animation::*, audio::*, chat::Chat, chosen_inline_result::*, contact::*,
    document::*, file::*, game::Game, inline_message_id::InlineMessageId,
    inline_query::InlineQuery, input_message_content::InputMessageContent,
    invoice::*, labeled_price::*, location::*, login_url::*, message::Message,
    order_info::*, photo_size::*, poll::Poll,
    pre_checkout_query::PreCheckoutQuery, sticker::Sticker,
    successful_payment::*, update::Update, user::User, venue::*, video::*,
    video_note::*, voice::*, webhook_info::WebhookInfo,
};

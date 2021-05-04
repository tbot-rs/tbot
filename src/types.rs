//! Types for interacting with the API.

#![allow(clippy::wrong_self_convention)]

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
pub mod location;
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

pub use animation::Animation;
pub use audio::Audio;
pub use bot_command::BotCommand;
pub use chat::Chat;
pub use chosen_inline_result::ChosenInlineResult;
pub use contact::Contact;
pub use dice::Dice;
pub use document::Document;
pub use file::File;
pub use game::Game;
pub use inline_message_id::InlineMessageId;
pub use inline_query::InlineQuery;
pub use input_message_content::InputMessageContent;
pub use invoice::Invoice;
pub use labeled_price::LabeledPrice;
pub use location::Location;
pub use login_url::LoginUrl;
pub use message::Message;
pub use order_info::OrderInfo;
pub use photo_size::PhotoSize;
pub use poll::Poll;
pub use pre_checkout_query::PreCheckoutQuery;
pub use sticker::Sticker;
pub use successful_payment::SuccessfulPayment;
pub use update::Update;
pub use user::User;
pub use venue::Venue;
pub use video::Video;
pub use video_note::VideoNote;
pub use voice::Voice;
pub use webhook_info::WebhookInfo;

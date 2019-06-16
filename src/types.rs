//! Types used when interacting with the API.

// we re-export everything under one namespace
#![allow(clippy::module_name_repetitions)]

use super::*;

pub mod input_file;
pub mod input_message_content;
pub mod keyboard;
pub mod raw;

mod animation;
mod audio;
pub mod callback;
pub mod chat;
mod chosen_inline_result;
mod contact;
mod document;
mod file;
mod game;
mod game_high_score;
pub mod inline_query;
mod invoice;
mod location;
mod login_url;
pub mod message;
pub mod parameters;
mod photo_size;
mod poll;
pub mod sticker;
mod updates;
pub mod user;
mod venue;
mod video;
mod video_note;
mod voice;
mod webhook_info;

pub use {
    animation::*, audio::*, chat::Chat, chosen_inline_result::*, contact::*,
    document::*, file::*, game::*, game_high_score::*,
    inline_query::InlineQuery, input_message_content::InputMessageContent,
    invoice::*, location::*, login_url::*, message::Message, photo_size::*,
    poll::*, sticker::Sticker, updates::*, user::User, venue::*, video::*,
    video_note::*, voice::*, webhook_info::*,
};

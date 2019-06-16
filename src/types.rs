//! Types used when interacting with the API.

// we re-export everything under one namespace
#![allow(clippy::module_name_repetitions)]

use super::*;

pub mod inline_query_result;
pub mod input_file;
pub mod input_message_content;
pub mod keyboard;
pub mod raw;

mod animation;
mod audio;
pub mod callback;
pub mod chat;
mod chat_id;
mod chosen_inline_result;
mod contact;
mod document;
mod file;
mod game;
mod game_high_score;
mod inline_query;
mod invoice;
mod location;
mod login_url;
pub mod message;
mod parse_mode;
mod photo_size;
mod poll;
pub mod sticker;
mod updates;
mod user;
mod user_profile_photos;
mod venue;
mod video;
mod video_note;
mod voice;
mod webhook_info;

pub use {
    animation::*, audio::*, chat::Chat, chat_id::*, chosen_inline_result::*,
    contact::*, document::*, file::*, game::*, game_high_score::*,
    inline_query::*, inline_query_result::InlineQueryResult,
    input_message_content::InputMessageContent, invoice::*, location::*,
    login_url::*, message::Message, parse_mode::*, photo_size::*, poll::*,
    sticker::Sticker, updates::*, user::*, user_profile_photos::*, venue::*,
    video::*, video_note::*, voice::*, webhook_info::*,
};

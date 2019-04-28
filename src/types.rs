//! Contains types used in Telegram Bots API.

// we re-export everything under one namespace
#![allow(clippy::module_name_repetitions)]

use super::*;

pub mod input_file;
pub mod raw;

mod animation;
mod audio;
mod callback_game;
mod chat_action;
mod chat_id;
mod chat_photo;
mod chat_types;
mod contact;
mod document;
mod file;
mod game;
mod invoice;
mod keyboards;
mod location;
mod mask_position;
mod message;
mod message_entity;
mod parse_mode;
mod photo_size;
mod poll;
mod sticker;
mod sticker_set;
mod updates;
mod user;
mod user_profile_photos;
mod venue;
mod video;
mod video_note;
mod voice;
mod webhook_info;

pub use {
    animation::*, audio::*, callback_game::*, chat_action::*, chat_id::*,
    chat_photo::*, chat_types::*, contact::*, document::*, file::*, game::*,
    invoice::*, keyboards::*, location::*, mask_position::*, message::*,
    message_entity::*, parse_mode::*, photo_size::*, poll::*, sticker::*,
    sticker_set::*, updates::*, user::*, user_profile_photos::*, venue::*,
    video::*, video_note::*, voice::*, webhook_info::*,
};

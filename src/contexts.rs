//! This module contants contexts that are passed to handlers.

#![allow(clippy::too_many_arguments)] // can't do much
                                      // we re-export everything under one namespace
#![allow(clippy::module_name_repetitions)]

use super::*;

#[macro_use]
mod macros;

mod animation;
mod audio;
mod contact;
mod document;
mod edited_animation;
mod edited_audio;
mod edited_document;
mod edited_location;
mod edited_photo;
mod edited_text;
mod edited_video;
mod game;
mod left_member;
mod location;
mod new_chat_title;
mod new_members;
mod photo;
mod poll;
mod sticker;
mod text;
mod unhandled;
mod update;
mod updated_poll;
mod venue;
mod video;
mod video_note;
mod voice;

pub mod traits;

pub use {
    animation::*, audio::*, contact::*, document::*, edited_animation::*,
    edited_audio::*, edited_document::*, edited_location::*, edited_photo::*,
    edited_text::*, edited_video::*, game::*, left_member::*, location::*, new_chat_title::*,
    new_members::*, photo::*, poll::*, sticker::*, text::*, unhandled::*,
    update::*, updated_poll::*, venue::*, video::*, video_note::*, voice::*,
};

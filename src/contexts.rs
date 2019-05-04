//! This module contants contexts that are passed to handlers.

#![allow(clippy::too_many_arguments)] // can't do much
                                      // we re-export everything under one namespace
#![allow(clippy::module_name_repetitions)]

use super::*;

#[macro_use]
mod macros;

mod animation;
mod audio;
mod document;
mod edited_text;
mod game;
mod photo;
mod poll;
mod sticker;
mod text;
mod unhandled;
mod update;
mod updated_poll;
mod video;
mod voice;

pub mod traits;

pub use {
    animation::*, audio::*, document::*, edited_text::*, game::*, photo::*,
    poll::*, sticker::*, text::*, unhandled::*, update::*, updated_poll::*,
    video::*, voice::*,
};

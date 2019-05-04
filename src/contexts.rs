//! This module contants contexts that are passed to handlers.

#![allow(clippy::too_many_arguments)] // can't do much
                                      // we re-export everything under one namespace
#![allow(clippy::module_name_repetitions)]

use super::*;

#[macro_use]
mod macros;

mod animation;
mod audio;
mod edited_text;
mod photo;
mod poll;
mod text;
mod unhandled;
mod update;
mod updated_poll;
mod video;
mod voice;

pub mod traits;

pub use {
    animation::*, audio::*, edited_text::*, photo::*, poll::*, text::*,
    unhandled::*, update::*, updated_poll::*, video::*, voice::*,
};

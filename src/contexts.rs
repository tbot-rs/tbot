//! This module contants contexts that are passed to handlers.

#![allow(clippy::too_many_arguments)] // can't do much
                                      // we re-export everything under one namespace
#![allow(clippy::module_name_repetitions)]

use super::*;

mod edited_text;
mod poll;
mod text;
mod update;
mod updated_poll;

pub mod traits;

pub use {edited_text::*, poll::*, text::*, update::*, updated_poll::*};

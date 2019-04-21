//! This module contants contexts that are passed to handlers.

use super::*;

mod edited_text;
mod text;
mod update;

pub mod traits;

pub use {edited_text::*, text::*, update::*};

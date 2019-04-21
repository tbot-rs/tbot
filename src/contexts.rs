//! This module contants contexts that are passed to handlers.

use super::*;

mod text;
mod update;

pub mod traits;

pub use {text::*, update::*};

//! Contains traits for different types of updates to simplify calling several
//! methods.

use super::*;
use crate::methods::*;

mod chat_methods;
mod forwardable;
mod pinnable;

pub use {chat_methods::*, forwardable::*, pinnable::*};

//! This module contains several traits for different types of events
//! to simplify calling some methods.

use super::*;
use crate::methods::*;

mod chat_methods;
mod forwardable;

pub use {chat_methods::*, forwardable::*};
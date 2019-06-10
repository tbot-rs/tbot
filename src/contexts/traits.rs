//! Contains traits for updates that infer some data from them, simplifying
//! calling several methods.

use super::*;
use crate::methods::*;

mod chat_methods;
mod forwardable;
mod pinnable;

pub use {chat_methods::*, forwardable::*, pinnable::*};

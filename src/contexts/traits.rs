//! Contains traits for updates that infer some data from them, simplifying
//! calling several methods.

use crate::methods::*;

mod callback;
mod chat_methods;
mod forwardable;
mod pinnable;

pub use {callback::*, chat_methods::*, forwardable::*, pinnable::*};

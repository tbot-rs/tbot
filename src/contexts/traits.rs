//! Contains traits for updates that infer some data from them, simplifying
//! calling several methods.

use super::*;
use crate::methods::*;

mod callback;
mod chat_methods;
mod forwardable;
mod inline;
mod pinnable;

pub use {
    callback::*, chat_methods::*, forwardable::*, inline::*, pinnable::*,
};

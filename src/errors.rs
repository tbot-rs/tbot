//! Types representing errors.

mod download;
mod method_call;
mod webhook;

pub use {download::*, method_call::*, webhook::*};

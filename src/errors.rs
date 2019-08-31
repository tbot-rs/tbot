//! Types representing errors.

mod download;
mod http_webhook;
mod method_call;

pub use {download::*, http_webhook::*, method_call::*};

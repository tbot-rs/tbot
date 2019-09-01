//! Types representing errors.

mod download;
mod http_webhook;
mod https_webhook;
mod method_call;

pub use {download::*, http_webhook::*, https_webhook::*, method_call::*};

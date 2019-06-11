//! Make cool Telegram bots with Rust easily. Here is a simple echo bot:
//!
//! ```no_run
//! use tbot::prelude::*;
//!
//! let mut bot = tbot::bot!("BOT_TOKEN");
//!
//! bot.text(|context| {
//!     let reply = context
//!         .send_message(&context.text.value)
//!         .into_future()
//!         .map_err(|error| {
//!             dbg!(error);
//!         });
//!
//!     tbot::spawn(reply);
//! });
//!
//! bot.polling().start();
//! ```
//!
//! If you're new to `tbot`, we recommend you go through the [tutorial] first.
//! We also have several [How-to guides][how-to] with snippets to solve your
//! problems.
//!
//! If you have a question, ask it in [our group] on Telegram. If you find
//! a bug, fill an issue on either our [GitLab] or [GitHub] repository.
//!
//! [our group]: t.me/tbot_group
//! [tutorial]: https://gitlab.com/SnejUgal/tbot/wikis/Tutorial
//! [how-to]: https://gitlab.com/SnejUgal/tbot/wikis/How-to
//! [GitLab]: https://gitlab.com/SnejUgal/tbot
//! [GitHub]: https://github.com/SnejUgal/tbot

#![deny(
    future_incompatible,
    nonstandard_style,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(clippy::multiple_crate_versions)] // can't do much

mod bot;
mod internal;
mod multipart;

pub mod contexts;
pub mod methods;
pub mod types;

pub use bot::*;
use serde::{Deserialize, Serialize};
use {internal::*, multipart::*, prelude::*};

#[cfg(feature = "proxy")]
pub use hyper_proxy as proxy;

/// A wrapper around `tokio::run` without `F::Item: ()`.
///
/// When calling an API method, you'll most likely throw away its result.
/// However, `tokio` requires that `F::Item` be `()`. `tbot` provides
/// a thin wrapper around `tokio::run` that maps `F::Item` to `()`.
/// On the other hand, `tbot` still requires that you handle possible errors
/// properly before running a future.
pub fn run<F>(future: F)
where
    F: futures::Future<Error = ()> + Send + 'static,
{
    tokio::run(future.map(|_| ()));
}

/// A wrapper around `tokio::spawn` without `F::Item: ()`.
///
/// When calling an API method, you'll most likely throw away its result.
/// However, `tokio` requires that `F::Item` be `()`. `tbot` provides
/// a thin wrapper around `tokio::spawn` that maps `F::Item` to `()`.
/// On the other hand, `tbot` still requires that you handle possible errors
/// properly before running a future.
pub fn spawn<F>(future: F) -> tokio::executor::Spawn
where
    F: futures::Future<Error = ()> + Send + 'static,
{
    tokio::spawn(future.map(|_| ()))
}

pub mod prelude {
    //! Traits needed when working with `tbot`.
    pub use super::{contexts::traits::*, methods::Methods};
    pub use futures::Future;
    pub use futures::IntoFuture;
}

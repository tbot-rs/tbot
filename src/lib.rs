//! Make cool Telegram bots with Rust easily. Here is a simple echo bot:
//!
//! ```no_run
//! use tbot::{prelude::*, Bot};
//!
//! let mut bot = Bot::from_env("BOT_TOKEN");
//!
//! bot.on_message(|context| {
//!     let reply = context
//!         .send_message(&context.message)
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
//! If you have a question, ask it in [our group] on Telegram. If you find
//! a bug, fill an issue on either our [GitLab] or [GitHub] repository.
//!
//! If you get stuck or find a bug, fill an issue on either our [GitLab] or
//! [GitHub] repository.
//!
//! [our group]: t.me/tbot_group
//! [tutorial]: https://gitlab.com/SnejUgal/tbot/wikis/Tutorial
//! [how-to]: https://gitlab.com/SnejUgal/tbot/wikis/How-to
//! [GitLab]: https://gitlab.com/SnejUgal/tbot
//! [GitHub]: https://github.com/SnejUgal/tbot

#![deny(future_incompatible)]
#![deny(nonstandard_style)]
#![deny(missing_docs)]

mod bot;
mod multipart;

pub mod contexts;
pub mod methods;
pub mod types;

pub use bot::*;
use serde::{Deserialize, Serialize};
use {multipart::*, prelude::*};

#[cfg(feature = "proxy")]
pub use hyper_proxy as proxy;

/// Like `tokio::run`, but doesn't require `future::Item` to be `()`.
///
/// Most use-caces of `tbot` do not need to use the future's `Item` value,
/// leading to many `.map(|_| ())` in the code. This function will implicitly
/// map `Item` to `()`. Note that it does **not** map `Error` to `()`, because
/// error handling must be done on your own.
pub fn run<F>(future: F)
where
    F: futures::Future<Error = ()> + Send + 'static,
{
    tokio::run(future.map(|_| ()));
}

/// Like `tokio::spawn`, but doesn't require `future::Item` to be `()`.
///
/// Most use-caces of `tbot` do not need to use the future's `Item` value,
/// leading to many `.map(|_| ())` in the code. This function will implicitly
/// map `Item` to `()`. Note that it does **not** map `Error` to `()`, because
/// error handling must be done on your own.
pub fn spawn<F>(future: F) -> tokio::executor::Spawn
where
    F: futures::Future<Error = ()> + Send + 'static,
{
    tokio::spawn(future.map(|_| ()))
}

pub mod prelude {
    //! Re-exports some traits the compiler may demand when working with `tbot`.
    pub use super::{contexts::traits::*, methods::Methods};
    pub use futures::Future;
}

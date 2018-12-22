//! A crate for easy working with Telegram Bots API. `tbot`'s API is simple,
//! but lets you do many things. Here's a simple echo bot:
//!
//! ```ignore (would timeout)
//! use tbot::{prelude::*, Bot};
//!
//! let mut bot = Bot::from_env("BOT_TOKEN");
//!
//! bot.on_message(|context| {
//!     let reply = context
//!         .send_message(context.message)
//!         .into_future()
//!         .map_err(|err| eprintln!("Couldn't send a message: {:#?}", err));
//!
//!     tbot::spawn(reply);
//! });
//!
//! bot.start_polling();
//! ```
//!
//! If you're a newcomer to `tbot`, we recommend you doing the [tutorial] first.
//! We also have several how-to's for you to see how to implement different
//! features using `tbot`. If you face a problem, feel free to fill an issue on
//! [our Gitlab repository][gitlab].
//!
//! [tutorial]: https://gitlab.com/SnejUgal/tbot/wikis/Tutorial
//! [gitlab]: https://gitlab.com/SnejUgal/tbot

#![deny(future_incompatible)]
#![deny(nonstandard_style)]
#![deny(missing_docs)]

mod bot;

pub mod methods;
pub mod types;

pub use self::bot::*;
use self::prelude::*;
use serde::{Deserialize, Serialize};

/// Like re-exported `tokio::run`, but doesn't require `future::Item` to be
/// `()`.
///
/// Most use-caces of `tbot` do not need to use the future's `Item` value,
/// leading to many `.map(|_| ())` in the code. This code will implicitly map
/// `Item` to `()`. Note that it does **not** map `Error` to `()`, because
/// error handling must be done on your own.
pub fn run<F>(future: F)
where
    F: futures::Future<Error = ()> + Send + 'static,
{
    tokio::run(future.map(|_| ()));
}

/// Like re-exported `tokio::spawn`, but doesn't require `future::Item` to be
/// `()`.
///
/// Most use-caces of `tbot` do not need to use the future's `Item` value,
/// leading to many `.map(|_| ())` in the code. This code will implicitly map
/// `Item` to `()`. Note that it does **not** map `Error` to `()`, because
/// error handling must be done on your own.
pub fn spawn<F>(future: F) -> tokio::executor::Spawn
where
    F: futures::Future<Error = ()> + Send + 'static,
{
    tokio::spawn(future.map(|_| ()))
}

pub mod prelude {
    //! Re-exports some traits the compile may demand when working with `tbot`.
    pub use futures::Future;
}

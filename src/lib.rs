//! Make cool Telegram bots with Rust easily. Here is a simple echo bot:
//!
//! ```no_run
//! use tbot::prelude::*;
//!
//! # /*
//! #[tokio::main]
//! async fn main() {
//! # */
//! # // is there a way to enable `tokio/macros` for examples?
//! # async fn bot() {
//!     let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();
//!
//!     bot.text(|context| async move {
//!         let echo = &context.text.value;
//!         let call_result = context.send_message(echo).call().await;
//!
//!         if let Err(err) = call_result {
//!             dbg!(err);
//!         }
//!     });
//!
//!     bot.polling().start().await.unwrap();
//! }
//! ```
//!
//! There are many [examples] to see `tbot` in action. If you want to see
//! real-world use of `tbot`, check out [this list][projects].
//!
//! If you're a newcomer, we recommend you go through the [tutorial] first.
//! We also have several [How-to guides][how-to] to help you use `tbot`.
//! You can always refer to our API docs on [_docs.rs_][api-docs]
//! (also, docs for `master` are available [here][master-docs]).
//!
//! If you have a question, ask it in [our group] on Telegram. If you find
//! a bug, file an issue on either our [GitLab] or [GitHub] repository.
//!
//! [examples]: https://gitlab.com/SnejUgal/tbot/-/tree/master/examples
//! [projects]: https://gitlab.com/SnejUgal/tbot/-/wikis/Projects-built-with-tbot
//!
//! [tutorial]: https://gitlab.com/SnejUgal/tbot/wikis/Tutorial
//! [how-to]: https://gitlab.com/SnejUgal/tbot/wikis/How-to
//! [api-docs]: https://docs.rs/tbot
//! [master-docs]: https://docs.tbot.rs
//!
//! [our group]: https://t.me/tbot_group
//! [gitlab]: https://gitlab.com/SnejUgal/tbot
//! [github]: https://github.com/tbot-rs/tbot

#![deny(
    future_incompatible,
    nonstandard_style,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
// It's just not worth adding an `# Errors` section because the returned
// errors clearly describe why something may fail
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::multiple_crate_versions)] // can't do much
// that's where you're wrong, kiddo
#![allow(clippy::needless_doctest_main)]
#![doc(
    html_logo_url = "https://gitlab.com/SnejUgal/tbot/-/raw/master/logo.svg",
    html_favicon_url = "https://gitlab.com/SnejUgal/tbot/-/raw/master/logo.svg"
)]

#[cfg(all(feature = "tls", feature = "rustls"))]
compile_error!("`tls` and `rustls` features are mutually exclusive. You should enable only one of them");

#[cfg(not(any(feature = "tls", feature = "rustls")))]
compile_error!("Either `tls` or `rustls` feature needs to be enabled");

pub mod bot;
mod download_file;
mod internal;
mod multipart;
mod token;

pub mod compositors;
mod connectors;
pub mod contexts;
pub mod errors;
pub mod event_loop;
pub mod markup;
pub mod methods;
pub mod predicates;
pub mod proxy;
pub mod state;
pub mod types;
pub mod util;

use {download_file::download_file, multipart::Multipart};

pub use {bot::Bot, event_loop::EventLoop};

pub mod prelude {
    //! Traits needed when working with `tbot`.
    pub use super::contexts::methods::Callback as _;
    pub use super::contexts::methods::Copyable as _;
    pub use super::contexts::methods::Forwardable as _;
    pub use super::contexts::methods::Message as _;
    pub use super::contexts::methods::Pinnable as _;
    pub use super::util::ChatActionLoop as _;
    pub use super::util::ChatActionLoopBotExt as _;
}

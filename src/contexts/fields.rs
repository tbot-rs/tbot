//! Traits for common context fields.
//!
//! Suppose that you want to process users' photos whenever they send or edit
//! one. You would like to abstract this as much as possible, like this:
//!
//! ```
//! # fn process_photo<T>(_: T) { }
//! let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();
//! bot.photo(process_photo);
//! bot.edited_photo(process_photo);
//! ```
//!
//! However, in the first case we have [`contexts::Photo`], but in the second
//! one we have [`contexts::EditedPhoto`]. Luckily, they both implement
//! [`fields::Photo`], which allows accessing the `photo` field without caring
//! about the exact update type. So, if you already have this handler:
//!
//! ```
//! use tbot::{contexts, connectors::Https};
//! fn process_photo(context: &contexts::Photo<Https>) {
//!     let photo = context.photo;
//!     // ..
//! }
//! ```
//!
//! You can generalize it to this one in order to also support
//! [`contexts::EditedPhoto`]:
//!
//! ```
//! use tbot::{contexts::fields::Photo, connectors::Https};
//! fn process_photo(context: &impl Photo<Https>) {
//!     let photo = context.photo();
//!     // ..
//! }
//! ```
//!
//! [`contexts::Photo`]: ../struct.Photo.html
//! [`contexts::EditedPhoto`]: ../struct.EditedPhoto.html
//! [`fields::Photo`]: ./struct.Photo.html

mod context;

pub use context::Context;

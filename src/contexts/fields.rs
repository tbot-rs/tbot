//! Traits for common context fields.
//!
//! Suppose that you want to process users' photos whenever they send or edit
//! one. You would like to abstract this as much as possible, like this:
//!
//! ```
//! # async fn process_photo<T>(_: std::sync::Arc<T>) { }
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
//! use std::sync::Arc;
//! use tbot::{contexts};
//! async fn process_photo(context: Arc<contexts::Photo>) {
//!     let photo = &context.photo;
//!     // ..
//! }
//! ```
//!
//! You can generalize it to this one in order to also support
//! [`contexts::EditedPhoto`]:
//!
//! ```
//! use std::sync::Arc;
//! use tbot::{contexts::fields::Photo};
//! async fn process_photo(context: Arc<impl Photo>) {
//!     let photo = context.photo();
//!     // ..
//! }
//! ```
//!
//! [`contexts::Photo`]: super::Photo
//! [`contexts::EditedPhoto`]: super::EditedPhoto
//! [`fields::Photo`]: Photo

mod album;
mod attachments;
mod callback;
mod context;
mod messages;
mod texts;

pub use {
    album::Album,
    attachments::{Animation, Audio, Document, Location, Photo, Video},
    callback::Callback,
    context::Context,
    messages::{EditedMessage, Forward, MediaMessage, Message},
    texts::{AnyText, Caption, Text},
};

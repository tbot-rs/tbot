//! Contains contexts for handlers.
//!
//! A context is a struct that is passed to update handlers, contains data about
//! the update, and provides methods that infer certain data from the update.
//! For example:
//!
//! ```no_run
//! use tbot::prelude::*;
//!
//! let mut bot = tbot::bot!("BOT_TOKEN");
//!
//! bot.text(|context| {
//!     let reversed: String = context.text.text.chars().rev().collect();
//!     let reply = context
//!         .send_message_in_reply(&reversed)
//!         .into_future()
//!         .map_err(|error| {
//!             dbg!(error);
//!         });
//!
//!     tbot::spawn(reply);
//! });
//! ```
//!
//! Here, we set a [`text`][text-handler] handler for our bot. Whenever we get
//! a text message, the handler is called with a reference to
//! a [`Text`][text-context] context that contains data about the incoming data,
//! e.g. the text of the message. Then we call the [`send_message_in_reply`]
//! method on the context, which does what the name says: sends a message
//! in the same chat in reply to the incoming message, inferring your bot's
//! token and IDs of the chat and the message.
//!
//! All contexts have one common field named `bot`. Through this field, you can
//! call any API method you can call using [`Bot`] (though it's really
//! a [`MockBot`]). For example:
//!
//! ```no_run
//! # use tbot::prelude::*;
//! # let mut bot = tbot::Bot::new(String::new());
//! const ADMIN_CHAT: i64 = 0;
//!
//! bot.text(|context| {
//!     let notification = context
//!         .bot
//!         .send_message(ADMIN_CHAT, "New message!")
//!         .into_future()
//!         .map_err(|error| {
//!             dbg!(error);
//!         });
//!
//!     tbot::spawn(notification);
//! });
//! ```
//!
//! Most contexts implement certain traits, such as [`ChatMethods`]
//! or [`Pinnable`]. These traits share common methods between contexts,
//! e.g. [`send_message_in_reply`] you have seen above.
//!
//! [text-handler]: ../struct.Bot.html#method.text
//! [text-context]: ./struct.Text.html
//! [`send_message_in_reply`]: ./traits/trait.ChatMethods.html#method.send_message_in_reply
//! [`Bot`]: ../struct.Bot.html
//! [`MockBot`]: ../struct.MockBot.html
//! [`ChatMethods`]: ./traits/trait.ChatMethods.html
//! [`Pinnable`]: ./traits/trait.Pinnable.html

#![allow(clippy::too_many_arguments)] // can't do much
                                      // we re-export everything under one namespace
#![allow(clippy::module_name_repetitions)]

use super::*;

#[macro_use]
mod macros;

mod animation;
mod audio;
mod contact;
mod created_group;
mod deleted_chat_photo;
mod document;
mod edited_animation;
mod edited_audio;
mod edited_document;
mod edited_location;
mod edited_photo;
mod edited_text;
mod edited_video;
mod game;
mod left_member;
mod location;
mod migration;
mod new_chat_photo;
mod new_chat_title;
mod new_members;
mod photo;
mod pinned_message;
mod poll;
mod sticker;
mod text;
mod unhandled;
mod update;
mod updated_poll;
mod venue;
mod video;
mod video_note;
mod voice;

pub mod traits;

pub use {
    animation::*, audio::*, contact::*, created_group::*,
    deleted_chat_photo::*, document::*, edited_animation::*, edited_audio::*,
    edited_document::*, edited_location::*, edited_photo::*, edited_text::*,
    edited_video::*, game::*, left_member::*, location::*, migration::*,
    new_chat_photo::*, new_chat_title::*, new_members::*, photo::*,
    pinned_message::*, poll::*, sticker::*, text::*, unhandled::*, update::*,
    updated_poll::*, venue::*, video::*, video_note::*, voice::*,
};

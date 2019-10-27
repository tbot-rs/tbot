//! Contexts for update handlers.
//!
//! A context is a struct that is passed to update handlers, contains data about
//! the update, and provides methods that infer certain data from the update.
//! For example:
//!
//! ```no_run
//! # async fn foo() {
//! use tbot::prelude::*;
//!
//! let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();
//!
//! bot.text(|context| {
//!     async move {
//!         let reversed: String = context.text.value.chars().rev().collect();
//!         context.send_message_in_reply(&reversed).call().await.unwrap();
//!     }
//! });
//! # }
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
//! call any API method you can call using a [`Bot`]:
//!
//! ```no_run
//! # async fn foo() {
//! # use tbot::prelude::*;
//! # let mut bot = tbot::Bot::new(String::new()).event_loop();
//! use tbot::types::chat;
//! const ADMIN_CHAT: chat::Id = chat::Id(0);
//!
//! bot.text(|context| {
//!     async move {
//!         context
//!             .bot
//!             .send_message(ADMIN_CHAT, "New message!")
//!             .call()
//!             .await
//!             .unwrap();
//!     }
//! });
//! # }
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
//! [`ChatMethods`]: ./traits/trait.ChatMethods.html
//! [`Pinnable`]: ./traits/trait.Pinnable.html

#[macro_use]
mod macros;

mod animation;
mod audio;
mod chosen_inline;
mod connected_website;
mod contact;
mod created_group;
mod data_callback;
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
mod game_callback;
mod inline;
mod invoice;
mod left_member;
mod location;
mod migration;
mod new_chat_photo;
mod new_chat_title;
mod new_members;
mod passport;
mod payment;
mod photo;
mod pinned_message;
mod poll;
mod pre_checkout;
mod shipping;
mod sticker;
mod text;
mod unhandled;
mod update;
mod updated_poll;
mod venue;
mod video;
mod video_note;
mod voice;

pub mod fields;
pub mod traits;

pub use {
    animation::Animation, audio::Audio, chosen_inline::ChosenInline,
    connected_website::ConnectedWebsite, contact::Contact,
    created_group::CreatedGroup, data_callback::DataCallback,
    deleted_chat_photo::DeletedChatPhoto, document::Document,
    edited_animation::EditedAnimation, edited_audio::EditedAudio,
    edited_document::EditedDocument, edited_location::EditedLocation,
    edited_photo::EditedPhoto, edited_text::EditedText,
    edited_video::EditedVideo, game::Game, game_callback::GameCallback,
    inline::Inline, invoice::Invoice, left_member::LeftMember,
    location::Location, migration::Migration, new_chat_photo::NewChatPhoto,
    new_chat_title::NewChatTitle, new_members::NewMembers, passport::Passport,
    payment::Payment, photo::Photo, pinned_message::PinnedMessage, poll::Poll,
    pre_checkout::PreCheckout, shipping::Shipping, sticker::Sticker,
    text::Text, unhandled::Unhandled, update::Update,
    updated_poll::UpdatedPoll, venue::Venue, video::Video,
    video_note::VideoNote, voice::Voice,
};

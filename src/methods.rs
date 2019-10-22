//! Structs for calling API methods.
//!
//! The methods from this module are low-level: you have to pass everything
//! a method needs to their `new` methods. More likely, you'd like to use
//! [`Bot`] to infer your bot's token when calling methods. Moreover, when
//! handling updates, their [contexts] provide methods that infer even more
//! information from the update.
//!
//! All the methods have a common pattern:
//!
//! - Methods provide the builder API for optional parameters;
//! - Methods have the `call` method, which returns a `Future` that makes
//!   the request and resolves with the call's result.
//!
//! For example, here's how to call [`SendMessage`]:
//!
//! ```no_run
//! # async fn foo() {
//! use tbot::{
//!     prelude::*,
//!     types::{chat, parameters::Text},
//!     Token,
//! };
//!
//! const CHAT: chat::Id = chat::Id(0);
//! const MESSAGE: &str = "`tbot` is a super-cool crate!";
//!
//! let bot = tbot::from_env!("BOT_TOKEN");
//!
//! bot.send_message(CHAT, Text::markdown(MESSAGE)).call().await.unwrap();
//! # }
//! ```
//!
//! Each method resolves with a `Result` that may be `Ok` if the call
//! was successful or `Err` if an error happened. Here, we simply `unwrap`
//! the result, but you should handle error properly.

//! # Inline/message methods
//!
//! Several API methods accept either (`chat_id` and `message_id`) or
//! `inline_message_id`, and their return type depends on the chosen
//! parameters. For such methods, `tbot` provides two structs, e.g. for
//! [`editMessageText`][tg-doc] there are [`EditMessageText`] which resolves to
//! [`types::Message`] and [`EditInlineText`] which resolves to `()`. This
//! brings a more straightforward API wrapper, unlike if we only had one method
//! which would resolve to `(() | types::Message)`.
//!
//! [`Bot`]: ../struct.Bot.html
//! [contexts]: ../contexts/
//! [`SendMessage`]: ./struct.SendMessage.html
//! [tg-doc]: https://core.telegram.org/bots/api#editmessagetext
//! [`EditMessageText`]: ./struct.EditMessageText.html
//! [`EditInlineText`]: ./struct.EditInlineText.html
//! [`types::Message`]: ../types/struct.Message.html

use super::*;
use crate::types::chat;

mod add_sticker_to_set;
mod answer_callback_query;
mod answer_inline_query;
mod answer_pre_checkout_query;
mod answer_shipping_query;
mod create_new_sticker_set;
mod delete_chat_photo;
mod delete_chat_sticker_set;
mod delete_message;
mod delete_sticker_from_set;
mod delete_webhook;
mod edit_inline_caption;
mod edit_inline_location;
mod edit_inline_media;
mod edit_inline_reply_markup;
mod edit_inline_text;
mod edit_message_caption;
mod edit_message_location;
mod edit_message_media;
mod edit_message_reply_markup;
mod edit_message_text;
mod export_chat_invite_link;
mod forward_message;
mod get_chat;
mod get_chat_administrators;
mod get_chat_member;
mod get_chat_members_count;
mod get_file;
mod get_inline_game_high_scores;
mod get_me;
mod get_message_game_high_scores;
mod get_sticker_set;
mod get_updates;
mod get_user_profile_photos;
mod get_webhook_info;
mod kick_chat_member;
mod leave_chat;
mod pin_chat_message;
mod promote_chat_member;
mod restrict_chat_member;
mod send_animation;
mod send_audio;
mod send_chat_action;
mod send_contact;
mod send_document;
mod send_game;
mod send_invoice;
mod send_location;
mod send_media_group;
mod send_message;
mod send_photo;
mod send_poll;
mod send_sticker;
mod send_venue;
mod send_video;
mod send_video_note;
mod send_voice;
mod set_chat_description;
mod set_chat_permissions;
mod set_chat_photo;
mod set_chat_sticker_set;
mod set_chat_title;
mod set_inline_game_score;
mod set_message_game_score;
mod set_passport_data_errors;
mod set_sticker_position_in_set;
mod set_webhook;
mod stop_inline_location;
mod stop_message_location;
mod stop_poll;
mod unban_chat_member;
mod unpin_chat_message;
mod upload_sticker_file;

pub use {
    add_sticker_to_set::*, answer_callback_query::*, answer_inline_query::*,
    answer_pre_checkout_query::*, answer_shipping_query::*,
    create_new_sticker_set::*, delete_chat_photo::*,
    delete_chat_sticker_set::*, delete_message::*, delete_sticker_from_set::*,
    edit_inline_caption::*, edit_inline_location::*, edit_inline_media::*,
    edit_inline_reply_markup::*, edit_inline_text::*, edit_message_caption::*,
    edit_message_location::*, edit_message_media::*,
    edit_message_reply_markup::*, edit_message_text::*,
    export_chat_invite_link::*, forward_message::*, get_chat::*,
    get_chat_administrators::*, get_chat_member::*, get_chat_members_count::*,
    get_file::*, get_inline_game_high_scores::*, get_me::*,
    get_message_game_high_scores::*, get_sticker_set::*,
    get_user_profile_photos::*, get_webhook_info::*, kick_chat_member::*,
    leave_chat::*, pin_chat_message::*, promote_chat_member::*,
    restrict_chat_member::*, send_animation::*, send_audio::*,
    send_chat_action::*, send_contact::*, send_document::*, send_game::*,
    send_invoice::*, send_location::*, send_media_group::*, send_message::*,
    send_photo::*, send_photo::*, send_poll::*, send_poll::*, send_sticker::*,
    send_sticker::*, send_venue::*, send_venue::*, send_video::*,
    send_video::*, send_video_note::*, send_video_note::*, send_voice::*,
    send_voice::*, set_chat_description::*, set_chat_description::*,
    set_chat_permissions::*, set_chat_photo::*, set_chat_photo::*,
    set_chat_sticker_set::*, set_chat_title::*, set_chat_title::*,
    set_inline_game_score::*, set_message_game_score::*,
    set_passport_data_errors::*, set_sticker_position_in_set::*,
    set_sticker_position_in_set::*, stop_inline_location::*,
    stop_inline_location::*, stop_message_location::*,
    stop_message_location::*, stop_poll::*, stop_poll::*, unban_chat_member::*,
    unban_chat_member::*, unpin_chat_message::*, unpin_chat_message::*,
    upload_sticker_file::*, upload_sticker_file::*,
};

pub(crate) use {delete_webhook::*, get_updates::*, set_webhook::*};

mod call_method;
use call_method::*;

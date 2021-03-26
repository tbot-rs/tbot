//! Structs for calling API methods.
//!
//! The methods from this module can't be constructed directly; instead, you
//! construct them using a [`Bot`] instance which infers the underlying client
//! and your bot's token, or using [contexts] that infer even more information
//! from the update, such as the chat ID.
//!
//! All the methods have a common pattern:
//!
//! - Methods provide Builder API for optional parameters;
//! - Methods have the asynchronous `call` method, which calls the method
//!   and resolves with a `Result`.
//!
//! For example, here's how to call [`SendMessage`]:
//!
//! ```no_run
//! # async fn foo() {
//! use tbot::types::{chat, parameters::Text};
//!
//! const CHAT: chat::Id = chat::Id(0);
//!
//! let bot = tbot::from_env!("BOT_TOKEN");
//! let message = Text::with_markdown("`tbot` is a super-cool crate!");
//! bot.send_message(CHAT, message).call().await.unwrap();
//! # }
//! ```
//!
//! # Inline/message methods
//!
//! Several API methods accept either (`chat_id` and `message_id`) or
//! `inline_message_id`, and their return type depends on the chosen
//! parameters. For such methods, `tbot` provides two structs, e.g. for
//! [`editMessageText`][tg-doc] there are [`EditMessageText`] which resolves to
//! [`types::Message`] and [`EditInlineText`] which resolves to `()`. This
//! brings a type-safer API wrapper, unlike if we only had one method
//! which would resolve to `(() | types::Message)`.
//!
//! [`Bot`]: ../bot/struct.Bot.html
//! [contexts]: ../contexts/
//! [`SendMessage`]: ./struct.SendMessage.html
//! [tg-doc]: https://core.telegram.org/bots/api#editmessagetext
//! [`EditMessageText`]: ./struct.EditMessageText.html
//! [`EditInlineText`]: ./struct.EditInlineText.html
//! [`types::Message`]: ../types/struct.Message.html

#![allow(clippy::wrong_self_convention, clippy::missing_panics_doc)]

mod add_sticker_to_set;
mod answer_callback_query;
mod answer_inline_query;
mod answer_pre_checkout_query;
mod answer_shipping_query;
mod close;
mod copy_message;
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
mod get_my_commands;
mod get_sticker_set;
mod get_updates;
mod get_user_profile_photos;
mod get_webhook_info;
mod kick_chat_member;
mod leave_chat;
mod log_out;
mod pin_chat_message;
mod promote_chat_member;
mod restrict_chat_member;
mod send_animation;
mod send_audio;
mod send_chat_action;
mod send_contact;
mod send_dice;
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
mod set_chat_administrator_custom_title;
mod set_chat_description;
mod set_chat_permissions;
mod set_chat_photo;
mod set_chat_sticker_set;
mod set_chat_title;
mod set_inline_game_score;
mod set_message_game_score;
mod set_my_commands;
mod set_passport_data_errors;
mod set_sticker_position_in_set;
mod set_sticker_set_thumb;
mod set_webhook;
mod stop_inline_location;
mod stop_message_location;
mod stop_poll;
mod unban_chat_member;
mod unpin_all_chat_messages;
mod unpin_chat_message;
mod upload_sticker_file;

pub use add_sticker_to_set::AddStickerToSet;
pub use answer_callback_query::AnswerCallbackQuery;
pub use answer_inline_query::AnswerInlineQuery;
pub use answer_pre_checkout_query::AnswerPreCheckoutQuery;
pub use answer_shipping_query::AnswerShippingQuery;
pub use copy_message::CopyMessage;
pub use create_new_sticker_set::CreateNewStickerSet;
pub use delete_chat_photo::DeleteChatPhoto;
pub use delete_chat_sticker_set::DeleteChatStickerSet;
pub use delete_message::DeleteMessage;
pub use delete_sticker_from_set::DeleteStickerFromSet;
pub use edit_inline_caption::EditInlineCaption;
pub use edit_inline_location::EditInlineLocation;
pub use edit_inline_media::EditInlineMedia;
pub use edit_inline_reply_markup::EditInlineReplyMarkup;
pub use edit_inline_text::EditInlineText;
pub use edit_message_caption::EditMessageCaption;
pub use edit_message_location::EditMessageLocation;
pub use edit_message_media::EditMessageMedia;
pub use edit_message_reply_markup::EditMessageReplyMarkup;
pub use edit_message_text::EditMessageText;
pub use export_chat_invite_link::ExportChatInviteLink;
pub use forward_message::ForwardMessage;
pub use get_chat::GetChat;
pub use get_chat_administrators::GetChatAdministrators;
pub use get_chat_member::GetChatMember;
pub use get_chat_members_count::GetChatMembersCount;
pub use get_file::GetFile;
pub use get_inline_game_high_scores::GetInlineGameHighScores;
pub use get_me::GetMe;
pub use get_message_game_high_scores::GetMessageGameHighScores;
pub use get_my_commands::GetMyCommands;
pub use get_sticker_set::GetStickerSet;
pub use get_user_profile_photos::GetUserProfilePhotos;
pub use get_webhook_info::GetWebhookInfo;
pub use kick_chat_member::KickChatMember;
pub use leave_chat::LeaveChat;
pub use pin_chat_message::PinChatMessage;
pub use promote_chat_member::PromoteChatMember;
pub use restrict_chat_member::RestrictChatMember;
pub use send_animation::SendAnimation;
pub use send_audio::SendAudio;
pub use send_chat_action::SendChatAction;
pub use send_contact::SendContact;
pub use send_dice::SendDice;
pub use send_document::SendDocument;
pub use send_game::SendGame;
pub use send_invoice::SendInvoice;
pub use send_location::SendLocation;
pub use send_media_group::SendMediaGroup;
pub use send_message::SendMessage;
pub use send_photo::SendPhoto;
pub use send_poll::SendPoll;
pub use send_sticker::SendSticker;
pub use send_venue::SendVenue;
pub use send_video::SendVideo;
pub use send_video_note::SendVideoNote;
pub use send_voice::SendVoice;
pub use set_chat_administrator_custom_title::SetChatAdministratorCustomTitle;
pub use set_chat_description::SetChatDescription;
pub use set_chat_permissions::SetChatPermissions;
pub use set_chat_photo::SetChatPhoto;
pub use set_chat_sticker_set::SetChatStickerSet;
pub use set_chat_title::SetChatTitle;
pub use set_inline_game_score::SetInlineGameScore;
pub use set_message_game_score::SetMessageGameScore;
pub use set_my_commands::SetMyCommands;
pub use set_passport_data_errors::SetPassportDataErrors;
pub use set_sticker_position_in_set::SetStickerPositionInSet;
pub use set_sticker_set_thumb::SetStickerSetThumb;
pub use stop_inline_location::StopInlineLocation;
pub use stop_message_location::StopMessageLocation;
pub use stop_poll::StopPoll;
pub use unban_chat_member::UnbanChatMember;
pub use unpin_all_chat_messages::UnpinAllChatMessages;
pub use unpin_chat_message::UnpinChatMessage;
pub use upload_sticker_file::UploadStickerFile;

pub(crate) use {
    close::Close, delete_webhook::DeleteWebhook, get_updates::GetUpdates,
    log_out::LogOut, set_webhook::SetWebhook,
};

mod call_method;
use call_method::call_method;

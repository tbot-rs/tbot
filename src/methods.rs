//! Structs for calling API methods.
//!
//! The methods from this module can't be constructed directly; instead, you
//! construct them using a [`Bot`] instance, which infers the underlying client
//! and your bot's token, and [contexts] that can infer even more information
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
//! const MESSAGE: &str = "`tbot` is a super-cool crate!";
//!
//! let bot = tbot::from_env!("BOT_TOKEN");
//! bot.send_message(CHAT, Text::markdown(MESSAGE)).call().await.unwrap();
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
mod get_my_commands;
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
mod set_webhook;
mod stop_inline_location;
mod stop_message_location;
mod stop_poll;
mod unban_chat_member;
mod unpin_chat_message;
mod upload_sticker_file;

pub use {
    add_sticker_to_set::AddStickerToSet,
    answer_callback_query::AnswerCallbackQuery,
    answer_inline_query::AnswerInlineQuery,
    answer_pre_checkout_query::AnswerPreCheckoutQuery,
    answer_shipping_query::AnswerShippingQuery,
    create_new_sticker_set::CreateNewStickerSet,
    delete_chat_photo::DeleteChatPhoto,
    delete_chat_sticker_set::DeleteChatStickerSet,
    delete_message::DeleteMessage,
    delete_sticker_from_set::DeleteStickerFromSet,
    edit_inline_caption::EditInlineCaption,
    edit_inline_location::EditInlineLocation,
    edit_inline_media::EditInlineMedia,
    edit_inline_reply_markup::EditInlineReplyMarkup,
    edit_inline_text::EditInlineText, edit_message_caption::EditMessageCaption,
    edit_message_location::EditMessageLocation,
    edit_message_media::EditMessageMedia,
    edit_message_reply_markup::EditMessageReplyMarkup,
    edit_message_text::EditMessageText,
    export_chat_invite_link::ExportChatInviteLink,
    forward_message::ForwardMessage, get_chat::GetChat,
    get_chat_administrators::GetChatAdministrators,
    get_chat_member::GetChatMember,
    get_chat_members_count::GetChatMembersCount, get_file::GetFile,
    get_inline_game_high_scores::GetInlineGameHighScores, get_me::GetMe,
    get_message_game_high_scores::GetMessageGameHighScores,
    get_my_commands::GetMyCommands, get_sticker_set::GetStickerSet,
    get_user_profile_photos::GetUserProfilePhotos,
    get_webhook_info::GetWebhookInfo, kick_chat_member::KickChatMember,
    leave_chat::LeaveChat, pin_chat_message::PinChatMessage,
    promote_chat_member::PromoteChatMember,
    restrict_chat_member::RestrictChatMember, send_animation::SendAnimation,
    send_audio::SendAudio, send_chat_action::SendChatAction,
    send_contact::SendContact, send_dice::SendDice,
    send_document::SendDocument, send_game::SendGame,
    send_invoice::SendInvoice, send_location::SendLocation,
    send_media_group::SendMediaGroup, send_message::SendMessage,
    send_photo::SendPhoto, send_poll::SendPoll, send_sticker::SendSticker,
    send_venue::SendVenue, send_video::SendVideo,
    send_video_note::SendVideoNote, send_voice::SendVoice,
    set_chat_administrator_custom_title::SetChatAdministratorCustomTitle,
    set_chat_description::SetChatDescription,
    set_chat_permissions::SetChatPermissions, set_chat_photo::SetChatPhoto,
    set_chat_sticker_set::SetChatStickerSet, set_chat_title::SetChatTitle,
    set_inline_game_score::SetInlineGameScore,
    set_message_game_score::SetMessageGameScore,
    set_my_commands::SetMyCommands,
    set_passport_data_errors::SetPassportDataErrors,
    set_sticker_position_in_set::SetStickerPositionInSet,
    stop_inline_location::StopInlineLocation,
    stop_message_location::StopMessageLocation, stop_poll::StopPoll,
    unban_chat_member::UnbanChatMember, unpin_chat_message::UnpinChatMessage,
    upload_sticker_file::UploadStickerFile,
};

pub(crate) use {
    delete_webhook::DeleteWebhook, get_updates::GetUpdates,
    set_webhook::SetWebhook,
};

mod call_method;
use call_method::send_method;

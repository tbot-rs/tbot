//! Types used as parameters, mainly for methods.

mod bot_command;
mod callback_action;
mod chat_id;
mod flexibility;
mod notification_state;
mod photo;
pub mod poll;
mod requirement;
mod send_to_provider_state;
mod text;
mod update_kind;
mod url_visibility;
mod web_page_preview_state;

pub(crate) use text::ParseMode;
pub use {
    bot_command::BotCommand,
    callback_action::CallbackAction,
    chat_id::{ChatId, ImplicitChatId},
    flexibility::Flexibility,
    notification_state::NotificationState,
    photo::Photo,
    poll::Any,
    requirement::Requirement,
    send_to_provider_state::SendToProviderState,
    text::Text,
    update_kind::UpdateKind,
    url_visibility::UrlVisibility,
    web_page_preview_state::WebPagePreviewState,
};

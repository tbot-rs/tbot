use super::call_method;
use crate::{
    connectors::Client,
    errors, token,
    types::{
        keyboard::inline,
        message::{self, Message},
        parameters::{ChatId, ImplicitChatId},
    },
};
use serde::Serialize;

/// Edits a live location sent by the bot itself.
///
/// Reflects the [`editMessageLiveLocation`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagelivelocation
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditMessageLocation<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
    message_id: message::Id,
    latitude: f64,
    longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<inline::Keyboard<'a>>,
}

impl<'a> EditMessageLocation<'a> {
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
        (latitude, longitude): (f64, f64),
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            message_id,
            latitude,
            longitude,
            reply_markup: None,
        }
    }

    /// Configures an inline keyboard for the message.
    /// Reflects the `reply_markup` parameter.
    pub fn reply_markup(mut self, markup: inline::Keyboard<'a>) -> Self {
        self.reply_markup = Some(markup);
        self
    }
}

impl EditMessageLocation<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Message, errors::MethodCall> {
        call_method(
            self.client,
            self.token,
            "editMessageLiveLocation",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}

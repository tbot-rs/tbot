use super::*;
use crate::{
    connectors::Connector,
    errors,
    internal::{BoxFuture, Client},
    types::{
        input_file::{InputFile, Sticker},
        keyboard, message,
        parameters::{ChatId, ImplicitChatId, NotificationState},
    },
};

/// Sends a sticker.
///
/// Reflects the [`sendSticker`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendsticker
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendSticker<'a, C> {
    client: &'a Client<C>,
    token: Token,
    chat_id: ChatId<'a>,
    sticker: Sticker<'a>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<message::Id>,
    reply_markup: Option<keyboard::Any<'a>>,
}

impl<'a, C> SendSticker<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl ImplicitChatId<'a>,
        sticker: Sticker<'a>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            sticker,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Configures if the message will be sent silently.
    /// Reflects the `disable_notification` parameter.
    pub fn notification(mut self, state: NotificationState) -> Self {
        self.disable_notification = Some(state.is_disabled());
        self
    }

    /// Configures which message this sticker is sent in reply to.
    /// Reflects the `reply_to_message_id` parameter.
    pub fn reply_to_message_id(mut self, id: message::Id) -> Self {
        self.reply_to_message_id = Some(id);
        self
    }

    /// Configures a keyboard for the message.
    /// Reflects the `reply_markup` parameter.
    pub fn reply_markup(
        mut self,
        markup: impl Into<keyboard::Any<'a>>,
    ) -> Self {
        self.reply_markup = Some(markup.into());
        self
    }
}

impl<C: Connector> IntoFuture for SendSticker<'_, C> {
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = types::Message;
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        let mut multipart = Multipart::new(5)
            .chat_id("chat_id", self.chat_id)
            .maybe_string("disabled_notification", self.disable_notification)
            .maybe_string("reply_to_message_id", self.reply_to_message_id)
            .maybe_json("reply_markup", self.reply_markup);

        match self.sticker.media {
            InputFile::File {
                filename, bytes, ..
            } => multipart = multipart.file("sticker", filename, bytes),
            InputFile::Id(sticker) | InputFile::Url(sticker) => {
                multipart = multipart.str("sticker", sticker);
            }
        }

        let (boundary, body) = multipart.finish();

        Box::new(send_method(
            self.client,
            &self.token,
            "sendSticker",
            Some(boundary),
            body,
        ))
    }
}

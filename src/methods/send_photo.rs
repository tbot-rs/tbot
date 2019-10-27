use super::send_method;
use crate::{
    connectors::Connector,
    errors,
    internal::Client,
    types::{
        input_file::{InputFile, Photo},
        keyboard,
        message::{self, Message},
        parameters::{ChatId, ImplicitChatId, NotificationState},
    },
    Multipart, token,
};

/// Sends a photo.
///
/// Reflects the [`sendPhoto`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendphoto
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendPhoto<'a, C> {
    client: &'a Client<C>,
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
    photo: Photo<'a>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<message::Id>,
    reply_markup: Option<keyboard::Any<'a>>,
}

impl<'a, C> SendPhoto<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: token::Ref<'a>,
        chat_id: impl ImplicitChatId<'a>,
        photo: Photo<'a>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            photo,
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

    /// Configures which message this photo is sent in reply to.
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

impl<C: Connector> SendPhoto<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<Message, errors::MethodCall> {
        let mut multipart = Multipart::new(7)
            .chat_id("chat_id", self.chat_id)
            .maybe_str("caption", self.photo.caption)
            .maybe_string("parse_mode", self.photo.parse_mode)
            .maybe_string("disabled_notification", self.disable_notification)
            .maybe_string("reply_to_message_id", self.reply_to_message_id)
            .maybe_json("reply_markup", self.reply_markup);

        match self.photo.media {
            InputFile::File {
                filename, bytes, ..
            } => multipart = multipart.file("photo", filename, bytes),
            InputFile::Id(photo) | InputFile::Url(photo) => {
                multipart = multipart.str("photo", photo);
            }
        }

        let (boundary, body) = multipart.finish();

        send_method(self.client, self.token, "sendPhoto", Some(boundary), body)
            .await
    }
}

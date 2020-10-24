use super::call_method;
#[allow(deprecated)]
use crate::{
    connectors::Client,
    errors, token,
    types::{
        input_file::{InputFile, Photo},
        keyboard,
        message::{self, Message},
        parameters::{ChatId, ImplicitChatId, NotificationState},
    },
    Multipart,
};

/// Sends a photo.
///
/// Reflects the [`sendPhoto`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendphoto
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendPhoto<'a> {
    client: &'a Client,
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
    photo: Photo<'a>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<message::Id>,
    reply_markup: Option<keyboard::Any<'a>>,
}

impl<'a> SendPhoto<'a> {
    pub(crate) fn new(
        client: &'a Client,
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
    pub fn is_notification_disabled(mut self, is_disabled: bool) -> Self {
        self.disable_notification = Some(is_disabled);
        self
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "use `is_notification_disabled` which takes a `bool`"
    )]
    #[allow(deprecated)]
    pub fn notification(self, state: NotificationState) -> Self {
        self.is_notification_disabled(state.is_disabled())
    }

    /// Configures which message this photo is sent in reply to.
    /// Reflects the `reply_to_message_id` parameter.
    pub fn in_reply_to(mut self, id: message::Id) -> Self {
        self.reply_to_message_id = Some(id);
        self
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "this method is renamed to `in_reply_to`"
    )]
    pub fn reply_to_message_id(self, id: message::Id) -> Self {
        self.in_reply_to(id)
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

impl SendPhoto<'_> {
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

        call_method(self.client, self.token, "sendPhoto", Some(boundary), body)
            .await
    }
}

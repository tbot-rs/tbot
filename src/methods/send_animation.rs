use super::*;
use crate::{
    connectors::Connector,
    errors,
    internal::{BoxFuture, Client},
    types::{
        input_file::{Animation, InputFile, Thumb},
        keyboard, message,
        parameters::{ChatId, ImplicitChatId, NotificationState},
    },
};

/// Sends an animation.
///
/// Reflects the [`sendAnimation`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendanimation
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendAnimation<'a, C> {
    client: &'a Client<C>,
    token: Token,
    chat_id: ChatId<'a>,
    animation: Animation<'a>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<message::Id>,
    reply_markup: Option<keyboard::Any<'a>>,
}

impl<'a, C> SendAnimation<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl ImplicitChatId<'a>,
        animation: Animation<'a>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            animation,
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

    /// Configures which message this animation is sent in reply to.
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

impl<C: Connector> IntoFuture for SendAnimation<'_, C> {
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = types::Message;
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        let mut multipart = Multipart::new(11)
            .chat_id("chat_id", self.chat_id)
            .maybe_string("duration", self.animation.duration)
            .maybe_string("width", self.animation.width)
            .maybe_string("height", self.animation.height)
            .maybe_str("caption", self.animation.caption)
            .maybe_string("parse_mode", self.animation.parse_mode)
            .maybe_string("disable_notification", self.disable_notification)
            .maybe_string("reply_to_message_id", self.reply_to_message_id)
            .maybe_json("reply_markup", self.reply_markup);

        match self.animation.media {
            InputFile::File {
                filename, bytes, ..
            } => multipart = multipart.file("animation", filename, bytes),
            InputFile::Id(animation) | InputFile::Url(animation) => {
                multipart = multipart.str("animation", animation);
            }
        }

        if let Some(Thumb(InputFile::File {
            filename, bytes, ..
        })) = self.animation.thumb
        {
            multipart = multipart.file("thumb", filename, bytes);
        }

        let (boundary, body) = multipart.finish();

        Box::new(send_method(
            self.client,
            &self.token,
            "sendAnimation",
            Some(boundary),
            body,
        ))
    }
}

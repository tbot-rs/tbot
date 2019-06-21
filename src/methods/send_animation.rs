use super::*;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    types::{
        input_file::{Animation, InputFile},
        keyboard, message,
        parameters::{ChatId, ImplicitChatId, NotificationState},
    },
};

/// Represents the [`sendAnimation`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendanimation
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendAnimation<'a, C> {
    client: &'a Client<C>,
    token: Token,
    chat_id: ChatId<'a>,
    animation: &'a Animation<'a>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<message::Id>,
    reply_markup: Option<keyboard::Any<'a>>,
}

impl<'a, C> SendAnimation<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl ImplicitChatId<'a>,
        animation: &'a Animation<'a>,
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

    /// Configures `disable_notification`.
    pub fn notification(mut self, state: NotificationState) -> Self {
        self.disable_notification = Some(state.is_disabled());
        self
    }

    /// Configures `reply_to_message_id`.
    pub fn reply_to_message_id(mut self, id: message::Id) -> Self {
        self.reply_to_message_id = Some(id);
        self
    }

    /// Configures `reply_markup`.
    pub fn reply_markup(
        mut self,
        markup: impl Into<keyboard::Any<'a>>,
    ) -> Self {
        self.reply_markup = Some(markup.into());
        self
    }
}

impl<C> IntoFuture for SendAnimation<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = types::Message;
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        let chat_id = match self.chat_id {
            ChatId::Id(id) => id.to_string(),
            ChatId::Username(username) => username.into(),
        };

        let duration = self.animation.duration.map(|x| x.to_string());
        let width = self.animation.width.map(|x| x.to_string());
        let height = self.animation.height.map(|x| x.to_string());
        let parse_mode = self.animation.parse_mode.map(|x| x.to_string());
        let is_disabled = self.disable_notification.map(|x| x.to_string());
        let reply_to = self.reply_to_message_id.map(|id| id.to_string());
        let reply_markup =
            self.reply_markup.and_then(|x| serde_json::to_string(&x).ok());

        let mut multipart = Multipart::new(11)
            .str("chat_id", &chat_id)
            .maybe_string("duration", &duration)
            .maybe_string("width", &width)
            .maybe_string("height", &height)
            .maybe_str("caption", self.animation.caption)
            .maybe_string("parse_mode", &parse_mode)
            .maybe_string("disable_notification", &is_disabled)
            .maybe_string("reply_to_message_id", &reply_to)
            .maybe_string("reply_markup", &reply_markup);

        match self.animation.media {
            InputFile::File {
                filename,
                bytes,
                ..
            } => multipart = multipart.file("animation", filename, bytes),
            InputFile::Id(animation) | InputFile::Url(animation) => {
                multipart = multipart.str("animation", animation);
            }
        }

        if let Some(InputFile::File {
            filename,
            bytes,
            ..
        }) = self.animation.thumb
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

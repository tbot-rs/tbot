use super::*;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    types::{
        input_file::{InputFile, Thumb, Video},
        keyboard, message,
        parameters::{ChatId, ImplicitChatId, NotificationState},
    },
};

/// Sends a video.
///
/// Reflects the [`sendVideo`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendvideo
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendVideo<'a, C> {
    client: &'a Client<C>,
    token: Token,
    chat_id: ChatId<'a>,
    video: Video<'a>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<message::Id>,
    reply_markup: Option<keyboard::Any<'a>>,
}

impl<'a, C> SendVideo<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl ImplicitChatId<'a>,
        video: Video<'a>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            video,
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

    /// Configures which message this video is sent in reply to.
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

impl<C> IntoFuture for SendVideo<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = types::Message;
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        let mut multipart = Multipart::new(12)
            .chat_id("chat_id", self.chat_id)
            .maybe_string("duration", self.video.duration)
            .maybe_string("width", self.video.width)
            .maybe_string("height", self.video.height)
            .maybe_str("caption", self.video.caption)
            .maybe_string("parse_mode", self.video.parse_mode)
            .maybe_string("disable_notification", self.disable_notification)
            .maybe_string("supports_streaming", self.video.supports_streaming)
            .maybe_string("reply_to_message_id", self.reply_to_message_id)
            .maybe_json("reply_markup", self.reply_markup);

        match self.video.media {
            InputFile::File {
                filename, bytes, ..
            } => multipart = multipart.file("video", filename, bytes),
            InputFile::Id(audio) | InputFile::Url(audio) => {
                multipart = multipart.str("video", audio);
            }
        }

        if let Some(Thumb(InputFile::File {
            filename, bytes, ..
        })) = self.video.thumb
        {
            multipart = multipart.file("thumb", filename, bytes);
        }

        let (boundary, body) = multipart.finish();

        Box::new(send_method(
            self.client,
            &self.token,
            "sendVideo",
            Some(boundary),
            body,
        ))
    }
}

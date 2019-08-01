use super::*;
use crate::{
    errors,
    internal::{AsInnerRef, BoxFuture, Client},
    types::{
        input_file::{InputFile, Thumb, Video},
        keyboard, message,
        parameters::{ChatId, ImplicitChatId, NotificationState},
        value::Ref,
    },
};

/// Represents the [`sendVideo`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendvideo
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendVideo<'a, C> {
    client: &'a Client<C>,
    token: Token,
    chat_id: ChatId<'a>,
    video: Ref<'a, Video<'a>>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<message::Id>,
    reply_markup: Option<Ref<'a, keyboard::Any<'a>>>,
}

impl<'a, C> SendVideo<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl ImplicitChatId<'a>,
        video: impl Into<Ref<'a, Video<'a>>>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            video: video.into(),
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
        markup: impl Into<Ref<'a, keyboard::Any<'a>>>,
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
        let video = self.video.as_ref();
        let mut multipart = Multipart::new(12)
            .chat_id("chat_id", self.chat_id)
            .maybe_from("duration", video.duration)
            .maybe_from("width", video.width)
            .maybe_from("height", video.height)
            .maybe_str(
                "caption",
                match &video.caption {
                    Some(caption) => Some(caption.as_str()),
                    None => None,
                },
            )
            .maybe_json("parse_mode", video.parse_mode)
            .maybe_from("disable_notification", self.disable_notification)
            .maybe_from("supports_streaming", video.supports_streaming)
            .maybe_from("reply_to_message_id", self.reply_to_message_id)
            .maybe_json("reply_markup", self.reply_markup);

        match &video.media {
            InputFile::File {
                filename,
                bytes,
                ..
            } => multipart = multipart.file("video", filename, bytes),
            InputFile::Id(id) => {
                multipart = multipart.str("video", id.as_ref().0);
            }
            InputFile::Url(url) => {
                multipart = multipart.str("video", url);
            }
        }

        if let Some(Thumb(InputFile::File {
            filename,
            bytes,
            ..
        })) = video.thumb.as_inner_ref()
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

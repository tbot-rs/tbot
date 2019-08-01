use super::*;
use crate::{
    errors,
    internal::{AsInnerRef, BoxFuture, Client},
    types::{
        input_file::{InputFile, Thumb, VideoNote},
        keyboard, message,
        parameters::{ChatId, ImplicitChatId, NotificationState},
        value::Ref,
    },
};

/// Represents the [`sendVideoNote`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendvideonote
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendVideoNote<'a, C> {
    client: &'a Client<C>,
    token: Token,
    chat_id: ChatId<'a>,
    video_note: Ref<'a, VideoNote<'a>>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<message::Id>,
    reply_markup: Option<Ref<'a, keyboard::Any<'a>>>,
}

impl<'a, C> SendVideoNote<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl ImplicitChatId<'a>,
        video_note: impl Into<Ref<'a, VideoNote<'a>>>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            video_note: video_note.into(),
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

impl<C> IntoFuture for SendVideoNote<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = types::Message;
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        let video_note = self.video_note.as_ref();

        let mut multipart = Multipart::new(8)
            .chat_id("chat_id", self.chat_id)
            .maybe_from("duration", video_note.duration)
            .maybe_from("length", video_note.length)
            .maybe_from("disable_notification", self.disable_notification)
            .maybe_from("reply_to_message_id", self.reply_to_message_id)
            .maybe_json("reply_markup", self.reply_markup);

        match video_note.media.file.as_ref() {
            InputFile::File {
                filename,
                bytes,
                ..
            } => multipart = multipart.file("video_note", filename, bytes),
            InputFile::Id(id) => {
                multipart = multipart.str("video_note", id.as_ref().0);
            }
            InputFile::Url(url) => {
                multipart = multipart.str("video_note", url);
            }
        }

        if let Some(Thumb(InputFile::File {
            filename,
            bytes,
            ..
        })) = video_note.thumb.as_inner_ref()
        {
            multipart = multipart.file("thumb", filename, bytes);
        }

        let (boundary, body) = multipart.finish();

        Box::new(send_method(
            self.client,
            &self.token,
            "sendVideoNote",
            Some(boundary),
            body,
        ))
    }
}

use super::*;
use crate::internal::Client;
use std::sync::Arc;
use types::input_file::{InputFile, VideoNote};

/// Represents the [`sendVideoNote`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendvideonote
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendVideoNote<'a, C> {
    client: Arc<Client<C>>,
    token: Token,
    chat_id: types::ChatId<'a>,
    video_note: &'a VideoNote<'a>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<u32>,
    reply_markup: Option<types::AnyKeyboard<'a>>,
}

impl<'a, C> SendVideoNote<'a, C> {
    /// Constructs a new `SendVideoNote`.
    pub fn new(
        client: Arc<Client<C>>,
        token: Token,
        chat_id: impl Into<types::ChatId<'a>>,
        video_note: &'a VideoNote<'a>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            video_note,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Configures `disable_notification`.
    pub fn disable_notification(mut self, is_disabled: bool) -> Self {
        self.disable_notification = Some(is_disabled);
        self
    }

    /// Configures `reply_to_message_id`.
    pub fn reply_to_message_id(mut self, id: u32) -> Self {
        self.reply_to_message_id = Some(id);
        self
    }

    /// Configures `reply_markup`.
    pub fn reply_markup(
        mut self,
        markup: impl Into<types::AnyKeyboard<'a>>,
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
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = types::Message;
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        let chat_id = match self.chat_id {
            types::ChatId::Id(id) => id.to_string(),
            types::ChatId::Username(username) => username.into(),
        };

        let duration = self.video_note.duration.map(|x| x.to_string());
        let length = self.video_note.length.map(|x| x.to_string());
        let reply_to = self.reply_to_message_id.map(|id| id.to_string());
        let is_disabled = self.disable_notification.map(|x| x.to_string());
        let reply_markup =
            self.reply_markup.and_then(|x| serde_json::to_string(&x).ok());

        let mut multipart = Multipart::new(8)
            .str("chat_id", &chat_id)
            .maybe_string("duration", &duration)
            .maybe_string("length", &length)
            .maybe_string("disable_notification", &is_disabled)
            .maybe_string("reply_to_message_id", &reply_to)
            .maybe_string("reply_markup", &reply_markup);

        match self.video_note.media {
            InputFile::File {
                filename,
                bytes,
                ..
            } => multipart = multipart.file("video_note", filename, bytes),
            InputFile::Id(video_note) | InputFile::Url(video_note) => {
                multipart = multipart.str("video_note", video_note);
            }
        }

        if let Some(InputFile::File {
            filename,
            bytes,
            ..
        }) = self.video_note.thumb
        {
            multipart = multipart.file("thumb", filename, bytes);
        }

        let (boundary, body) = multipart.finish();

        Box::new(send_method(
            &self.client,
            &self.token,
            "sendVideoNote",
            Some(boundary),
            body,
        ))
    }
}

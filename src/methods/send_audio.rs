use super::*;
use crate::internal::Client;
use types::input_file::{Audio, InputFile};

/// Represents the [`sendAudio`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendaudio
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendAudio<'a, C> {
    client: &'a Client<C>,
    token: Token,
    chat_id: types::ChatId<'a>,
    audio: &'a Audio<'a>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<u32>,
    reply_markup: Option<types::AnyKeyboard<'a>>,
}

impl<'a, C> SendAudio<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl Into<types::ChatId<'a>>,
        audio: &'a Audio<'a>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            audio,
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

impl<C> IntoFuture for SendAudio<'_, C>
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

        let duration = self.audio.duration.map(|x| x.to_string());
        let parse_mode = self.audio.parse_mode.map(|x| x.to_string());
        let is_disabled = self.disable_notification.map(|x| x.to_string());
        let reply_to = self.reply_to_message_id.map(|id| id.to_string());
        let reply_markup = self
            .reply_markup
            .and_then(|markup| serde_json::to_string(&markup).ok());

        let mut multipart = Multipart::new(11)
            .str("chat_id", &chat_id)
            .maybe_string("duration", &duration)
            .maybe_str("caption", self.audio.caption)
            .maybe_str("performer", self.audio.performer)
            .maybe_str("title", self.audio.title)
            .maybe_string("parse_mode", &parse_mode)
            .maybe_string("disable_notification", &is_disabled)
            .maybe_string("reply_to_message_id", &reply_to)
            .maybe_string("reply_markup", &reply_markup);

        match self.audio.media {
            InputFile::File {
                filename,
                bytes,
                ..
            } => multipart = multipart.file("audio", filename, bytes),
            InputFile::Id(audio) | InputFile::Url(audio) => {
                multipart = multipart.str("audio", audio);
            }
        }

        if let Some(InputFile::File {
            filename,
            bytes,
            ..
        }) = self.audio.thumb
        {
            multipart = multipart.file("thumb", filename, bytes);
        }

        let (boundary, body) = multipart.finish();

        Box::new(send_method(
            &self.client,
            &self.token,
            "sendAudio",
            Some(boundary),
            body,
        ))
    }
}

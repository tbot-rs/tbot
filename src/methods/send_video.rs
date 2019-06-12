use super::*;
use types::input_file::{InputFile, Video};

/// Represents the [`sendVideo`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendvideo
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendVideo<'a> {
    token: Token,
    #[cfg(feature = "proxy")]
    proxy: Option<proxy::Proxy>,
    chat_id: types::ChatId<'a>,
    video: &'a Video<'a>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<u32>,
    reply_markup: Option<types::AnyKeyboard<'a>>,
}

impl<'a> SendVideo<'a> {
    /// Constructs a new `SendVideo`.
    pub fn new(
        token: Token,
        chat_id: impl Into<types::ChatId<'a>>,
        video: &'a Video<'a>,
    ) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            video,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
            #[cfg(feature = "proxy")]
            proxy: None,
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

impl IntoFuture for SendVideo<'_> {
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = types::Message;
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        let chat_id = match self.chat_id {
            types::ChatId::Id(id) => id.to_string(),
            types::ChatId::Username(username) => username.into(),
        };

        let duration = self.video.duration.map(|x| x.to_string());
        let width = self.video.width.map(|x| x.to_string());
        let height = self.video.height.map(|x| x.to_string());
        let parse_mode = self.video.parse_mode.map(|x| x.to_string());
        let is_disabled = self.disable_notification.map(|x| x.to_string());
        let is_streamed = self.video.supports_streaming.map(|x| x.to_string());
        let reply_to = self.reply_to_message_id.map(|id| id.to_string());
        let reply_markup = self
            .reply_markup
            .and_then(|markup| serde_json::to_string(&markup).ok());

        let mut multipart = Multipart::new(12)
            .str("chat_id", &chat_id)
            .maybe_string("duration", &duration)
            .maybe_string("width", &width)
            .maybe_string("height", &height)
            .maybe_str("caption", self.video.caption)
            .maybe_string("parse_mode", &parse_mode)
            .maybe_string("disable_notification", &is_disabled)
            .maybe_string("supports_streaming", &is_streamed)
            .maybe_string("reply_to_message_id", &reply_to)
            .maybe_string("reply_markup", &reply_markup);

        match self.video.media {
            InputFile::File {
                filename,
                bytes,
                ..
            } => multipart = multipart.file("video", filename, bytes),
            InputFile::Id(audio) | InputFile::Url(audio) => {
                multipart = multipart.str("video", audio);
            }
        }

        if let Some(InputFile::File {
            filename,
            bytes,
            ..
        }) = self.video.thumb
        {
            multipart = multipart.file("thumb", filename, bytes);
        }

        let (boundary, body) = multipart.finish();

        Box::new(send_method(
            &self.token,
            "sendVideo",
            Some(boundary),
            body,
            #[cfg(feature = "proxy")]
            self.proxy,
        ))
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for SendVideo<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}

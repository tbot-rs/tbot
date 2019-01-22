use super::*;

/// Represents the [`sendVoice`] method.
///
/// [`sendVoice`]: https://core.telegram.org/bots/api#sendvoice
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendVoice<'a> {
    token: &'a str,
    #[cfg(feature = "proxy")]
    proxy: Option<proxy::Proxy>,
    chat_id: types::ChatId<'a>,
    voice: types::Voice<'a>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<u64>,
    reply_markup: Option<types::raw::Keyboard<'a>>,
}

impl<'a> SendVoice<'a> {
    /// Constructs a new `SendVoice`.
    pub fn new<'b: 'a>(
        token: &'b str,
        chat_id: impl Into<types::ChatId<'b>>,
        voice: types::Voice<'a>,
    ) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            voice,
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
    pub fn reply_to_message_id(mut self, id: u64) -> Self {
        self.reply_to_message_id = Some(id);
        self
    }

    /// Configures `reply_markup`.
    pub fn reply_markup(
        mut self,
        markup: impl Into<types::raw::Keyboard<'a>>,
    ) -> Self {
        self.reply_markup = Some(markup.into());
        self
    }

    /// Prepares the request and returns a `Future`.
    #[must_use = "futures do nothing unless polled"]
    pub fn into_future(
        self,
    ) -> impl Future<Item = types::raw::Message, Error = DeliveryError> {
        let chat_id = match self.chat_id {
            types::ChatId::Id(id) => id.to_string(),
            types::ChatId::Username(username) => username.into(),
        };

        let duration = self.voice.duration.map(|x| x.to_string());
        let parse_mode = self.voice.parse_mode.map(|x| x.to_string());
        let reply_to = self.reply_to_message_id.map(|id| id.to_string());
        let is_disabled = self.disable_notification.map(|x| x.to_string());
        let reply_markup =
            self.reply_markup.and_then(|x| serde_json::to_string(&x).ok());

        let mut multipart = Multipart::new(8)
            .str("chat_id", &chat_id)
            .maybe_string("duration", &duration)
            .maybe_str("caption", self.voice.caption)
            .maybe_string("parse_mode", &parse_mode)
            .maybe_string("disable_notification", &is_disabled)
            .maybe_string("reply_to_message_id", &reply_to)
            .maybe_string("reply_markup", &reply_markup);

        match self.voice.media {
            types::InputFile::File {
                filename,
                bytes,
                ..
            } => multipart = multipart.file("voice", filename, bytes),
            types::InputFile::Id(voice) | types::InputFile::Url(voice) => {
                multipart = multipart.str("voice", voice);
            }
        }

        let (boundary, body) = multipart.finish();

        send_method(
            self.token,
            "sendVoice",
            Some(boundary),
            body,
            #[cfg(feature = "proxy")]
            self.proxy,
        )
    }
}

#[cfg(feature = "proxy")]
impl<'a> ProxyMethod for SendVoice<'a> {
    /// Configures `proxy`.
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}

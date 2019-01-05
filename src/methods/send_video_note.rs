use super::*;

/// Representation of the [`sendVideoNote`] method.
///
/// [`sendVideoNote`]: https://core.telegram.org/bots/api#sendvideonote
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendVideoNote<'a> {
    token: &'a str,
    #[cfg(feature = "proxy")]
    proxy: Option<proxy::Proxy>,
    chat_id: types::ChatId<'a>,
    video_note: types::VideoNote<'a>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<u64>,
    reply_markup: Option<types::raw::Keyboard<'a>>,
}

impl<'a> SendVideoNote<'a> {
    /// Constructs a new `SendVideoNote`.
    pub fn new<'b: 'a>(
        token: &'b str,
        chat_id: impl Into<types::ChatId<'b>>,
        video_note: types::VideoNote<'a>,
    ) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            video_note,
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
            types::InputFile::File {
                filename,
                bytes,
                ..
            } => multipart = multipart.file("video_note", filename, bytes),
            types::InputFile::Id(video_note)
            | types::InputFile::Url(video_note) => {
                multipart = multipart.str("video_note", video_note);
            }
        }

        if let Some(types::InputFile::File {
            filename,
            bytes,
            ..
        }) = self.video_note.thumb
        {
            multipart = multipart.file("thumb", filename, bytes);
        }

        let (boundary, body) = multipart.finish();

        send_method(
            self.token,
            "sendVideoNote",
            Some(boundary),
            body,
            #[cfg(feature = "proxy")]
            self.proxy,
        )
    }
}

#[cfg(feature = "proxy")]
impl<'a> ProxyMethod for SendVideoNote<'a> {
    /// Configures `proxy`.
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}

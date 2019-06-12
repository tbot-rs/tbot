use super::*;
use types::input_file::{Animation, InputFile};

/// Represents the [`sendAnimation`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendanimation
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendAnimation<'a> {
    token: &'a str,
    #[cfg(feature = "proxy")]
    proxy: Option<proxy::Proxy>,
    chat_id: types::ChatId<'a>,
    animation: &'a Animation<'a>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<u32>,
    reply_markup: Option<types::AnyKeyboard<'a>>,
}

impl<'a> SendAnimation<'a> {
    /// Constructs a new `SendAnimation`.
    pub fn new(
        token: &'a str,
        chat_id: impl Into<types::ChatId<'a>>,
        animation: &'a Animation<'a>,
    ) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            animation,
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

impl IntoFuture for SendAnimation<'_> {
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = types::Message;
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        let chat_id = match self.chat_id {
            types::ChatId::Id(id) => id.to_string(),
            types::ChatId::Username(username) => username.into(),
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
            self.token,
            "sendAnimation",
            Some(boundary),
            body,
            #[cfg(feature = "proxy")]
            self.proxy,
        ))
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for SendAnimation<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}

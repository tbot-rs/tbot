use super::*;
use types::input_file::*;

/// Represents the [`editMessageMedia`][docs] method for when the message was
/// sent by the bot.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagemedia
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditMessageMedia<'a> {
    token: &'a str,
    #[cfg(feature = "proxy")]
    proxy: Option<proxy::Proxy>,
    chat_id: types::ChatId<'a>,
    message_id: u64,
    media: EditableMedia<'a>,
    reply_markup: Option<types::InlineKeyboard<'a>>,
}

impl<'a> EditMessageMedia<'a> {
    /// Constructs a new `EditMessageMedia`.
    pub fn new(
        token: &'a str,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u64,
        media: impl Into<EditableMedia<'a>>,
    ) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            message_id,
            media: media.into(),
            reply_markup: None,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }

    /// Configures `reply_markup`.
    pub fn reply_markup(mut self, markup: types::InlineKeyboard<'a>) -> Self {
        self.reply_markup = Some(markup);
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
        let message_id = self.message_id.to_string();
        let reply_markup =
            self.reply_markup.and_then(|x| serde_json::to_string(&x).ok());

        let mut multipart = Multipart::new(4)
            .str("chat_id", &chat_id)
            .str("message_id", &message_id)
            .maybe_string("reply_markup", &reply_markup);

        match &self.media {
            EditableMedia::Animation(Animation {
                media,
                ..
            })
            | EditableMedia::Audio(Audio {
                media,
                ..
            })
            | EditableMedia::Document(Document {
                media,
                ..
            })
            | EditableMedia::Photo(Photo {
                media,
                ..
            })
            | EditableMedia::Video(Video {
                media,
                ..
            }) => {
                if let InputFile::File {
                    name,
                    filename,
                    bytes,
                } = media
                {
                    multipart = multipart.file(name, filename, bytes);
                }
            }
        }

        let media = serde_json::to_string(&self.media).unwrap();
        let (boundary, body) = multipart.str("media", &media).finish();

        send_method(
            self.token,
            "editMessageMedia",
            Some(boundary),
            body,
            #[cfg(feature = "proxy")]
            self.proxy,
        )
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for EditMessageMedia<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}

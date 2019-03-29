use super::*;

/// Represents the [`editMessageMedia`][docs] method for when the message was
/// sent via inline mode.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagemedia
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditInlineMedia<'a> {
    token: &'a str,
    #[cfg(feature = "proxy")]
    proxy: Option<proxy::Proxy>,
    inline_message_id: u64,
    media: types::EditableMedia<'a>,
    reply_markup: Option<types::InlineKeyboard<'a>>,
}

impl<'a> EditInlineMedia<'a> {
    /// Constructs a new `EditInlineMedia`.
    pub fn new(
        token: &'a str,
        inline_message_id: u64,
        media: impl Into<types::EditableMedia<'a>>,
    ) -> Self {
        Self {
            token,
            inline_message_id,
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
    pub fn into_future(self) -> impl Future<Item = (), Error = DeliveryError> {
        let inline_message_id = self.inline_message_id.to_string();
        let reply_markup =
            self.reply_markup.and_then(|x| serde_json::to_string(&x).ok());

        let mut multipart = Multipart::new(3)
            .str("inline_message_id", &inline_message_id)
            .maybe_string("reply_markup", &reply_markup);

        match &self.media {
            types::EditableMedia::Animation(types::Animation {
                media,
                ..
            })
            | types::EditableMedia::Audio(types::Audio {
                media,
                ..
            })
            | types::EditableMedia::Document(types::Document {
                media,
                ..
            })
            | types::EditableMedia::Photo(types::Photo {
                media,
                ..
            })
            | types::EditableMedia::Video(types::Video {
                media,
                ..
            }) => {
                if let types::InputFile::File {
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

        send_method::<bool>(
            self.token,
            "editMessageMedia",
            Some(boundary),
            body,
            #[cfg(feature = "proxy")]
            self.proxy,
        )
        .map(|_| ())
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for EditInlineMedia<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}

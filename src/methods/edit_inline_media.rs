use super::*;
use types::input_file::*;

/// Represents the [`editMessageMedia`][docs] method for inline messages.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagemedia
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditInlineMedia<'a> {
    token: Token,
    #[cfg(feature = "proxy")]
    proxy: Option<proxy::Proxy>,
    inline_message_id: &'a str,
    media: EditableMedia<'a>,
    reply_markup: Option<types::InlineKeyboard<'a>>,
}

impl<'a> EditInlineMedia<'a> {
    /// Constructs a new `EditInlineMedia`.
    pub fn new(
        token: Token,
        inline_message_id: &'a str,
        media: impl Into<EditableMedia<'a>>,
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
}

impl IntoFuture for EditInlineMedia<'_> {
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = ();
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        let reply_markup =
            self.reply_markup.and_then(|x| serde_json::to_string(&x).ok());

        let mut multipart = Multipart::new(4)
            .str("inline_message_id", self.inline_message_id)
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

        Box::new(
            send_method::<bool>(
                &self.token,
                "editMessageMedia",
                Some(boundary),
                body,
                #[cfg(feature = "proxy")]
                self.proxy,
            )
            .map(|_| ()),
        )
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for EditInlineMedia<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}

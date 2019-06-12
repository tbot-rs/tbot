use super::*;
use types::input_file::*;

/// Represents the [`sendMediaGroup`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendmediagroup
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendMediaGroup<'a> {
    token: Token,
    #[cfg(feature = "proxy")]
    proxy: Option<proxy::Proxy>,
    chat_id: types::ChatId<'a>,
    media: Vec<GroupMedia<'a>>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<u32>,
}

impl<'a> SendMediaGroup<'a> {
    /// Contructs a new `SendMediaGroup`.
    ///
    /// **Note:** unlike other methods, this one takes ownership of the media
    /// because it modifies the media's metadata, and thus further reuse of the
    /// media would lead to errors.
    pub fn new(
        token: Token,
        chat_id: impl Into<types::ChatId<'a>>,
        media: Vec<GroupMedia<'a>>,
    ) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            media,
            disable_notification: None,
            reply_to_message_id: None,
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
}

impl IntoFuture for SendMediaGroup<'_> {
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = Vec<types::Message>;
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        let chat_id = match self.chat_id {
            types::ChatId::Id(id) => id.to_string(),
            types::ChatId::Username(username) => username.into(),
        };

        let is_disabled = self.disable_notification.map(|x| x.to_string());
        let reply_to = self.reply_to_message_id.map(|id| id.to_string());

        let mut media = self.media;

        for (index, media) in media.iter_mut().enumerate() {
            if let GroupMedia::Photo(Photo {
                media:
                    InputFile::File {
                        ref mut name,
                        ..
                    },
                ..
            }) = media
            {
                *name = format!("photo_{}", index);
            }

            if let GroupMedia::Video(Video {
                media:
                    InputFile::File {
                        ref mut name,
                        ..
                    },
                thumb,
                ..
            }) = media
            {
                *name = format!("video_{}", index);

                if let Some(InputFile::File {
                    ref mut name,
                    ..
                }) = thumb
                {
                    *name = format!("thumb_{}", index);
                }
            }
        }

        let mut multipart = Multipart::new(4 + media.len())
            .str("chat_id", &chat_id)
            .maybe_string("disabled_notification", &is_disabled)
            .maybe_string("reply_to_message_id", &reply_to);

        for media in &media {
            match media {
                GroupMedia::Photo(Photo {
                    media:
                        InputFile::File {
                            name,
                            filename,
                            bytes,
                        },
                    ..
                })
                | GroupMedia::Video(Video {
                    media:
                        InputFile::File {
                            name,
                            filename,
                            bytes,
                        },
                    ..
                }) => {
                    multipart = multipart.file(name, filename, bytes);
                }
                _ => (),
            }
        }

        let media = serde_json::to_string(&media).unwrap();
        let (boundary, body) = multipart.str("media", &media).finish();

        Box::new(send_method(
            &self.token,
            "sendMediaGroup",
            Some(boundary),
            body,
            #[cfg(feature = "proxy")]
            self.proxy,
        ))
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for SendMediaGroup<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}

use super::*;
use types::input_file::{ChatPhoto, InputFile};

/// Represents the [`setChatPhoto`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setchatphoto
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetChatPhoto<'a> {
    token: Token,
    #[cfg(feature = "proxy")]
    proxy: Option<proxy::Proxy>,
    chat_id: types::ChatId<'a>,
    photo: &'a ChatPhoto<'a>,
}

impl<'a> SetChatPhoto<'a> {
    /// Constructs a new `SetChatPhoto`.
    pub fn new(
        token: Token,
        chat_id: impl Into<types::ChatId<'a>>,
        photo: &'a ChatPhoto<'a>,
    ) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            photo,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }
}

impl IntoFuture for SetChatPhoto<'_> {
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = ();
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        let chat_id = match self.chat_id {
            types::ChatId::Id(id) => id.to_string(),
            types::ChatId::Username(username) => username.into(),
        };

        let mut multipart = Multipart::new(2).str("chat_id", &chat_id);

        if let InputFile::File {
            filename,
            bytes,
            ..
        } = self.photo.0
        {
            multipart = multipart.file("photo", filename, bytes);
        }

        let (boundary, body) = multipart.finish();

        Box::new(
            send_method::<bool>(
                &self.token,
                "setChatPhoto",
                Some(boundary),
                body,
                #[cfg(feature = "proxy")]
                self.proxy,
            )
            .map(|_| ()), // Only `true` is returned on success
        )
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for SetChatPhoto<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}

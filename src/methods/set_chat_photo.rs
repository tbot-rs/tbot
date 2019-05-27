use super::*;
use types::input_file::{ChatPhoto, InputFile};

/// Represents the [`setChatPhoto`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setchatphoto
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetChatPhoto<'a> {
    token: &'a str,
    #[cfg(feature = "proxy")]
    proxy: Option<proxy::Proxy>,
    chat_id: types::ChatId<'a>,
    photo: &'a ChatPhoto<'a>,
}

impl<'a> SetChatPhoto<'a> {
    /// Constructs a new `SetChatPhoto`.
    pub fn new(
        token: &'a str,
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

    /// Prepares the request and returns a `Future`.
    #[must_use = "futures do nothing unless polled"]
    pub fn into_future(self) -> impl Future<Item = (), Error = DeliveryError> {
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

        send_method::<bool>(
            self.token,
            "setChatPhoto",
            Some(boundary),
            body,
            #[cfg(feature = "proxy")]
            self.proxy,
        )
        .map(|_| ()) // Only `true` is returned on success
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for SetChatPhoto<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}

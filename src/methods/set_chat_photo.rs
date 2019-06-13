use super::*;
use crate::internal::Client;
use types::input_file::{ChatPhoto, InputFile};

/// Represents the [`setChatPhoto`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setchatphoto
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetChatPhoto<'a, C> {
    client: &'a Client<C>,
    token: Token,
    chat_id: types::ChatId<'a>,
    photo: &'a ChatPhoto<'a>,
}

impl<'a, C> SetChatPhoto<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl Into<types::ChatId<'a>>,
        photo: &'a ChatPhoto<'a>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            photo,
        }
    }
}

impl<C> IntoFuture for SetChatPhoto<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
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
            send_method::<bool, C>(
                &self.client,
                &self.token,
                "setChatPhoto",
                Some(boundary),
                body,
            )
            .map(|_| ()), // Only `true` is returned on success
        )
    }
}

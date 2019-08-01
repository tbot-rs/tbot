use super::*;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    types::{
        input_file::{ChatPhoto, InputFile},
        parameters::{ChatId, ImplicitChatId},
        value::Ref,
    },
};

/// Represents the [`setChatPhoto`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setchatphoto
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetChatPhoto<'a, C> {
    client: &'a Client<C>,
    token: Token,
    chat_id: ChatId<'a>,
    photo: Ref<'a, ChatPhoto<'a>>,
}

impl<'a, C> SetChatPhoto<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl ImplicitChatId<'a>,
        photo: impl Into<Ref<'a, ChatPhoto<'a>>>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            photo: photo.into(),
        }
    }
}

impl<C> IntoFuture for SetChatPhoto<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = ();
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        let chat_id = match self.chat_id {
            ChatId::Id(id) => id.to_string().into(),
            ChatId::Username(username) => username,
        };

        let mut multipart = Multipart::new(2).str("chat_id", chat_id);

        if let InputFile::File {
            filename,
            bytes,
            ..
        } = self.photo.as_ref().0.file.as_ref()
        {
            multipart = multipart.file("photo", filename, bytes);
        }

        let (boundary, body) = multipart.finish();

        Box::new(
            send_method::<bool, C>(
                self.client,
                &self.token,
                "setChatPhoto",
                Some(boundary),
                body,
            )
            .map(|_| ()), // Only `true` is returned on success
        )
    }
}

use super::*;
use crate::{
    connectors::Connector,
    errors,
    internal::Client,
    types::{
        input_file::{ChatPhoto, InputFile},
        parameters::{ChatId, ImplicitChatId},
    },
};

/// Sets a chat's photo.
///
/// Reflects the [`setChatPhoto`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setchatphoto
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetChatPhoto<'a, C> {
    client: &'a Client<C>,
    token: Token,
    chat_id: ChatId<'a>,
    photo: ChatPhoto<'a>,
}

impl<'a, C> SetChatPhoto<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl ImplicitChatId<'a>,
        photo: ChatPhoto<'a>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            photo,
        }
    }
}

impl<C: Connector> SetChatPhoto<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        let chat_id = match self.chat_id {
            ChatId::Id(id) => id.to_string(),
            ChatId::Username(username) => username.into(),
        };

        let mut multipart = Multipart::new(2).str("chat_id", &chat_id);

        if let InputFile::File {
            filename, bytes, ..
        } = self.photo.0.file
        {
            multipart = multipart.file("photo", filename, bytes);
        }

        let (boundary, body) = multipart.finish();

        send_method::<bool, _>(
            self.client,
            &self.token,
            "setChatPhoto",
            Some(boundary),
            body,
        )
        .await?;

        Ok(())
    }
}

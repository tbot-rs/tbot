use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{
        input_file::{ChatPhoto, InputFile},
        parameters::{ChatId, ImplicitChatId},
    },
    Multipart,
};

/// Sets a chat's photo.
///
/// Reflects the [`setChatPhoto`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setchatphoto
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetChatPhoto<'a> {
    bot: &'a InnerBot,
    chat_id: ChatId,
    photo: ChatPhoto<'a>,
}

impl<'a> SetChatPhoto<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId,
        photo: ChatPhoto<'a>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            photo,
        }
    }
}

impl SetChatPhoto<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        let chat_id = match self.chat_id {
            ChatId::Id(id) => id.to_string(),
            ChatId::Username(username) => username,
        };

        let mut multipart = Multipart::new(2).str("chat_id", &chat_id);

        if let InputFile::File {
            filename, bytes, ..
        } = &self.photo.0
        {
            multipart = multipart.file("photo", filename, bytes);
        }

        let (boundary, body) = multipart.finish();

        call_method::<bool>(self.bot, "setChatPhoto", Some(boundary), body)
            .await?;

        Ok(())
    }
}

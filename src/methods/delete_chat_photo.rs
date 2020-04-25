use super::send_method;
use crate::{
    connectors::Client,
    errors, token,
    types::parameters::{ChatId, ImplicitChatId},
};
use serde::Serialize;

/// Deletes a chat's photo.
///
/// Reflects the [`deleteChatPhoto`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#deletechatphoto
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct DeleteChatPhoto<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
}

impl<'a> DeleteChatPhoto<'a> {
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        chat_id: impl ImplicitChatId<'a>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
        }
    }
}

impl DeleteChatPhoto<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        send_method::<bool>(
            self.client,
            self.token,
            "deleteChatPhoto",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}

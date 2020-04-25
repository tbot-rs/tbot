use super::call_method;
use crate::{
    connectors::Client,
    errors, token,
    types::parameters::{ChatId, ImplicitChatId},
};
use serde::Serialize;

/// Sets a chat's description.
///
/// Reflects the [`setChatDescription`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setchatdescription
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetChatDescription<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
    description: &'a str,
}

impl<'a> SetChatDescription<'a> {
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        chat_id: impl ImplicitChatId<'a>,
        description: &'a str,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            description,
        }
    }
}

impl SetChatDescription<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.client,
            self.token,
            "setChatDescription",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}

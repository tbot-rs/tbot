use super::send_method;
use crate::{
    connectors::Connector,
    errors,
    internal::Client,
    types::parameters::{ChatId, ImplicitChatId},
    token,
};
use serde::Serialize;

/// Sets a chat's description.
///
/// Reflects the [`setChatDescription`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setchatdescription
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetChatDescription<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
    description: &'a str,
}

impl<'a, C> SetChatDescription<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
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

impl<C: Connector> SetChatDescription<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        send_method::<bool, _>(
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

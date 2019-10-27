use super::send_method;
use crate::{
    connectors::Connector,
    errors,
    internal::Client,
    types::parameters::{ChatId, ImplicitChatId},
    token,
};
use serde::Serialize;

/// Leaves a chat.
///
/// Reflects the [`leaveChat`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#leavechat
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct LeaveChat<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
}

impl<'a, C> LeaveChat<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
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

impl<C: Connector> LeaveChat<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        send_method::<bool, _>(
            self.client,
            self.token,
            "leaveChat",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}

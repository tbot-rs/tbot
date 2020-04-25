use super::call_method;
use crate::{
    connectors::Client,
    errors, token,
    types::{
        chat,
        parameters::{ChatId, ImplicitChatId},
    },
};
use serde::Serialize;

/// Sets a group's global permissions.
///
/// Reflects the [`setChatPermissions`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setchatpermissions
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetChatPermissions<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
    permissions: chat::Permissions,
}

impl<'a> SetChatPermissions<'a> {
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        chat_id: impl ImplicitChatId<'a>,
        permissions: chat::Permissions,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            permissions,
        }
    }
}

impl SetChatPermissions<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.client,
            self.token,
            "setChatPermissions",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}

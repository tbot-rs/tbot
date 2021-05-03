use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
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
    bot: &'a InnerBot,
    chat_id: ChatId,
    permissions: chat::Permissions,
}

impl<'a> SetChatPermissions<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId,
        permissions: chat::Permissions,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            permissions,
        }
    }
}

impl SetChatPermissions<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.bot,
            "setChatPermissions",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}

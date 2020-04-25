use super::send_method;
use crate::{
    connectors::Client,
    errors, token,
    types::{
        parameters::{ChatId, ImplicitChatId},
        user,
    },
};
use serde::Serialize;

/// Sets a custom title for an admin in a supergroup promoted by the bot.
///
/// Reflects the [`setChatAdministratorCustomTitle`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setchatadministratorcustomtitle
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetChatAdministratorCustomTitle<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
    user_id: user::Id,
    custom_title: &'a str,
}

impl<'a> SetChatAdministratorCustomTitle<'a> {
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        chat_id: impl ImplicitChatId<'a>,
        user_id: user::Id,
        custom_title: &'a str,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            user_id,
            custom_title,
        }
    }
}

impl SetChatAdministratorCustomTitle<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        send_method::<bool>(
            self.client,
            self.token,
            "setChatAdministratorCustomTitle",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}

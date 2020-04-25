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

/// Lifts all restrictions from a group's member.
///
/// Reflects the [`unbanChatMember`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#unbanchatmember
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct UnbanChatMember<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
    user_id: user::Id,
}

impl<'a> UnbanChatMember<'a> {
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        chat_id: impl ImplicitChatId<'a>,
        user_id: user::Id,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            user_id,
        }
    }
}

impl UnbanChatMember<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        send_method::<bool>(
            &*self.client,
            self.token,
            "unbanChatMember",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}

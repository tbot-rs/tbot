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

/// Creates a secondary invite link for a chat.
///
/// Reflects the [`createChatInviteLink`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#createchatinvitelink
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct CreateChatInviteLink<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member_limit: Option<u32>,
}

impl<'a> CreateChatInviteLink<'a> {
    pub(crate) fn new(bot: &'a InnerBot, chat_id: impl ImplicitChatId) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            expire_date: None,
            member_limit: None,
        }
    }

    /// Configures the time when the link expires.
    /// Reflects the `expire_date` parameter.
    pub const fn expire_date(mut self, date: i64) -> Self {
        self.expire_date = Some(date);
        self
    }

    /// Configures how many users may be chat members at the same time if
    /// they joined via this link. Must be in range `1..100_000`.
    /// Reflects the `member_limit` parameter.
    pub fn member_limit(mut self, limit: u32) -> Self {
        assert!(
            (1..=100_000).contains(&limit),
            "[tbot] Received invalid `limit` in \
             `CreateChatInviteLink::member_limit`: \
             {}, must be in range `1..100_000`",
            limit,
        );

        self.member_limit = Some(limit);
        self
    }
}

impl CreateChatInviteLink<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<chat::InviteLink, errors::MethodCall> {
        call_method(
            self.bot,
            "createChatInviteLink",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}

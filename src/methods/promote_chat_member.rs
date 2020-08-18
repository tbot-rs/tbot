use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{
        parameters::{ChatId, ImplicitChatId},
        user,
    },
};
use serde::Serialize;

/// Promotes a chat member to an admin.
///
/// Reflects the [`promoteChatMember`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#promotechatmember
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct PromoteChatMember<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId<'a>,
    user_id: user::Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_change_info: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_post_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_edit_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_delete_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_invite_users: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_restrict_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_pin_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_promote_members: Option<bool>,
}

impl<'a> PromoteChatMember<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId<'a>,
        user_id: user::Id,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            user_id,
            can_change_info: None,
            can_post_messages: None,
            can_edit_messages: None,
            can_delete_messages: None,
            can_invite_users: None,
            can_restrict_members: None,
            can_pin_messages: None,
            can_promote_members: None,
        }
    }

    /// Configures if the user will be able to change the group's information.
    /// Reflects the `can_change_info` parameter.
    pub const fn can_change_info(mut self, can_change: bool) -> Self {
        self.can_change_info = Some(can_change);
        self
    }

    /// Configures if the user will be able to post messages, if the chat is
    /// a channel. Reflects the `can_post_messages` parameter.
    pub const fn can_post_messages(mut self, can_post: bool) -> Self {
        self.can_post_messages = Some(can_post);
        self
    }

    /// Configures if the user will be able to edit messages, if the chat is
    /// a channel. Reflects the `can_edit_messages` parameter.
    pub const fn can_edit_messages(mut self, can_edit: bool) -> Self {
        self.can_edit_messages = Some(can_edit);
        self
    }

    /// Configures if the user will be able to delete messages.
    /// Reflects the `can_delete_messages` parameter.
    pub const fn can_delete_messages(mut self, can_delete: bool) -> Self {
        self.can_delete_messages = Some(can_delete);
        self
    }

    /// Configures if the user will be able to invite new users.
    /// Reflects the `can_invite_users` parameter.
    pub const fn can_invite_users(mut self, can_invite: bool) -> Self {
        self.can_invite_users = Some(can_invite);
        self
    }

    /// Configures if the user will be able to restrict members.
    /// Reflects the `can_restrict_members` parameter.
    pub const fn can_restrict_members(mut self, can_restrict: bool) -> Self {
        self.can_restrict_members = Some(can_restrict);
        self
    }

    /// Configures if the user will be able to pin messages.
    /// Reflects the `can_pin_messages` parameter.
    pub const fn can_pin_messages(mut self, can_pin: bool) -> Self {
        self.can_pin_messages = Some(can_pin);
        self
    }

    /// Configures if the user will be able to promote other members.
    /// Reflects the `can_promote_members` parameter.
    pub const fn can_promote_members(mut self, can_promote: bool) -> Self {
        self.can_promote_members = Some(can_promote);
        self
    }
}

impl PromoteChatMember<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.bot,
            "promoteChatMember",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}

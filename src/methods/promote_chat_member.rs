use super::*;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    types::{
        parameters::{ChatId, ImplicitChatId},
        user,
    },
};

/// Represents the [`promoteChatMember`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#promotechatmember
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct PromoteChatMember<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
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

impl<'a, C> PromoteChatMember<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl ImplicitChatId<'a>,
        user_id: user::Id,
    ) -> Self {
        Self {
            client,
            token,
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

    /// Configures `can_change_info`.
    pub fn can_change_info(mut self, can_change: bool) -> Self {
        self.can_change_info = Some(can_change);
        self
    }

    /// Configures `can_post_messages`.
    pub fn can_post_messages(mut self, can_post: bool) -> Self {
        self.can_post_messages = Some(can_post);
        self
    }

    /// Configures `can_edit_messages`.
    pub fn can_edit_messages(mut self, can_edit: bool) -> Self {
        self.can_edit_messages = Some(can_edit);
        self
    }

    /// Configures `can_delete_messages`.
    pub fn can_delete_messages(mut self, can_delete: bool) -> Self {
        self.can_delete_messages = Some(can_delete);
        self
    }

    /// Configures `can_invite_users`.
    pub fn can_invite_users(mut self, can_invite: bool) -> Self {
        self.can_invite_users = Some(can_invite);
        self
    }

    /// Configures `can_restrict_members`.
    pub fn can_restrict_members(mut self, can_restrict: bool) -> Self {
        self.can_restrict_members = Some(can_restrict);
        self
    }

    /// Configures `can_pin_messages`.
    pub fn can_pin_messages(mut self, can_pin: bool) -> Self {
        self.can_pin_messages = Some(can_pin);
        self
    }

    /// Configures `can_promote_members`.
    pub fn can_promote_members(mut self, can_promote: bool) -> Self {
        self.can_promote_members = Some(can_promote);
        self
    }
}

impl<C> IntoFuture for PromoteChatMember<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = ();
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        Box::new(
            send_method::<bool, C>(
                self.client,
                &self.token,
                "promoteChatMember",
                None,
                serde_json::to_vec(&self).unwrap(),
            )
            .map(|_| ()), // Only `true` is returned on success
        )
    }
}

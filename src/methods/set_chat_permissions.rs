use super::*;
use crate::{
    connectors::Connector,
    errors,
    internal::{BoxFuture, Client},
    types::{
        chat,
        parameters::{ChatId, ImplicitChatId},
    },
};

/// Sets a group's global permissions.
///
/// Reflects the [`setChatPermissions`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setchatpermissions
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetChatPermissions<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    chat_id: ChatId<'a>,
    permissions: chat::Permissions,
}

impl<'a, C> SetChatPermissions<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
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

impl<C: Connector> IntoFuture for SetChatPermissions<'_, C> {
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = ();
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        Box::new(
            send_method::<bool, C>(
                self.client,
                &self.token,
                "setChatPermissions",
                None,
                serde_json::to_vec(&self).unwrap(),
            )
            .map(|_| ()), // Only `true` is returned on success
        )
    }
}

use super::*;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    types::{
        parameters::{ChatId, ImplicitChatId},
        value,
    },
};

/// Represents the [`setChatDescription`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setchatdescription
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetChatDescription<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    chat_id: ChatId<'a>,
    description: value::String<'a>,
}

impl<'a, C> SetChatDescription<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl ImplicitChatId<'a>,
        description: impl Into<value::String<'a>>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            description: description.into(),
        }
    }
}

impl<C> IntoFuture for SetChatDescription<'_, C>
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
                "setChatDescription",
                None,
                serde_json::to_vec(&self).unwrap(),
            )
            .map(|_| ()), // Only `true` is returned on success
        )
    }
}

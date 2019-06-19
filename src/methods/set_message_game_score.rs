use super::*;
use crate::{
    internal::{BoxFuture, Client},
    types::{message, parameters::ChatId, user},
};

/// Represents the [`setGameScore`][docs] method for chat messages.
///
/// [docs]: https://core.telegram.org/bots/api#setgamescore
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetMessageGameScore<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    user_id: user::Id,
    score: u32,
    chat_id: ChatId<'a>,
    message_id: message::Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    force: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_edit_message: Option<bool>,
}

impl<'a, C> SetMessageGameScore<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl Into<ChatId<'a>>,
        message_id: message::Id,
        user_id: user::Id,
        score: u32,
    ) -> Self {
        Self {
            client,
            token,
            user_id,
            score,
            chat_id: chat_id.into(),
            message_id,
            force: None,
            disable_edit_message: None,
        }
    }

    /// Configures `force`.
    pub fn force(mut self, is_forced: bool) -> Self {
        self.force = Some(is_forced);
        self
    }

    /// Configures `disable_edit_message`.
    pub fn disable_edit_message(mut self, is_disabled: bool) -> Self {
        self.disable_edit_message = Some(is_disabled);
        self
    }
}

impl<C> IntoFuture for SetMessageGameScore<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = types::Message;
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            self.client,
            &self.token,
            "setGameScore",
            None,
            serde_json::to_vec(&self).unwrap(),
        ))
    }
}

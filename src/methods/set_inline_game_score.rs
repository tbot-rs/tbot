use super::*;
use crate::internal::Client;

/// Represents the [`setGameScore`][docs] method for inline messages.
///
/// [docs]: https://core.telegram.org/bots/api#setgamescore
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetInlineGameScore<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    user_id: i64,
    score: u32,
    inline_message_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    force: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_edit_message: Option<bool>,
}

impl<'a, C> SetInlineGameScore<'a, C> {
    pub(crate) const fn new(
        client: &'a Client<C>,
        token: Token,
        inline_message_id: &'a str,
        user_id: i64,
        score: u32,
    ) -> Self {
        Self {
            client,
            token,
            user_id,
            score,
            inline_message_id,
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

impl<C> IntoFuture for SetInlineGameScore<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = ();
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(
            send_method::<bool, C>(
                self.client,
                &self.token,
                "setGameScore",
                None,
                serde_json::to_vec(&self).unwrap(),
            )
            .map(|_| ()), // Only `true` is returned on success
        )
    }
}

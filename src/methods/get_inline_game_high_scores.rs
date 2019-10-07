use super::*;
use crate::{
    connectors::Connector,
    errors,
    internal::{BoxFuture, Client},
    types::{game::HighScore, inline_message_id, user},
};

/// Gets an excerpt from the high score table of a game sent via the inline
/// mode.
///
/// Reflects the [`getGameHighScores`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getgamehighscores
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetInlineGameHighScores<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    user_id: user::Id,
    inline_message_id: inline_message_id::Ref<'a>,
}

impl<'a, C> GetInlineGameHighScores<'a, C> {
    pub(crate) const fn new(
        client: &'a Client<C>,
        token: Token,
        inline_message_id: inline_message_id::Ref<'a>,
        user_id: user::Id,
    ) -> Self {
        Self {
            client,
            token,
            user_id,
            inline_message_id,
        }
    }
}

impl<C: Connector> IntoFuture for GetInlineGameHighScores<'_, C> {
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = Vec<HighScore>;
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            self.client,
            &self.token,
            "getGameHighScores",
            None,
            serde_json::to_vec(&self).unwrap(),
        ))
    }
}

use super::*;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    types::{game::HighScore, user, value::InlineMessageId},
};

/// Represents the [`getGameHighScores`][docs] method for inline messages.
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
    inline_message_id: InlineMessageId<'a>,
}

impl<'a, C> GetInlineGameHighScores<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        inline_message_id: impl Into<InlineMessageId<'a>>,
        user_id: user::Id,
    ) -> Self {
        Self {
            client,
            token,
            user_id,
            inline_message_id: inline_message_id.into(),
        }
    }
}

impl<C> IntoFuture for GetInlineGameHighScores<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
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

use super::*;
use crate::internal::Client;

type HighScores = Vec<types::GameHighScore>;

/// Represents the [`getGameHighScores`][docs] method for chat messages.
///
/// [docs]: https://core.telegram.org/bots/api#getgamehighscores
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetMessageGameHighScores<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    user_id: i64,
    chat_id: types::ChatId<'a>,
    message_id: u32,
}

impl<'a, C> GetMessageGameHighScores<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
        user_id: i64,
    ) -> Self {
        Self {
            client,
            token,
            user_id,
            chat_id: chat_id.into(),
            message_id,
        }
    }
}

impl<C> IntoFuture for GetMessageGameHighScores<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = HighScores;
    type Error = DeliveryError;

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

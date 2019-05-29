use super::*;

type HighScores = Vec<types::raw::GameHighScore>;

/// Represents the [`getGameHighScores`][docs] method for when the game was
/// sent by the bot.
///
/// [docs]: https://core.telegram.org/bots/api#getgamehighscores
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetMessageGameHighScores<'a> {
    #[serde(skip)]
    token: &'a str,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
    user_id: i64,
    chat_id: types::ChatId<'a>,
    message_id: u32,
}

impl<'a> GetMessageGameHighScores<'a> {
    /// Constructs a new `GetMessageGameHighScores`.
    pub fn new(
        token: &'a str,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
        user_id: i64,
    ) -> Self {
        Self {
            token,
            user_id,
            chat_id: chat_id.into(),
            message_id,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }

    /// Prepares the request and returns a `Future`.
    #[must_use = "futures do nothing unless polled"]
    pub fn into_future(
        self,
    ) -> impl Future<Item = HighScores, Error = DeliveryError> {
        send_method(
            self.token,
            "getGameHighScores",
            None,
            serde_json::to_vec(&self).unwrap(),
            #[cfg(feature = "proxy")]
            self.proxy,
        )
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for GetMessageGameHighScores<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}

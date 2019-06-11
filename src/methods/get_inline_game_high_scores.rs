use super::*;

type HighScores = Vec<types::GameHighScore>;

/// Represents the [`getGameHighScores`][docs] method for inline messages.
///
/// [docs]: https://core.telegram.org/bots/api#getgamehighscores
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetInlineGameHighScores<'a> {
    #[serde(skip)]
    token: &'a str,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
    user_id: i64,
    inline_message_id: &'a str,
}

impl<'a> GetInlineGameHighScores<'a> {
    /// Constructs a new `GetInlineGameHighScores`.
    pub const fn new(
        token: &'a str,
        inline_message_id: &'a str,
        user_id: i64,
    ) -> Self {
        Self {
            token,
            user_id,
            inline_message_id,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }
}

impl IntoFuture for GetInlineGameHighScores<'_> {
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = HighScores;
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            self.token,
            "getGameHighScores",
            None,
            serde_json::to_vec(&self).unwrap(),
            #[cfg(feature = "proxy")]
            self.proxy,
        ))
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for GetInlineGameHighScores<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}

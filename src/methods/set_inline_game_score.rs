use super::*;

/// Represents the [`setGameScore`][docs] method for when the game was
/// sent via the inline mode.
///
/// [docs]: https://core.telegram.org/bots/api#setgamescore
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetInlineGameScore<'a> {
    #[serde(skip)]
    token: &'a str,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
    user_id: i64,
    score: u32,
    inline_message_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    force: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_edit_message: Option<bool>,
}

impl<'a> SetInlineGameScore<'a> {
    /// Constructs a new `SetInlineGameScore`.
    pub const fn new(
        token: &'a str,
        inline_message_id: &'a str,
        user_id: i64,
        score: u32,
    ) -> Self {
        Self {
            token,
            user_id,
            score,
            inline_message_id,
            force: None,
            disable_edit_message: None,
            #[cfg(feature = "proxy")]
            proxy: None,
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

    /// Prepares the request and returns a `Future`.
    #[must_use = "futures do nothing unless polled"]
    pub fn into_future(self) -> impl Future<Item = (), Error = DeliveryError> {
        send_method::<bool>(
            self.token,
            "setGameScore",
            None,
            serde_json::to_vec(&self).unwrap(),
            #[cfg(feature = "proxy")]
            self.proxy,
        )
        .map(|_| ()) //
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for SetInlineGameScore<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}

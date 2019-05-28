use super::*;

/// Represents the [`sendGame`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendgame
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendGame<'a> {
    #[serde(skip)]
    token: &'a str,
    #[serde(skip)]
    #[cfg(feature = "proxy")]
    proxy: Option<proxy::Proxy>,
    chat_id: types::ChatId<'a>,
    game_short_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<types::AnyKeyboard<'a>>,
}

impl<'a> SendGame<'a> {
    /// Constructs a new `SendGame`.
    pub fn new(
        token: &'a str,
        chat_id: impl Into<types::ChatId<'a>>,
        game_short_name: &'a str,
    ) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            game_short_name,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }

    /// Configures `disable_notification`.
    pub fn disable_notification(mut self, is_disabled: bool) -> Self {
        self.disable_notification = Some(is_disabled);
        self
    }

    /// Configures `reply_to_message_id`.
    pub fn reply_to_message_id(mut self, id: u32) -> Self {
        self.reply_to_message_id = Some(id);
        self
    }

    /// Configures `reply_markup`.
    pub fn reply_markup(
        mut self,
        markup: impl Into<types::AnyKeyboard<'a>>,
    ) -> Self {
        self.reply_markup = Some(markup.into());
        self
    }

    /// Prepares the request and returns a `Future`.
    #[must_use = "futures do nothing unless polled"]
    pub fn into_future(
        self,
    ) -> impl Future<Item = types::Message, Error = DeliveryError> {
        send_method(
            self.token,
            "sendGame",
            None,
            serde_json::to_vec(&self).unwrap(),
            #[cfg(feature = "proxy")]
            self.proxy,
        )
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for SendGame<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}

use super::*;

/// Representation of the [`stopMessageLiveLocation`] method for the case if
/// the message was sent via the inline mode.
///
/// [`stopMessageLiveLocation`]: https://core.telegram.org/bots/api#stopmessagelivelocation
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct StopInlineLocation<'a> {
    #[serde(skip)]
    token: &'a str,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
    inline_message_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<types::InlineKeyboard<'a>>,
}

impl<'a> StopInlineLocation<'a> {
    /// Constructs a new `StopInlineLocation`.
    pub fn new<'b: 'a>(token: &'b str, inline_message_id: u64) -> Self {
        Self {
            token,
            inline_message_id,
            reply_markup: None,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }

    /// Configures `reply_markup`.
    pub fn reply_markup(mut self, markup: types::InlineKeyboard<'a>) -> Self {
        self.reply_markup = Some(markup);
        self
    }

    /// Prepares the request and returns a `Future`.
    #[must_use = "futures do nothing unless polled"]
    pub fn into_future(self) -> impl Future<Item = (), Error = DeliveryError> {
        send_method::<bool>(
            self.token,
            "stopMessageLiveLocation",
            None,
            serde_json::to_vec(&self).unwrap(),
            #[cfg(feature = "proxy")]
            self.proxy,
        )
        // The only value `true` is returned on success.
        .map(|_| ())
    }
}

#[cfg(feature = "proxy")]
impl<'a> ProxyMethod for StopInlineLocation<'a> {
    /// Configures `proxy`.
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}

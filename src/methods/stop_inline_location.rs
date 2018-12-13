use super::*;

/// Representation of the [`stopMessageLiveLocation`] method for the case if
/// the message was sent via the inline mode.
///
/// [`stopMessageLiveLocation`]: https://core.telegram.org/bots/api#stopmessagelivelocation
#[derive(Serialize)]
pub struct StopInlineLocation<'a> {
    #[serde(skip)]
    token: &'a str,
    inline_message_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<types::InlineKeyboard<'a>>,
}

impl<'a> StopInlineLocation<'a> {
    /// Constructs a new `StopInlineLocation`.
    #[must_use]
    pub fn new<'b: 'a>(token: &'b str, inline_message_id: u64) -> Self {
        Self {
            token,
            inline_message_id,
            reply_markup: None,
        }
    }

    /// Sets `reply_markup` to `Some(markup)`.
    #[must_use]
    pub fn reply_markup(mut self, markup: types::InlineKeyboard<'a>) -> Self {
        self.reply_markup = Some(markup);
        self
    }

    /// Prepares the request and returns a `Future`.
    #[must_use]
    pub fn into_future(self) -> impl Future<Item = (), Error = DeliveryError> {
        send_method::<bool>(
            self.token,
            "stopMessageLiveLocation",
            None,
            serde_json::to_vec(&self).unwrap(),
            // It returns only `true` if suceess, handling it is meaningless.
        )
        .map(|_| ())
    }
}

use super::*;

/// Representation of the [`stopMessageLiveLocation`] method for the case if
/// the message was sent by the bot.
///
/// [`stopMessageLiveLocation`]: https://core.telegram.org/bots/api#stopmessagelivelocation
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct StopMessageLocation<'a> {
    #[serde(skip)]
    token: &'a str,
    chat_id: types::ChatId<'a>,
    message_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<types::InlineKeyboard<'a>>,
}

impl<'a> StopMessageLocation<'a> {
    /// Constructs a new `StopMessageLocation`.
    pub fn new<'b: 'a>(
        token: &'b str,
        chat_id: impl Into<types::ChatId<'b>>,
        message_id: u64,
    ) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            message_id,
            reply_markup: None,
        }
    }

    /// Sets `reply_markup` to `Some(markup)`.
    pub fn reply_markup(mut self, markup: types::InlineKeyboard<'a>) -> Self {
        self.reply_markup = Some(markup);
        self
    }

    /// Prepares the request and returns a `Future`.
    #[must_use = "futures do nothing unless polled"]
    pub fn into_future(
        self,
    ) -> impl Future<Item = types::raw::Message, Error = DeliveryError> {
        send_method::<types::raw::Message>(
            self.token,
            "stopMessageLiveLocation",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
    }
}

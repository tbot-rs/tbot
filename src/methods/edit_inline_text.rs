use super::*;

/// Representation of the [`editMessageText`] method for the case if
/// the message was sent by the bot via the inline mode.
///
/// [`editMessageText`]: https://core.telegram.org/bots/api#editmessagetext
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditInlineText<'a> {
    #[serde(skip)]
    token: &'a str,
    inline_message_id: u64,
    text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<types::ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<types::InlineKeyboard<'a>>,
}

impl<'a> EditInlineText<'a> {
    /// Constructs a new `EditMessageText`.
    pub fn new<'b: 'a>(
        token: &'b str,
        inline_message_id: u64,
        text: &'a str,
    ) -> Self {
        Self {
            token,
            inline_message_id,
            text,
            parse_mode: None,
            disable_web_page_preview: None,
            reply_markup: None,
        }
    }

    /// Configures `parse_mode`.
    pub fn parse_mode(mut self, mode: types::ParseMode) -> Self {
        self.parse_mode = Some(mode);
        self
    }

    /// Configures `disable_web_page_preview`.
    pub fn disable_web_page_preview(mut self, is_disabled: bool) -> Self {
        self.disable_web_page_preview = Some(is_disabled);
        self
    }

    /// Configures `reply_markup`.
    pub fn reply_markup(mut self, markup: types::InlineKeyboard<'a>) -> Self {
        self.reply_markup = Some(markup.into());
        self
    }

    /// Prepares the request and returns a `Future`.
    #[must_use = "futures do nothing unless polled"]
    pub fn into_future(self) -> impl Future<Item = (), Error = DeliveryError> {
        send_method::<bool>(
            self.token,
            "editMessageText",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .map(|_| ())
    }
}

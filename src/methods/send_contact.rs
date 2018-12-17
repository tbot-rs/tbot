use super::*;

/// Representation of the [`sendContact`] method.
///
/// [`sendContact`]: https://core.telegram.org/bots/api#sendcontact
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendContact<'a> {
    #[serde(skip)]
    token: &'a str,
    chat_id: types::ChatId<'a>,
    phone_number: &'a str,
    first_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcard: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<types::raw::Keyboard<'a>>,
}

impl<'a> SendContact<'a> {
    /// Constructs a new `SendContact`.
    pub fn new<'b: 'a>(
        token: &'b str,
        chat_id: impl Into<types::ChatId<'b>>,
        phone_number: &'a str,
        first_name: &'a str,
    ) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            phone_number,
            first_name,
            last_name: None,
            vcard: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Sets `last_name` to `Some(last_name)`.
    pub fn last_name<'b: 'a>(mut self, last_name: &'b str) -> Self {
        self.last_name = Some(last_name);
        self
    }

    /// Sets `vcard` to `Some(vcard)`.
    pub fn vcard<'b: 'a>(mut self, vcard: &'b str) -> Self {
        self.vcard = Some(vcard);
        self
    }

    /// Sets `disable_notification` to `Some(is_disabled)`.
    pub fn disable_notification(mut self, is_disabled: bool) -> Self {
        self.disable_notification = Some(is_disabled);
        self
    }

    /// Sets `reply_to_message_id` to `Some(id)`.
    pub fn reply_to_message_id(mut self, id: u64) -> Self {
        self.reply_to_message_id = Some(id);
        self
    }

    /// Sets `reply_markup` to `Some(markup)`.
    pub fn reply_markup(
        mut self,
        markup: impl Into<types::raw::Keyboard<'a>>,
    ) -> Self {
        self.reply_markup = Some(markup.into());
        self
    }

    /// Prepares the request and returns a `Future`.
    #[must_use = "futures do nothing unless polled"]
    pub fn into_future(
        self,
    ) -> impl Future<Item = types::raw::Message, Error = DeliveryError> {
        send_method::<types::raw::Message>(
            self.token,
            "sendContact",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
    }
}

use super::*;

/// Represents the [`sendContact`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendcontact
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendContact<'a> {
    #[serde(skip)]
    token: &'a str,
    #[serde(skip)]
    #[cfg(feature = "proxy")]
    proxy: Option<proxy::Proxy>,
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
    reply_to_message_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<types::AnyKeyboard<'a>>,
}

impl<'a> SendContact<'a> {
    /// Constructs a new `SendContact`.
    pub fn new(
        token: &'a str,
        chat_id: impl Into<types::ChatId<'a>>,
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
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }

    /// Configures `last_name`.
    pub fn last_name(mut self, last_name: &'a str) -> Self {
        self.last_name = Some(last_name);
        self
    }

    /// Configures `vcard`.
    pub fn vcard(mut self, vcard: &'a str) -> Self {
        self.vcard = Some(vcard);
        self
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
}

impl IntoFuture for SendContact<'_> {
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = types::Message;
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            self.token,
            "sendContact",
            None,
            serde_json::to_vec(&self).unwrap(),
            #[cfg(feature = "proxy")]
            self.proxy,
        ))
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for SendContact<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}

use super::*;

/// Representation of the [`sendVenue`] method.
///
/// [`sendVenue`]: https://core.telegram.org/bots/api#sendvenue
#[derive(Serialize)]
pub struct SendVenue<'a> {
    #[serde(skip)]
    token: &'a str,
    chat_id: types::ChatId<'a>,
    latitude: f64,
    longitude: f64,
    title: &'a str,
    address: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_type: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<types::raw::Keyboard<'a>>,
}

impl<'a> SendVenue<'a> {
    /// Constructs a new `SendVenue`.
    #[must_use]
    pub fn new<'b: 'a>(
        token: &'b str,
        chat_id: impl Into<types::ChatId<'b>>,
        (latitude, longitude): (f64, f64),
        title: &'b str,
        address: &'b str,
    ) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            latitude,
            longitude,
            title,
            address,
            foursquare_id: None,
            foursquare_type: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Sets `foursquare_id` to `Some(id)`.
    #[must_use]
    pub fn foursquare_id<'b: 'a>(mut self, id: &'b str) -> Self {
        self.foursquare_id = Some(id);
        self
    }

    /// Sets `foursquare_type` to `Some(fs_type)`.
    #[must_use]
    pub fn foursquare_type<'b: 'a>(mut self, fs_type: &'b str) -> Self {
        self.foursquare_type = Some(fs_type);
        self
    }

    /// Sets `disable_notification` to `Some(is_disabled)`.
    #[must_use]
    pub fn disable_notification(mut self, is_disabled: bool) -> Self {
        self.disable_notification = Some(is_disabled);
        self
    }

    /// Sets `reply_to_message_id` to `Some(id)`.
    #[must_use]
    pub fn reply_to_message_id(mut self, id: u64) -> Self {
        self.reply_to_message_id = Some(id);
        self
    }

    /// Sets `reply_markup` to `Some(markup)`.
    #[must_use]
    pub fn reply_markup(
        mut self,
        markup: impl Into<types::raw::Keyboard<'a>>,
    ) -> Self {
        self.reply_markup = Some(markup.into());
        self
    }

    /// Prepares the request and returns a `Future`.
    #[must_use]
    pub fn into_future(
        self,
    ) -> impl Future<Item = types::raw::Message, Error = DeliveryError> {
        send_method::<types::raw::Message>(
            self.token,
            "sendVenue",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
    }
}

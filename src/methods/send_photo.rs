use super::*;

/// Representation of the [`sendPhoto`] method.
///
/// [`sendPhoto`]: https://core.telegram.org/bots/api#sendphoto
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendPhoto<'a> {
    #[serde(skip)]
    token: &'a str,
    chat_id: types::ChatId<'a>,
    photo: types::InputFile<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<types::ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<types::raw::Keyboard<'a>>,
}

impl<'a> SendPhoto<'a> {
    /// Constructs a new `SendPhoto`.
    pub fn new<'b: 'a>(
        token: &'b str,
        chat_id: impl Into<types::ChatId<'b>>,
        photo: types::Photo<'b>,
    ) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            photo: photo.0,
            caption: None,
            parse_mode: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Configures `caption`.
    pub fn caption(mut self, caption: &'a str) -> Self {
        self.caption = Some(caption);
        self
    }

    /// Configures `parse_mode`.
    pub fn parse_mode(mut self, mode: types::ParseMode) -> Self {
        self.parse_mode = Some(mode);
        self
    }

    /// Configures `disable_notification`.
    pub fn disable_notification(mut self, is_disabled: bool) -> Self {
        self.disable_notification = Some(is_disabled);
        self
    }

    /// Configures `reply_to_message_id`.
    pub fn reply_to_message_id(mut self, id: u64) -> Self {
        self.reply_to_message_id = Some(id);
        self
    }

    /// Configures `reply_markup`.
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
        let (boundary, body) = if let types::InputFile::File {
            filename,
            bytes,
            ..
        } = self.photo
        {
            let chat_id = match self.chat_id {
                types::ChatId::Id(id) => id.to_string(),
                types::ChatId::Username(username) => username.into(),
            };

            let parse_mode = self
                .parse_mode
                .and_then(|parse_mode| serde_json::to_string(&parse_mode).ok());
            let is_disabled = self.disable_notification.map(|x| x.to_string());
            let reply_to = self.reply_to_message_id.map(|id| id.to_string());
            let reply_markup = self
                .reply_markup
                .and_then(|markup| serde_json::to_string(&markup).ok());

            let (boundary, body) = Multipart::new(7)
                .str("chat_id", &chat_id)
                .file("photo", filename, bytes)
                .maybe_str("caption", self.caption)
                .maybe_string("parse_mode", &parse_mode)
                .maybe_string("disabled_notification", &is_disabled)
                .maybe_string("reply_to_message_id", &reply_to)
                .maybe_string("reply_markup", &reply_markup)
                .finish();

            (Some(boundary), body)
        } else {
            (None, serde_json::to_vec(&self).unwrap())
        };

        send_method::<types::raw::Message>(
            self.token,
            "sendPhoto",
            boundary,
            body,
        )
    }
}

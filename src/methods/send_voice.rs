use super::*;

/// Represents the [`sendVoice`] method.
///
/// [`sendVoice`]: https://core.telegram.org/bots/api#sendvoice
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendVoice<'a> {
    #[serde(skip)]
    token: &'a str,
    chat_id: types::ChatId<'a>,
    voice: types::InputFile<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<u64>,
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

impl<'a> SendVoice<'a> {
    /// Constructs a new `SendVoice`.
    pub fn new<'b: 'a>(
        token: &'b str,
        chat_id: impl Into<types::ChatId<'b>>,
        voice: types::Voice<'a>,
    ) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            voice: voice.0,
            duration: None,
            caption: None,
            parse_mode: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Configures `duration`.
    pub fn duration(mut self, duration: u64) -> Self {
        self.duration = Some(duration);
        self
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
        } = self.voice
        {
            let chat_id = match self.chat_id {
                types::ChatId::Id(id) => id.to_string(),
                types::ChatId::Username(username) => username.into(),
            };

            let duration = self.duration.map(|duration| duration.to_string());
            let parse_mode = self
                .parse_mode
                .and_then(|parse_mode| serde_json::to_string(&parse_mode).ok());
            let reply_to = self.reply_to_message_id.map(|id| id.to_string());
            let is_disabled = self.disable_notification.map(|x| x.to_string());
            let reply_markup = self
                .reply_markup
                .and_then(|markup| serde_json::to_string(&markup).ok());

            let mut multipart = Multipart::new(8)
                .str("chat_id", &chat_id)
                .file("voice", filename, bytes)
                .maybe_string("duration", &duration)
                .maybe_str("caption", self.caption)
                .maybe_string("parse_mode", &parse_mode)
                .maybe_string("disable_notification", &is_disabled)
                .maybe_string("reply_to_message_id", &reply_to)
                .maybe_string("reply_markup", &reply_markup);

            let (boundary, body) = multipart.finish();

            (Some(boundary), body)
        } else {
            (None, serde_json::to_vec(&self).unwrap())
        };

        send_method(self.token, "sendVoice", boundary, body)
    }
}

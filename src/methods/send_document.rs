use super::*;

/// Representation of the [`sendDocument`] method.
///
/// [`sendDocument`]: https://core.telegram.org/bots/api#senddocument
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendDocument<'a> {
    #[serde(skip)]
    token: &'a str,
    chat_id: types::ChatId<'a>,
    document: types::InputFile<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb: Option<types::InputFile<'a>>,
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

impl<'a> SendDocument<'a> {
    /// Constructs a new `SendDocument`.
    pub fn new<'b: 'a>(
        token: &'b str,
        chat_id: impl Into<types::ChatId<'b>>,
        document: types::Document<'a>,
    ) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            document: document.0,
            thumb: None,
            caption: None,
            parse_mode: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Configures `thumb`.
    pub fn thumb(mut self, thumb: types::Thumb<'a>) -> Self {
        self.thumb = Some(thumb.0);
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
        } = self.document
        {
            let chat_id = match self.chat_id {
                types::ChatId::Id(id) => id.to_string(),
                types::ChatId::Username(username) => username.into(),
            };

            let parse_mode = if let Some(parse_mode) = self.parse_mode {
                serde_json::to_string(&parse_mode).ok()
            } else {
                None
            };

            let is_disabled =
                if let Some(is_disabled) = self.disable_notification {
                    Some(is_disabled.to_string())
                } else {
                    None
                };

            let reply_to = if let Some(id) = self.reply_to_message_id {
                Some(id.to_string())
            } else {
                None
            };

            let reply_markup = if let Some(keyboard) = self.reply_markup {
                serde_json::to_string(&keyboard).ok()
            } else {
                None
            };

            let mut multipart = Multipart::new(8)
                .str("chat_id", &chat_id)
                .file("document", filename, bytes)
                .maybe_str("caption", self.caption)
                .maybe_string("parse_mode", &parse_mode)
                .maybe_string("disable_notification", &is_disabled)
                .maybe_string("reply_to_message_id", &reply_to)
                .maybe_string("reply_markup", &reply_markup);

            if let Some(types::InputFile::File {
                filename,
                bytes,
                ..
            }) = self.thumb
            {
                multipart = multipart.file("thumb", filename, bytes);
            }

            let (boundary, body) = multipart.finish();

            (Some(boundary), body)
        } else {
            (None, serde_json::to_vec(&self).unwrap())
        };

        send_method(
            self.token,
            "sendDocument",
            boundary,
            body,
        )
    }
}

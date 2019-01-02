use super::*;

/// Representation of the [`sendVideoNote`] method.
///
/// [`sendVideoNote`]: https://core.telegram.org/bots/api#sendvideonote
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendVideoNote<'a> {
    #[serde(skip)]
    token: &'a str,
    chat_id: types::ChatId<'a>,
    video_note: types::InputFile<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    length: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb: Option<types::InputFile<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<types::raw::Keyboard<'a>>,
}

impl<'a> SendVideoNote<'a> {
    /// Constructs a new `SendVideoNote`.
    pub fn new<'b: 'a>(
        token: &'b str,
        chat_id: impl Into<types::ChatId<'b>>,
        video_note: types::VideoNote<'a>,
    ) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            video_note: video_note.0,
            duration: None,
            length: None,
            thumb: None,
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

    /// Configures `length`.
    pub fn length(mut self, length: u64) -> Self {
        self.length = Some(length);
        self
    }

    /// Configures `thumb`.
    pub fn thumb(mut self, thumb: types::Thumb<'a>) -> Self {
        self.thumb = Some(thumb.0);
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
        } = self.video_note
        {
            let chat_id = match self.chat_id {
                types::ChatId::Id(id) => id.to_string(),
                types::ChatId::Username(username) => username.into(),
            };

            let duration = self.duration.map(|duration| duration.to_string());
            let length = self.length.map(|length| length.to_string());
            let reply_to = self.reply_to_message_id.map(|id| id.to_string());
            let is_disabled = self.disable_notification.map(|x| x.to_string());
            let reply_markup = self
                .reply_markup
                .and_then(|markup| serde_json::to_string(&markup).ok());

            let mut multipart = Multipart::new(8)
                .str("chat_id", &chat_id)
                .file("video_note", filename, bytes)
                .maybe_string("duration", &duration)
                .maybe_string("length", &length)
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

        send_method::<types::raw::Message>(
            self.token,
            "sendVideoNote",
            boundary,
            body,
        )
    }
}

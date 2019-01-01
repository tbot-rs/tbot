use super::*;

/// Representation of the [`sendAnimation`] method.
///
/// [`sendAnimation`]: https://core.telegram.org/bots/api#sendanimation
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendAnimation<'a> {
    #[serde(skip)]
    token: &'a str,
    chat_id: types::ChatId<'a>,
    animation: types::InputFile<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<u64>,
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

impl<'a> SendAnimation<'a> {
    /// Constructs a new `SendMessage`.
    pub fn new<'b: 'a>(
        token: &'b str,
        chat_id: impl Into<types::ChatId<'b>>,
        animation: types::Animation<'a>,
    ) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            animation: animation.0,
            duration: None,
            width: None,
            height: None,
            thumb: None,
            caption: None,
            parse_mode: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Configures `duration`.
    pub fn duration(mut self, duration: u64) -> Self {
        self.height = Some(duration);
        self
    }

    /// Configures `width`.
    pub fn width(mut self, width: u64) -> Self {
        self.width = Some(width);
        self
    }

    /// Configures `height`.
    pub fn height(mut self, height: u64) -> Self {
        self.height = Some(height);
        self
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
        let (boundary, body) =
            if let types::InputFile::File {
                filename,
                bytes,
                ..
            } = self.animation
            {
                let chat_id = match self.chat_id {
                    types::ChatId::Id(id) => id.to_string(),
                    types::ChatId::Username(username) => username.into(),
                };

                let duration = if let Some(duration) = self.duration {
                    Some(duration.to_string())
                } else {
                    None
                };

                let width = if let Some(width) = self.width {
                    Some(width.to_string())
                } else {
                    None
                };

                let height = if let Some(height) = self.height {
                    Some(height.to_string())
                } else {
                    None
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

                let reply_to_message_id =
                    if let Some(id) = self.reply_to_message_id {
                        Some(id.to_string())
                    } else {
                        None
                    };

                let reply_markup = if let Some(keyboard) = self.reply_markup {
                    serde_json::to_string(&keyboard).ok()
                } else {
                    None
                };

                let mut body = Multipart::new(7)
                    .field("chat_id", &chat_id)
                    .file("animation", filename, bytes);

                if let Some(ref duration) = duration {
                    body = body.field("duration", duration);
                }
                if let Some(ref width) = width {
                    body = body.field("width", width);
                }
                if let Some(ref height) = height {
                    body = body.field("height", height);
                }
                // It's either Some(File) or None. Some(OtherVariant) won't
                // exist
                if let Some(types::InputFile::File {
                    filename,
                    bytes,
                    ..
                }) = self.thumb
                {
                    body = body.file("thumb", filename, bytes);
                }
                if let Some(caption) = self.caption {
                    body = body.field("caption", caption);
                }
                if let Some(ref parse_mode) = parse_mode {
                    body = body.field("parse_mode", parse_mode);
                }
                if let Some(ref is_disabled) = is_disabled {
                    body = body.field("disable_notification", is_disabled);
                }
                if let Some(ref id) = reply_to_message_id {
                    body = body.field("reply_to_message_id", id);
                }
                if let Some(ref keyboard) = reply_markup {
                    body = body.field("reply_keyboard", keyboard);
                }

                let (boundary, body) = body.finish();

                (Some(boundary), body)
            } else {
                (None, serde_json::to_vec(&self).unwrap())
            };

        send_method::<types::raw::Message>(
            self.token,
            "sendAnimation",
            boundary,
            body,
        )
    }
}

use super::call_method;
use crate::{
    connectors::Client,
    errors, token,
    types::{
        input_file::{Audio, InputFile, Thumb},
        keyboard,
        message::{self, Message},
        parameters::{ChatId, ImplicitChatId, NotificationState},
    },
    Multipart,
};

/// Sends an audio.
///
/// Reflects the [`sendAudio`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendaudio
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendAudio<'a> {
    client: &'a Client,
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
    audio: Audio<'a>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<message::Id>,
    reply_markup: Option<keyboard::Any<'a>>,
}

impl<'a> SendAudio<'a> {
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        chat_id: impl ImplicitChatId<'a>,
        audio: Audio<'a>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            audio,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Configures if the message will be sent silently.
    /// Reflects the `disable_notification` parameter.
    pub fn notification(mut self, state: NotificationState) -> Self {
        self.disable_notification = Some(state.is_disabled());
        self
    }

    /// Configures which message this audio is sent in reply to.
    /// Reflects the `reply_to_message_id` parameter.
    pub fn reply_to_message_id(mut self, id: message::Id) -> Self {
        self.reply_to_message_id = Some(id);
        self
    }

    /// Configures a keyboard for the message.
    /// Reflects the `reply_markup` parameter.
    pub fn reply_markup(
        mut self,
        markup: impl Into<keyboard::Any<'a>>,
    ) -> Self {
        self.reply_markup = Some(markup.into());
        self
    }
}

impl SendAudio<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Message, errors::MethodCall> {
        let mut multipart = Multipart::new(11)
            .chat_id("chat_id", &self.chat_id)
            .maybe_string("duration", self.audio.duration)
            .maybe_str("caption", self.audio.caption.as_deref())
            .maybe_str("performer", self.audio.performer.as_deref())
            .maybe_str("title", self.audio.title.as_deref())
            .maybe_json("parse_mode", self.audio.parse_mode)
            .maybe_string("disable_notification", self.disable_notification)
            .maybe_string("reply_to_message_id", self.reply_to_message_id)
            .maybe_json("reply_markup", self.reply_markup);

        match &self.audio.media {
            InputFile::File {
                filename, bytes, ..
            } => multipart = multipart.file("audio", filename, bytes),
            InputFile::Id(audio) | InputFile::Url(audio) => {
                multipart = multipart.str("audio", audio);
            }
        }

        if let Some(Thumb(InputFile::File {
            filename, bytes, ..
        })) = &self.audio.thumb
        {
            multipart = multipart.file("thumb", filename, bytes);
        }

        let (boundary, body) = multipart.finish();

        call_method(self.client, self.token, "sendAudio", Some(boundary), body)
            .await
    }
}

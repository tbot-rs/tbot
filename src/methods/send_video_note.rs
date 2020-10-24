use super::call_method;
#[allow(deprecated)]
use crate::{
    connectors::Client,
    errors, token,
    types::{
        input_file::{InputFile, Thumb, VideoNote},
        keyboard,
        message::{self, Message},
        parameters::{ChatId, ImplicitChatId, NotificationState},
    },
    Multipart,
};

/// Sends a video note.
///
/// Reflects the [`sendVideoNote`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendvideonote
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendVideoNote<'a> {
    client: &'a Client,
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
    video_note: VideoNote<'a>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<message::Id>,
    reply_markup: Option<keyboard::Any<'a>>,
}

impl<'a> SendVideoNote<'a> {
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        chat_id: impl ImplicitChatId<'a>,
        video_note: VideoNote<'a>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            video_note,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Configures if the message will be sent silently.
    /// Reflects the `disable_notification` parameter.
    pub fn is_notification_disabled(mut self, is_disabled: bool) -> Self {
        self.disable_notification = Some(is_disabled);
        self
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "use `is_notification_disabled` which takes a `bool`"
    )]
    #[allow(deprecated)]
    pub fn notification(self, state: NotificationState) -> Self {
        self.is_notification_disabled(state.is_disabled())
    }

    /// Configures which message this video note is sent in reply to.
    /// Reflects the `reply_to_message_id` parameter.
    pub fn in_reply_to(mut self, id: message::Id) -> Self {
        self.reply_to_message_id = Some(id);
        self
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "this method is renamed to `in_reply_to`"
    )]
    pub fn reply_to_message_id(self, id: message::Id) -> Self {
        self.in_reply_to(id)
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

impl SendVideoNote<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Message, errors::MethodCall> {
        let mut multipart = Multipart::new(8)
            .chat_id("chat_id", self.chat_id)
            .maybe_string("duration", self.video_note.duration)
            .maybe_string("length", self.video_note.length)
            .maybe_string("disable_notification", self.disable_notification)
            .maybe_string("reply_to_message_id", self.reply_to_message_id)
            .maybe_json("reply_markup", self.reply_markup);

        match self.video_note.media.file {
            InputFile::File {
                filename, bytes, ..
            } => multipart = multipart.file("video_note", filename, bytes),
            InputFile::Id(video_note) | InputFile::Url(video_note) => {
                multipart = multipart.str("video_note", video_note);
            }
        }

        if let Some(Thumb(InputFile::File {
            filename, bytes, ..
        })) = self.video_note.thumb
        {
            multipart = multipart.file("thumb", filename, bytes);
        }

        let (boundary, body) = multipart.finish();

        call_method(
            self.client,
            self.token,
            "sendVideoNote",
            Some(boundary),
            body,
        )
        .await
    }
}

use super::call_method;
use crate::{
    connectors::Client,
    errors, token,
    types::{
        input_file::{
            Animation, Audio, Document, EditableMedia, InputFile, Photo, Video,
        },
        keyboard::inline,
        message::{self, Message},
        parameters::{ChatId, ImplicitChatId},
    },
    Multipart,
};

/// Edits the media of a message sent by the bot itself.
///
/// Reflects the [`editMessageMedia`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagemedia
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditMessageMedia<'a> {
    client: &'a Client,
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
    message_id: message::Id,
    media: EditableMedia<'a>,
    reply_markup: Option<inline::Keyboard<'a>>,
}

impl<'a> EditMessageMedia<'a> {
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
        media: impl Into<EditableMedia<'a>>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            message_id,
            media: media.into(),
            reply_markup: None,
        }
    }

    /// Configures an inline keyboard for the message.
    /// Reflects the `reply_markup` parameter.
    pub const fn reply_markup(mut self, markup: inline::Keyboard<'a>) -> Self {
        self.reply_markup = Some(markup);
        self
    }
}

impl EditMessageMedia<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Message, errors::MethodCall> {
        let mut multipart = Multipart::new(5)
            .chat_id("chat_id", self.chat_id)
            .string("message_id", &self.message_id)
            .maybe_json("reply_markup", self.reply_markup);

        match &self.media {
            EditableMedia::Animation(Animation { media, .. })
            | EditableMedia::Audio(Audio { media, .. })
            | EditableMedia::Document(Document { media, .. })
            | EditableMedia::Photo(Photo { media, .. })
            | EditableMedia::Video(Video { media, .. }) => {
                if let InputFile::File { filename, bytes } = media {
                    multipart =
                        multipart.file(self.media.name(), filename, bytes);
                }
            }
        }

        let (boundary, body) = multipart.json("media", self.media).finish();

        call_method(
            self.client,
            self.token,
            "editMessageMedia",
            Some(boundary),
            body,
        )
        .await
    }
}

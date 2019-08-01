use super::*;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    types::{
        input_file::*,
        keyboard::inline,
        message,
        parameters::{ChatId, ImplicitChatId},
        value::{Ref, Value},
    },
};

/// Represents the [`editMessageMedia`][docs] method for chat messages.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagemedia
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditMessageMedia<'a, C> {
    client: &'a Client<C>,
    token: Token,
    chat_id: ChatId<'a>,
    message_id: message::Id,
    media: Ref<'a, EditableMedia<'a>>,
    reply_markup: Option<Ref<'a, inline::Keyboard<'a>>>,
}

impl<'a, C> EditMessageMedia<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
        media: impl Into<Ref<'a, EditableMedia<'a>>>,
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

    /// Configures `reply_markup`.
    pub fn reply_markup(
        mut self,
        markup: impl Into<Ref<'a, inline::Keyboard<'a>>>,
    ) -> Self {
        self.reply_markup = Some(markup.into());
        self
    }
}

impl<C> IntoFuture for EditMessageMedia<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = types::Message;
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        let mut multipart = Multipart::new(5)
            .chat_id("chat_id", self.chat_id)
            .from("message_id", &self.message_id)
            .maybe_json("reply_markup", self.reply_markup);

        let media = self.media.as_ref();
        let name = media.name();

        match media {
            EditableMedia::Animation(Value::Owned(Animation {
                media,
                ..
            }))
            | EditableMedia::Animation(Value::Borrowed(Animation {
                media,
                ..
            }))
            | EditableMedia::Audio(Value::Owned(Audio {
                media,
                ..
            }))
            | EditableMedia::Audio(Value::Borrowed(Audio {
                media,
                ..
            }))
            | EditableMedia::Document(Value::Owned(Document {
                media,
                ..
            }))
            | EditableMedia::Document(Value::Borrowed(Document {
                media,
                ..
            }))
            | EditableMedia::Photo(Value::Owned(Photo {
                media,
                ..
            }))
            | EditableMedia::Photo(Value::Borrowed(Photo {
                media,
                ..
            }))
            | EditableMedia::Video(Value::Owned(Video {
                media,
                ..
            }))
            | EditableMedia::Video(Value::Borrowed(Video {
                media,
                ..
            })) => {
                if let InputFile::File {
                    filename,
                    bytes,
                } = media
                {
                    multipart = multipart.file(name, filename, bytes);
                }
            }
        }

        let (boundary, body) = multipart.json("media", media).finish();

        Box::new(send_method(
            self.client,
            &self.token,
            "editMessageMedia",
            Some(boundary),
            body,
        ))
    }
}

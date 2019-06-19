use super::*;
use crate::{
    internal::{BoxFuture, Client},
    types::{
        input_file::*,
        keyboard::inline,
        message,
        parameters::{ChatId, ImplicitChatId},
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
    media: EditableMedia<'a>,
    reply_markup: Option<inline::Keyboard<'a>>,
}

impl<'a, C> EditMessageMedia<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
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

    /// Configures `reply_markup`.
    pub fn reply_markup(mut self, markup: inline::Keyboard<'a>) -> Self {
        self.reply_markup = Some(markup);
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
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        let chat_id = match self.chat_id {
            ChatId::Id(id) => id.to_string(),
            ChatId::Username(username) => username.into(),
        };
        let message_id = self.message_id.to_string();
        let reply_markup =
            self.reply_markup.and_then(|x| serde_json::to_string(&x).ok());

        let mut multipart = Multipart::new(5)
            .str("chat_id", &chat_id)
            .str("message_id", &message_id)
            .maybe_string("reply_markup", &reply_markup);

        match &self.media {
            EditableMedia::Animation(Animation {
                media,
                ..
            })
            | EditableMedia::Audio(Audio {
                media,
                ..
            })
            | EditableMedia::Document(Document {
                media,
                ..
            })
            | EditableMedia::Photo(Photo {
                media,
                ..
            })
            | EditableMedia::Video(Video {
                media,
                ..
            }) => {
                if let InputFile::File {
                    name,
                    filename,
                    bytes,
                } = media
                {
                    multipart = multipart.file(name, filename, bytes);
                }
            }
        }

        let media = serde_json::to_string(&self.media).unwrap();
        let (boundary, body) = multipart.str("media", &media).finish();

        Box::new(send_method(
            self.client,
            &self.token,
            "editMessageMedia",
            Some(boundary),
            body,
        ))
    }
}

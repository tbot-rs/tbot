use super::*;
use crate::{
    internal::{BoxFuture, Client},
    types::{inline_message_id, input_file::*, keyboard::inline},
};

/// Represents the [`editMessageMedia`][docs] method for inline messages.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagemedia
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditInlineMedia<'a, C> {
    client: &'a Client<C>,
    token: Token,
    inline_message_id: inline_message_id::Ref<'a>,
    media: EditableMedia<'a>,
    reply_markup: Option<inline::Keyboard<'a>>,
}

impl<'a, C> EditInlineMedia<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        inline_message_id: inline_message_id::Ref<'a>,
        media: impl Into<EditableMedia<'a>>,
    ) -> Self {
        Self {
            client,
            token,
            inline_message_id,
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

impl<C> IntoFuture for EditInlineMedia<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = ();
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        let reply_markup =
            self.reply_markup.and_then(|x| serde_json::to_string(&x).ok());

        let mut multipart = Multipart::new(4)
            .str("inline_message_id", self.inline_message_id.0)
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

        Box::new(
            send_method::<bool, C>(
                self.client,
                &self.token,
                "editMessageMedia",
                Some(boundary),
                body,
            )
            .map(|_| ()),
        )
    }
}

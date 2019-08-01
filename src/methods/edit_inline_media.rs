use super::*;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    types::{
        input_file::*,
        keyboard::inline,
        value::{InlineMessageId, Ref, Value},
    },
};

/// Represents the [`editMessageMedia`][docs] method for inline messages.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagemedia
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditInlineMedia<'a, C> {
    client: &'a Client<C>,
    token: Token,
    inline_message_id: InlineMessageId<'a>,
    media: Ref<'a, EditableMedia<'a>>,
    reply_markup: Option<Ref<'a, inline::Keyboard<'a>>>,
}

impl<'a, C> EditInlineMedia<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        inline_message_id: impl Into<InlineMessageId<'a>>,
        media: impl Into<Ref<'a, EditableMedia<'a>>>,
    ) -> Self {
        Self {
            client,
            token,
            inline_message_id: inline_message_id.into(),
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

impl<C> IntoFuture for EditInlineMedia<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = ();
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        let mut multipart = Multipart::new(4)
            .str("inline_message_id", self.inline_message_id.as_ref().0)
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

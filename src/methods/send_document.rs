use super::*;
use crate::{
    errors,
    internal::{AsInnerRef, BoxFuture, Client},
    types::{
        input_file::{Document, InputFile, Thumb},
        keyboard, message,
        parameters::{ChatId, ImplicitChatId, NotificationState},
        value::Ref,
    },
};

/// Represents the [`sendDocument`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#senddocument
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendDocument<'a, C> {
    client: &'a Client<C>,
    token: Token,
    chat_id: ChatId<'a>,
    document: Ref<'a, Document<'a>>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<message::Id>,
    reply_markup: Option<Ref<'a, keyboard::Any<'a>>>,
}

impl<'a, C> SendDocument<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl ImplicitChatId<'a>,
        document: impl Into<Ref<'a, Document<'a>>>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            document: document.into(),
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Configures `disable_notification`.
    pub fn notification(mut self, state: NotificationState) -> Self {
        self.disable_notification = Some(state.is_disabled());
        self
    }

    /// Configures `reply_to_message_id`.
    pub fn reply_to_message_id(mut self, id: message::Id) -> Self {
        self.reply_to_message_id = Some(id);
        self
    }

    /// Configures `reply_markup`.
    pub fn reply_markup(
        mut self,
        markup: impl Into<Ref<'a, keyboard::Any<'a>>>,
    ) -> Self {
        self.reply_markup = Some(markup.into());
        self
    }
}

impl<C> IntoFuture for SendDocument<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = types::Message;
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        let document = self.document.as_ref();
        let mut multipart = Multipart::new(8)
            .chat_id("chat_id", self.chat_id)
            .maybe_str(
                "caption",
                match &document.caption {
                    Some(caption) => Some(caption.as_str()),
                    None => None,
                },
            )
            .maybe_json("parse_mode", document.parse_mode)
            .maybe_from("disable_notification", self.disable_notification)
            .maybe_from("reply_to_message_id", self.reply_to_message_id)
            .maybe_json("reply_markup", self.reply_markup);

        match &document.media {
            InputFile::File {
                filename,
                bytes,
                ..
            } => multipart = multipart.file("document", filename, bytes),
            InputFile::Id(id) => {
                multipart = multipart.str("document", id.as_ref().0);
            }
            InputFile::Url(url) => {
                multipart = multipart.str("document", url);
            }
        }

        if let Some(Thumb(InputFile::File {
            filename,
            bytes,
            ..
        })) = document.thumb.as_inner_ref()
        {
            multipart = multipart.file("thumb", filename, bytes);
        }

        let (boundary, body) = multipart.finish();

        Box::new(send_method(
            self.client,
            &self.token,
            "sendDocument",
            Some(boundary),
            body,
        ))
    }
}

use super::*;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    types::{
        input_file::{InputFile, Voice},
        keyboard, message,
        parameters::{ChatId, ImplicitChatId, NotificationState},
        value::Ref,
    },
};

/// Represents the [`sendVoice`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendvoice
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendVoice<'a, C> {
    client: &'a Client<C>,
    token: Token,
    chat_id: ChatId<'a>,
    voice: Ref<'a, Voice<'a>>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<message::Id>,
    reply_markup: Option<Ref<'a, keyboard::Any<'a>>>,
}

impl<'a, C> SendVoice<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl ImplicitChatId<'a>,
        voice: impl Into<Ref<'a, Voice<'a>>>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            voice: voice.into(),
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

impl<C> IntoFuture for SendVoice<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = types::Message;
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        let voice = self.voice.as_ref();
        let mut multipart = Multipart::new(8)
            .chat_id("chat_id", self.chat_id)
            .maybe_from("duration", voice.duration)
            .maybe_str(
                "caption",
                match &voice.caption {
                    Some(caption) => Some(caption.as_str()),
                    None => None,
                },
            )
            .maybe_json("parse_mode", voice.parse_mode)
            .maybe_from("disable_notification", self.disable_notification)
            .maybe_from("reply_to_message_id", self.reply_to_message_id)
            .maybe_json("reply_markup", self.reply_markup);

        match voice.media.file.as_ref() {
            InputFile::File {
                filename,
                bytes,
                ..
            } => multipart = multipart.file("voice", filename, bytes),
            InputFile::Id(id) => {
                multipart = multipart.str("voice", id.as_ref().0);
            }
            InputFile::Url(url) => {
                multipart = multipart.str("voice", url);
            }
        }

        let (boundary, body) = multipart.finish();

        Box::new(send_method(
            self.client,
            &self.token,
            "sendVoice",
            Some(boundary),
            body,
        ))
    }
}

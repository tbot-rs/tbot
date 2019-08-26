use super::*;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    types::{
        input_file::*,
        message,
        parameters::{ChatId, ImplicitChatId, NotificationState},
    },
};

/// Represents the [`sendMediaGroup`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendmediagroup
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendMediaGroup<'a, C> {
    client: &'a Client<C>,
    token: Token,
    chat_id: ChatId<'a>,
    media: &'a [GroupMedia<'a>],
    disable_notification: Option<bool>,
    reply_to_message_id: Option<message::Id>,
}

impl<'a, C> SendMediaGroup<'a, C> {
    /// Contructs a new `SendMediaGroup`.
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl ImplicitChatId<'a>,
        media: &'a [GroupMedia<'a>],
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            media,
            disable_notification: None,
            reply_to_message_id: None,
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
}

impl<C> IntoFuture for SendMediaGroup<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = Vec<types::Message>;
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        let mut multipart = Multipart::new(4 + self.media.len())
            .chat_id("chat_id", self.chat_id)
            .maybe_string("disabled_notification", self.disable_notification)
            .maybe_string("reply_to_message_id", self.reply_to_message_id);

        for (index, media) in self.media.iter().enumerate() {
            match media {
                GroupMedia::Photo(Photo {
                    media: InputFile::File { filename, bytes },
                    ..
                }) => {
                    let name = format!("photo_{}", index);

                    multipart =
                        multipart.file_owned_name(name, filename, bytes);
                }
                GroupMedia::Video(Video {
                    media: InputFile::File { filename, bytes },
                    thumb,
                    ..
                }) => {
                    let name = format!("video_{}", index);
                    multipart =
                        multipart.file_owned_name(name, filename, bytes);

                    if let Some(Thumb(InputFile::File { filename, bytes })) =
                        thumb
                    {
                        let name = format!("thumb_{}", index);
                        multipart =
                            multipart.file_owned_name(name, filename, bytes);
                    }
                }
                _ => (),
            }
        }

        let media = Album(self.media);
        let (boundary, body) = multipart.json("media", &media).finish();

        Box::new(send_method(
            self.client,
            &self.token,
            "sendMediaGroup",
            Some(boundary),
            body,
        ))
    }
}

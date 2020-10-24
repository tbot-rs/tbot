use super::call_method;
#[allow(deprecated)]
use crate::{
    connectors::Client,
    errors, token,
    types::{
        input_file::{Album, GroupMedia, InputFile, Photo, Thumb, Video},
        message::{self, Message},
        parameters::{ChatId, ImplicitChatId, NotificationState},
    },
    Multipart,
};

/// Sends an album.
///
/// Reflects the [`sendMediaGroup`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendmediagroup
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendMediaGroup<'a> {
    client: &'a Client,
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
    media: &'a [GroupMedia<'a>],
    disable_notification: Option<bool>,
    reply_to_message_id: Option<message::Id>,
}

impl<'a> SendMediaGroup<'a> {
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
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

    /// Configures if the album will be sent silently.
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

    /// Configures which message this album is sent in reply to.
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
}

impl SendMediaGroup<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Vec<Message>, errors::MethodCall> {
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

        call_method(
            self.client,
            self.token,
            "sendMediaGroup",
            Some(boundary),
            body,
        )
        .await
    }
}

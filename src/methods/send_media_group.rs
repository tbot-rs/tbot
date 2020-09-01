use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{
        input_file::{Album, GroupMedia, InputFile, Photo, Thumb, Video},
        message::{self, Message},
        parameters::{ChatId, ImplicitChatId},
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
    bot: &'a InnerBot,
    chat_id: ChatId<'a>,
    media: &'a [GroupMedia<'a>],
    disable_notification: Option<bool>,
    reply_to_message_id: Option<message::Id>,
}

impl<'a> SendMediaGroup<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId<'a>,
        media: &'a [GroupMedia<'a>],
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            media,
            disable_notification: None,
            reply_to_message_id: None,
        }
    }

    /// Configures whether the album is sent silently.
    /// Reflects the `disable_notification` parameter.
    pub const fn is_notification_disabled(mut self, is_disabled: bool) -> Self {
        self.disable_notification = Some(is_disabled);
        self
    }

    /// Configures which message this album is sent in reply to.
    /// Reflects the `reply_to_message_id` parameter.
    pub const fn in_reply_to(mut self, id: message::Id) -> Self {
        self.reply_to_message_id = Some(id);
        self
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

        call_method(self.bot, "sendMediaGroup", Some(boundary), body).await
    }
}

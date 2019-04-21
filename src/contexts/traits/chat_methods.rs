use super::*;
use crate::types::input_file::*;

/// Contains general methods which can infer some data from the context.
pub trait ChatMethods<'a> {
    #[doc(hidden)]
    fn bot(&self) -> &MockBot;
    #[doc(hidden)]
    fn chat_id(&self) -> i64;
    #[doc(hidden)]
    fn message_id(&self) -> u32;

    /// Constructs a [`DeleteMessage`] inferring the token and the chat ID.
    ///
    /// [`DeleteMessage`]: ../methods/struct.DeleteMessage.html
    fn delete_message(&'a self, message_id: u32) -> DeleteMessage<'a> {
        self.bot().delete_message(self.chat_id(), message_id)
    }

    /// Constructs a [`DeleteMessage`] inferring the token, the chat ID and
    /// the message ID.
    ///
    /// [`DeleteMessage`]: ../methods/struct.DeleteMessage.html
    fn delete_this_message(&'a self) -> DeleteMessage<'a> {
        self.delete_message(self.message_id())
    }

    /// Constructs an [`EditMessageCaption`] inferring the token and
    /// the chat ID.
    ///
    /// [`EditMessageCaption`]: ../methods/struct.EditMessageCaption.html
    fn edit_message_caption(
        &'a self,
        message_id: u32,
        caption: &'a str,
    ) -> EditMessageCaption<'a> {
        self.bot().edit_message_caption(self.chat_id(), message_id, caption)
    }

    /// Constructs an [`EditMessageLocation`] inferring the token and
    /// the chat ID.
    ///
    /// [`EditMessageLocation`]: ../methods/struct.EditMessageLocation.html
    fn edit_message_location(
        &'a self,
        message_id: u32,
        location: (f64, f64),
    ) -> EditMessageLocation<'a> {
        self.bot().edit_message_location(self.chat_id(), message_id, location)
    }

    /// Constructs an [`EditMessageMedia`] inferring the token and the chat ID.
    ///
    /// [`EditMessageMedia`]: ../methods/struct.EditMessageMedia.html
    fn edit_message_media(
        &'a self,
        message_id: u32,
        media: impl Into<EditableMedia<'a>>,
    ) -> EditMessageMedia<'a> {
        self.bot().edit_message_media(self.chat_id(), message_id, media)
    }

    /// Constructs a new [`EditMessageReplyMarkup`] inferring the token and
    /// the chat ID.
    ///
    /// [`EditMessageReplyMarkup`]: ./struct.EditMessageReplyMarkup.html
    fn edit_message_reply_markup(
        &'a self,
        message_id: u32,
        reply_markup: types::InlineKeyboard<'a>,
    ) -> EditMessageReplyMarkup<'a> {
        self.bot().edit_message_reply_markup(
            self.chat_id(),
            message_id,
            reply_markup,
        )
    }

    /// Constructs an [`EditMessageText`] inferring the token and the chat ID.
    ///
    /// [`EditMessageText`]: ../methods/struct.EditMessageText.html
    fn edit_message_text(
        &'a self,
        message_id: u32,
        text: &'a str,
    ) -> EditMessageText<'a> {
        self.bot().edit_message_text(self.chat_id(), message_id, text)
    }

    /// Constructs a new [`ForwardMessage`] inferring the token and the
    /// destination chat ID.
    ///
    /// [`ForwardMessage`]: ../methods/struct.ForwardMessage.html
    fn forward_here(
        &'a self,
        from_chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
    ) -> ForwardMessage<'a> {
        self.bot().forward_message(self.chat_id(), from_chat_id, message_id)
    }

    /// Constructs a [`SendAnimation`] inferring the token and the chat ID.
    ///
    /// [`SendAnimation`]: ../methods/struct.SendAnimation.html
    fn send_animation(
        &'a self,
        animation: &'a Animation<'a>,
    ) -> SendAnimation<'a> {
        self.bot().send_animation(self.chat_id(), animation)
    }

    /// Constructs a [`SendAnimation`] inferring the token, chat ID and message
    /// ID.
    ///
    /// [`SendAnimation`]: ../methods/struct.SendAnimation.html
    fn send_animation_in_reply(
        &'a self,
        animation: &'a Animation<'a>,
    ) -> SendAnimation<'a> {
        self.send_animation(animation).reply_to_message_id(self.message_id())
    }

    /// Constructs a [`SendAudio`] inferring the token and the chat ID.
    ///
    /// [`SendAudio`]: ../methods/struct.SendAudio.html
    fn send_audio(&'a self, audio: &'a Audio<'a>) -> SendAudio<'a> {
        self.bot().send_audio(self.chat_id(), audio)
    }

    /// Constructs a [`SendAudio`] inferring the token, chat ID and message ID.
    ///
    /// [`SendAudio`]: ../methods/struct.SendAudio.html
    fn send_audio_in_reply(&'a self, audio: &'a Audio<'a>) -> SendAudio<'a> {
        self.send_audio(audio).reply_to_message_id(self.message_id())
    }

    /// Constructs a [`SendChatAction`] inferring the token and the chat ID.
    ///
    /// [`SendChatAction`]: ../methods/struct.SendChatAction.html
    fn send_chat_action(&self, action: types::ChatAction) -> SendChatAction {
        self.bot().send_chat_action(self.chat_id(), action)
    }

    /// Construct a [`SendContact`] inferring the token and the chat ID.
    ///
    /// [`SendContact`]: ../methods/struct.SendContact.html
    fn send_contact(
        &'a self,
        phone_number: &'a str,
        first_name: &'a str,
    ) -> SendContact<'a> {
        self.bot().send_contact(self.chat_id(), phone_number, first_name)
    }

    /// Construct a [`SendContact`] inferring the token, chat ID and message ID.
    ///
    /// [`SendContact`]: ../methods/struct.SendContact.html
    fn send_contact_in_reply(
        &'a self,
        phone_number: &'a str,
        first_name: &'a str,
    ) -> SendContact<'a> {
        self.send_contact(phone_number, first_name)
            .reply_to_message_id(self.message_id())
    }

    /// Constructs a [`SendDocument`] inferring the token and the chat ID.
    ///
    /// [`SendDocument`]: ../methods/struct.SendDocument.html
    fn send_document(&'a self, document: &'a Document<'a>) -> SendDocument<'a> {
        self.bot().send_document(self.chat_id(), document)
    }

    /// Constructs a [`SendDocument`] inferring the token, chat ID and message
    /// ID.
    ///
    /// [`SendDocument`]: ../methods/struct.SendDocument.html
    fn send_document_in_reply(
        &'a self,
        document: &'a Document<'a>,
    ) -> SendDocument<'a> {
        self.send_document(document).reply_to_message_id(self.message_id())
    }

    /// Constructs a [`SendLocation`] inferring the token and the chat ID.
    ///
    /// [`SendLocation`]: ../methods/struct.SendLocation.html
    fn send_location(&self, location: (f64, f64)) -> SendLocation<'_> {
        self.bot().send_location(self.chat_id(), location)
    }

    /// Constructs a [`SendLocation`] inferring the token, chat ID and
    /// message ID.
    ///
    /// [`SendLocation`]: ../methods/struct.SendLocation.html
    fn send_location_in_reply(&self, location: (f64, f64)) -> SendLocation<'_> {
        self.send_location(location).reply_to_message_id(self.message_id())
    }

    /// Constructs a [`SendMediaGroup`] inferring the token and the chat ID.
    ///
    /// [`SendMediaGroup`]: ../methods/struct.SendMediaGroup.html
    fn send_media_group(
        &'a self,
        media: Vec<GroupMedia<'a>>,
    ) -> SendMediaGroup<'a> {
        self.bot().send_media_group(self.chat_id(), media)
    }

    /// Constructs a [`SendMediaGroup`] inferring the token, chat ID and
    /// message ID.
    ///
    /// [`SendMediaGroup`]: ../methods/struct.SendMediaGroup.html
    fn send_media_group_in_reply(
        &'a self,
        media: Vec<GroupMedia<'a>>,
    ) -> SendMediaGroup<'a> {
        self.send_media_group(media).reply_to_message_id(self.message_id())
    }

    /// Constructs a [`SendMessage`] inferring the token and the chat ID.
    ///
    /// [`SendMessage`]: ../methods/struct.SendMessage.html
    fn send_message(&'a self, text: &'a str) -> SendMessage<'a> {
        self.bot().send_message(self.chat_id(), text)
    }

    /// Constructs a [`SendMessage`] inferring the token, chat ID and
    /// message ID.
    ///
    /// [`SendMessage`]: ../methods/struct.SendMessage.html
    fn send_message_in_reply(&'a self, text: &'a str) -> SendMessage<'a> {
        self.send_message(text).reply_to_message_id(self.message_id())
    }

    /// Constructs a [`SendPhoto`] inferring the token and the chat ID.
    ///
    /// [`SendPhoto`]: ../methods/struct.SendPhoto.html
    fn send_photo(&'a self, photo: &'a Photo<'a>) -> SendPhoto<'a> {
        self.bot().send_photo(self.chat_id(), photo)
    }

    /// Constructs a [`SendPhoto`] inferring the token, chat ID and message ID.
    ///
    /// [`SendPhoto`]: ../methods/struct.SendPhoto.html
    fn send_photo_in_reply(&'a self, photo: &'a Photo<'a>) -> SendPhoto<'a> {
        self.send_photo(photo).reply_to_message_id(self.message_id())
    }

    /// Constructs a [`SendSticker`] inferring the token and the chat ID.
    ///
    /// [`SendSticker`]: ../methods/struct.SendSticker.html
    fn send_sticker(&'a self, sticker: &'a Sticker<'a>) -> SendSticker<'a> {
        self.bot().send_sticker(self.chat_id(), sticker)
    }

    /// Constructs a [`SendSticker`] inferring the token, chat ID and message ID.
    ///
    /// [`SendSticker`]: ../methods/struct.SendSticker.html
    fn send_sticker_in_reply(
        &'a self,
        sticker: &'a Sticker<'a>,
    ) -> SendSticker<'a> {
        self.send_sticker(sticker).reply_to_message_id(self.message_id())
    }

    /// Constructs a [`SendVenue`] inferring the token and the chat ID.
    ///
    /// [`SendVenue`]: ../methods/struct.SendLocation.html
    fn send_venue(
        &'a self,
        location: (f64, f64),
        title: &'a str,
        address: &'a str,
    ) -> SendVenue<'a> {
        self.bot().send_venue(self.chat_id(), location, title, address)
    }

    /// Constructs a [`SendVenue`] inferring the token, chat ID and
    /// message ID.
    ///
    /// [`SendVenue`]: ../methods/struct.SendLocation.html
    fn send_venue_in_reply(
        &'a self,
        location: (f64, f64),
        title: &'a str,
        address: &'a str,
    ) -> SendVenue<'a> {
        self.send_venue(location, title, address)
            .reply_to_message_id(self.message_id())
    }

    /// Constructs a [`SendVideo`] inferring the token and the chat ID.
    ///
    /// [`SendVideo`]: ../methods/struct.SendVideo.html
    fn send_video(&'a self, video: &'a Video<'a>) -> SendVideo<'a> {
        self.bot().send_video(self.chat_id(), video)
    }

    /// Constructs a [`SendVideo`] inferring the token, chat ID and message ID.
    ///
    /// [`SendVideo`]: ../methods/struct.SendVideo.html
    fn send_video_in_reply(&'a self, video: &'a Video<'a>) -> SendVideo<'a> {
        self.send_video(video).reply_to_message_id(self.message_id())
    }

    /// Constructs a [`SendVideoNote`] inferring the token and the chat ID.
    ///
    /// [`SendVideoNote`]: ../methods/struct.SendVideoNote.html
    fn send_video_note(
        &'a self,
        video_note: &'a VideoNote<'a>,
    ) -> SendVideoNote<'a> {
        self.bot().send_video_note(self.chat_id(), video_note)
    }

    /// Constructs a [`SendVideoNote`] inferring the token, chat ID and message
    /// ID.
    ///
    /// [`SendVideoNote`]: ../methods/struct.SendVideoNote.html
    fn send_video_note_in_reply(
        &'a self,
        video_note: &'a VideoNote<'a>,
    ) -> SendVideoNote<'a> {
        self.send_video_note(video_note).reply_to_message_id(self.message_id())
    }

    /// Constructs a [`SendVoice`] inferring the token and the chat ID.
    ///
    /// [`SendVoice`]: ../methods/struct.SendVoice.html
    fn send_voice(&'a self, voice: &'a Voice<'a>) -> SendVoice<'a> {
        self.bot().send_voice(self.chat_id(), voice)
    }

    /// Constructs a [`SendVoice`] inferring the token, chat ID and message ID.
    ///
    /// [`SendVoice`]: ../methods/struct.SendVoice.html
    fn send_voice_in_reply(&'a self, voice: &'a Voice<'a>) -> SendVoice<'a> {
        self.send_voice(voice).reply_to_message_id(self.message_id())
    }
}

// Type out about 80 method names? No, thanks
#![allow(clippy::wildcard_imports)]
use crate::{
    contexts::fields,
    methods::*,
    types::{
        chat,
        input_file::{
            Animation, Audio, ChatPhoto, Document, EditableMedia, MediaGroup,
            Photo, Sticker, Video, VideoNote, Voice,
        },
        keyboard::inline,
        message,
        parameters::{poll, ImplicitChatId, Text},
        user, LabeledPrice,
    },
};
use std::borrow::Cow;

/// Provides methods appliable to all messages.
pub trait Message: fields::Message {
    /// Copies a message to this chat.
    fn copy_here<'a>(
        &'a self,
        from_chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
    ) -> CopyMessage<'_> {
        self.bot()
            .copy_message(self.chat().id, from_chat_id, message_id)
    }

    /// Copies a message in reply to this message.
    fn copy_here_in_reply<'a>(
        &'a self,
        from_chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
    ) -> CopyMessage<'_> {
        self.bot()
            .copy_message(self.chat().id, from_chat_id, message_id)
            .in_reply_to(self.message_id())
    }

    /// Deletes the photo of this chat.
    fn delete_chat_photo(&self) -> DeleteChatPhoto<'_> {
        self.bot().delete_chat_photo(self.chat().id)
    }

    /// Deletes the sticker set of this chat.
    fn delete_chat_sticker_set(&self) -> DeleteChatStickerSet<'_> {
        self.bot().delete_chat_sticker_set(self.chat().id)
    }

    /// Deletes a message in this chat.
    fn delete_message(&self, message_id: message::Id) -> DeleteMessage<'_> {
        self.bot().delete_message(self.chat().id, message_id)
    }

    /// Deletes the incoming message.
    fn delete_this_message(&self) -> DeleteMessage<'_> {
        self.delete_message(self.message_id())
    }

    /// Updates the caption of a message in this group.
    fn edit_message_caption<'a>(
        &'a self,
        message_id: message::Id,
        caption: impl Into<Text<'a>>,
    ) -> EditMessageCaption<'a> {
        self.bot()
            .edit_message_caption(self.chat().id, message_id, caption)
    }

    /// Updates a live location in this group.
    fn edit_message_location(
        &self,
        message_id: message::Id,
        location: (f64, f64),
    ) -> EditMessageLocation<'_> {
        self.bot()
            .edit_message_location(self.chat().id, message_id, location)
    }

    /// Updates the media of a message in this group.
    fn edit_message_media<'a>(
        &'a self,
        message_id: message::Id,
        media: impl Into<EditableMedia<'a>>,
    ) -> EditMessageMedia<'a> {
        self.bot()
            .edit_message_media(self.chat().id, message_id, media)
    }

    /// Updates the reply markup of a message in this group.
    fn edit_message_reply_markup<'a>(
        &'a self,
        message_id: message::Id,
        reply_markup: inline::Keyboard<'a>,
    ) -> EditMessageReplyMarkup<'a> {
        self.bot().edit_message_reply_markup(
            self.chat().id,
            message_id,
            reply_markup,
        )
    }

    /// Updates the text of a message in this group.
    fn edit_message_text<'a>(
        &'a self,
        message_id: message::Id,
        text: impl Into<Text<'a>>,
    ) -> EditMessageText<'a> {
        self.bot()
            .edit_message_text(self.chat().id, message_id, text)
    }

    /// Exports the invite link of this chat.
    fn export_chat_invite_link(&self) -> ExportChatInviteLink<'_> {
        self.bot().export_chat_invite_link(self.chat().id)
    }

    /// Forwards a message to this chat.
    fn forward_here<'a>(
        &'a self,
        from_chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
    ) -> ForwardMessage<'a> {
        self.bot()
            .forward_message(self.chat().id, from_chat_id, message_id)
    }

    /// Gets information about this chat.
    fn get_chat(&self) -> GetChat<'_> {
        self.bot().get_chat(self.chat().id)
    }

    /// Gets a list of admins of this chat.
    fn get_chat_administrators(&self) -> GetChatAdministrators<'_> {
        self.bot().get_chat_administrators(self.chat().id)
    }

    /// Gets information about a member of this chat.
    fn get_chat_member(&self, user_id: user::Id) -> GetChatMember<'_> {
        self.bot().get_chat_member(self.chat().id, user_id)
    }

    /// Gets the number of members in this chat.
    fn get_chat_members_count(&self) -> GetChatMembersCount<'_> {
        self.bot().get_chat_members_count(self.chat().id)
    }

    /// Gets infomation about high scores in a game sent in this chat.
    fn get_message_game_high_scores(
        &self,
        message_id: message::Id,
        user_id: user::Id,
    ) -> GetMessageGameHighScores<'_> {
        self.bot().get_message_game_high_scores(
            self.chat().id,
            message_id,
            user_id,
        )
    }

    /// Kicks a member of this chat.
    fn kick_chat_member(&self, user_id: user::Id) -> KickChatMember<'_> {
        self.bot().kick_chat_member(self.chat().id, user_id)
    }

    /// Leaves this chat.
    fn leave_chat(&self) -> LeaveChat<'_> {
        self.bot().leave_chat(self.chat().id)
    }

    /// Pins a message in this chat.
    fn pin_chat_message(&self, message_id: message::Id) -> PinChatMessage<'_> {
        self.bot().pin_chat_message(self.chat().id, message_id)
    }

    /// Promotes a member of this chat.
    fn promote_chat_member(&self, user_id: user::Id) -> PromoteChatMember<'_> {
        self.bot().promote_chat_member(self.chat().id, user_id)
    }

    /// Restricts a member of this chat.
    fn restrict_chat_member(
        &self,
        user_id: user::Id,
        permissions: chat::Permissions,
    ) -> RestrictChatMember<'_> {
        self.bot()
            .restrict_chat_member(self.chat().id, user_id, permissions)
    }

    /// Send an animation to this chat.
    fn send_animation<'a>(
        &'a self,
        animation: Animation<'a>,
    ) -> SendAnimation<'a> {
        self.bot().send_animation(self.chat().id, animation)
    }

    /// Sends an animation in reply to this message.
    fn send_animation_in_reply<'a>(
        &'a self,
        animation: Animation<'a>,
    ) -> SendAnimation<'a> {
        self.send_animation(animation)
            .in_reply_to(self.message_id())
    }

    /// Sends an audio to this chat.
    fn send_audio<'a>(&'a self, audio: Audio<'a>) -> SendAudio<'a> {
        self.bot().send_audio(self.chat().id, audio)
    }

    /// Sends an audio in reply to this message.
    fn send_audio_in_reply<'a>(&'a self, audio: Audio<'a>) -> SendAudio<'a> {
        self.send_audio(audio).in_reply_to(self.message_id())
    }

    /// Sends an action to this group.
    fn send_chat_action(&self, action: chat::Action) -> SendChatAction {
        self.bot().send_chat_action(self.chat().id, action)
    }

    /// Sends a contact to this group.
    fn send_contact<'a>(
        &'a self,
        phone_number: impl Into<Cow<'a, str>>,
        first_name: impl Into<Cow<'a, str>>,
    ) -> SendContact<'a> {
        self.bot()
            .send_contact(self.chat().id, phone_number, first_name)
    }

    /// Sends a contact in reply to this message.
    fn send_contact_in_reply<'a>(
        &'a self,
        phone_number: impl Into<Cow<'a, str>>,
        first_name: impl Into<Cow<'a, str>>,
    ) -> SendContact<'a> {
        self.send_contact(phone_number, first_name)
            .in_reply_to(self.message_id())
    }

    /// Sends a game to this chat.
    fn send_game<'a>(
        &'a self,
        game_short_name: impl Into<Cow<'a, str>>,
    ) -> SendGame<'a> {
        self.bot().send_game(self.chat().id, game_short_name)
    }

    /// Sends a game in reply to this message.
    fn send_game_in_reply<'a>(
        &'a self,
        game_short_name: impl Into<Cow<'a, str>>,
    ) -> SendGame<'a> {
        self.send_game(game_short_name)
            .in_reply_to(self.message_id())
    }

    /// Sends a dice to this chat.
    fn send_dice(&self) -> SendDice<'_> {
        self.bot().send_dice(self.chat().id)
    }

    /// Sends a dice in reply to this message.
    fn send_dice_in_reply(&self) -> SendDice<'_> {
        self.send_dice().in_reply_to(self.message_id())
    }

    /// Sends a document to this chat.
    fn send_document<'a>(&'a self, document: Document<'a>) -> SendDocument<'a> {
        self.bot().send_document(self.chat().id, document)
    }

    /// Sends a document in reply to this message.
    fn send_document_in_reply<'a>(
        &'a self,
        document: Document<'a>,
    ) -> SendDocument<'a> {
        self.send_document(document).in_reply_to(self.message_id())
    }

    /// Sends an invoice to this chat.
    #[allow(clippy::too_many_arguments)]
    fn send_invoice<'a>(
        &'a self,
        title: impl Into<Cow<'a, str>>,
        description: impl Into<Cow<'a, str>>,
        payload: impl Into<Cow<'a, str>>,
        provider_token: impl Into<Cow<'a, str>>,
        start_parameter: impl Into<Cow<'a, str>>,
        currency: impl Into<Cow<'a, str>>,
        prices: impl Into<Cow<'a, [LabeledPrice<'a>]>>,
    ) -> SendInvoice<'a> {
        self.bot().send_invoice(
            self.chat().id,
            title,
            description,
            payload,
            provider_token,
            start_parameter,
            currency,
            prices,
        )
    }

    /// Sends an invoice in reply to this message.
    #[allow(clippy::too_many_arguments)]
    fn send_invoice_in_reply<'a>(
        &'a self,
        title: impl Into<Cow<'a, str>>,
        description: impl Into<Cow<'a, str>>,
        payload: impl Into<Cow<'a, str>>,
        provider_token: impl Into<Cow<'a, str>>,
        start_parameter: impl Into<Cow<'a, str>>,
        currency: impl Into<Cow<'a, str>>,
        prices: impl Into<Cow<'a, [LabeledPrice<'a>]>>,
    ) -> SendInvoice<'a> {
        self.send_invoice(
            title,
            description,
            payload,
            provider_token,
            start_parameter,
            currency,
            prices,
        )
        .in_reply_to(self.message_id())
    }

    /// Sends a location to this chat.
    fn send_location(&self, location: (f64, f64)) -> SendLocation {
        self.bot().send_location(self.chat().id, location)
    }

    /// Sends a location in reply to this message.
    fn send_location_in_reply(&self, location: (f64, f64)) -> SendLocation {
        self.send_location(location).in_reply_to(self.message_id())
    }

    /// Sends an album to this chat.
    fn send_media_group<'a>(
        &'a self,
        media: impl Into<MediaGroup<'a>>,
    ) -> SendMediaGroup<'a> {
        self.bot().send_media_group(self.chat().id, media)
    }

    /// Sends an album in reply to this message.
    fn send_media_group_in_reply<'a>(
        &'a self,
        media: impl Into<MediaGroup<'a>>,
    ) -> SendMediaGroup<'a> {
        self.send_media_group(media).in_reply_to(self.message_id())
    }

    /// Sends a message to this chat.
    fn send_message<'a>(
        &'a self,
        text: impl Into<Text<'a>>,
    ) -> SendMessage<'a> {
        self.bot().send_message(self.chat().id, text)
    }

    /// Sends a message in reply to this message.
    fn send_message_in_reply<'a>(
        &'a self,
        text: impl Into<Text<'a>>,
    ) -> SendMessage<'a> {
        self.send_message(text).in_reply_to(self.message_id())
    }

    /// Sends a photo to this chat.
    fn send_photo<'a>(&'a self, photo: Photo<'a>) -> SendPhoto<'a> {
        self.bot().send_photo(self.chat().id, photo)
    }

    /// Sends a photo in reply to this message.
    fn send_photo_in_reply<'a>(&'a self, photo: Photo<'a>) -> SendPhoto<'a> {
        self.send_photo(photo).in_reply_to(self.message_id())
    }

    /// Sends a poll to this chat.
    fn send_poll<'a>(&'a self, poll: poll::Any<'a>) -> SendPoll<'a> {
        self.bot().send_poll(self.chat().id, poll)
    }

    /// Sends a poll in reply to this message.
    fn send_poll_in_reply<'a>(&'a self, poll: poll::Any<'a>) -> SendPoll<'a> {
        self.send_poll(poll).in_reply_to(self.message_id())
    }

    /// Sends a sticker to this chat.
    fn send_sticker<'a>(&'a self, sticker: Sticker<'a>) -> SendSticker<'a> {
        self.bot().send_sticker(self.chat().id, sticker)
    }

    /// Sends a sticker in reply to this message.
    fn send_sticker_in_reply<'a>(
        &'a self,
        sticker: Sticker<'a>,
    ) -> SendSticker<'a> {
        self.send_sticker(sticker).in_reply_to(self.message_id())
    }

    /// Sends a venue to this chat.
    fn send_venue<'a>(
        &'a self,
        location: (f64, f64),
        title: impl Into<Cow<'a, str>>,
        address: impl Into<Cow<'a, str>>,
    ) -> SendVenue<'a> {
        self.bot()
            .send_venue(self.chat().id, location, title, address)
    }

    /// Sends a venue in reply to this message.
    fn send_venue_in_reply<'a>(
        &'a self,
        location: (f64, f64),
        title: impl Into<Cow<'a, str>>,
        address: impl Into<Cow<'a, str>>,
    ) -> SendVenue<'a> {
        self.send_venue(location, title, address)
            .in_reply_to(self.message_id())
    }

    /// Sends a video to this chat.
    fn send_video<'a>(&'a self, video: Video<'a>) -> SendVideo<'a> {
        self.bot().send_video(self.chat().id, video)
    }

    /// Sends a video in reply to this message.
    fn send_video_in_reply<'a>(&'a self, video: Video<'a>) -> SendVideo<'a> {
        self.send_video(video).in_reply_to(self.message_id())
    }

    /// Sends a video note to this chat.
    fn send_video_note<'a>(
        &'a self,
        video_note: VideoNote<'a>,
    ) -> SendVideoNote<'a> {
        self.bot().send_video_note(self.chat().id, video_note)
    }

    /// Sends a video note in reply to this message.
    fn send_video_note_in_reply<'a>(
        &'a self,
        video_note: VideoNote<'a>,
    ) -> SendVideoNote<'a> {
        self.send_video_note(video_note)
            .in_reply_to(self.message_id())
    }

    /// Sends a voice to this chat.
    fn send_voice<'a>(&'a self, voice: Voice<'a>) -> SendVoice<'a> {
        self.bot().send_voice(self.chat().id, voice)
    }

    /// Sends a voice in reply to this message.
    fn send_voice_in_reply<'a>(&'a self, voice: Voice<'a>) -> SendVoice<'a> {
        self.send_voice(voice).in_reply_to(self.message_id())
    }

    /// Sets a custom title for an admin in this chat.
    fn set_chat_administrator_custom_title<'a>(
        &'a self,
        user_id: user::Id,
        custom_title: impl Into<Cow<'a, str>>,
    ) -> SetChatAdministratorCustomTitle<'a> {
        self.bot().set_chat_administrator_custom_title(
            self.chat().id,
            user_id,
            custom_title,
        )
    }

    /// Sets a new description of this chat.
    fn set_chat_description<'a>(
        &'a self,
        description: impl Into<Cow<'a, str>>,
    ) -> SetChatDescription<'a> {
        self.bot().set_chat_description(self.chat().id, description)
    }

    /// Sets new permissions of this chat.
    fn set_chat_permissions(
        &self,
        permissions: chat::Permissions,
    ) -> SetChatPermissions<'_> {
        self.bot().set_chat_permissions(self.chat().id, permissions)
    }

    /// Sets a new photo of this chat.
    fn set_chat_photo<'a>(&'a self, photo: ChatPhoto<'a>) -> SetChatPhoto<'a> {
        self.bot().set_chat_photo(self.chat().id, photo)
    }

    /// Sets a new sticker set of this chat.
    fn set_chat_sticker_set<'a>(
        &'a self,
        sticker_set_name: impl Into<Cow<'a, str>>,
    ) -> SetChatStickerSet<'a> {
        self.bot()
            .set_chat_sticker_set(self.chat().id, sticker_set_name)
    }

    /// Sets a new chat title of this chat.
    fn set_chat_title<'a>(
        &'a self,
        title: impl Into<Cow<'a, str>>,
    ) -> SetChatTitle<'a> {
        self.bot().set_chat_title(self.chat().id, title)
    }

    /// Sets a new high score for a player who played a game in this chat.
    fn set_message_game_score(
        &self,
        message_id: message::Id,
        user_id: user::Id,
        score: u32,
    ) -> SetMessageGameScore<'_> {
        self.bot().set_message_game_score(
            self.chat().id,
            message_id,
            user_id,
            score,
        )
    }

    /// Unbans a member of this chat.
    fn unban_chat_member(&self, user_id: user::Id) -> UnbanChatMember<'_> {
        self.bot().unban_chat_member(self.chat().id, user_id)
    }

    /// Unpins all messages in this chat.
    fn unpin_all_chat_messages(&self) -> UnpinAllChatMessages<'_> {
        self.bot().unpin_all_chat_messages(self.chat().id)
    }

    /// Unpins the pinned message in this chat.
    fn unpin_chat_message(&self) -> UnpinChatMessage<'_> {
        self.bot().unpin_chat_message(self.chat().id)
    }
}

impl<T: fields::Message> Message for T {}

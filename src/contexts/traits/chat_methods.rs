use crate::{
    contexts::fields::Message,
    methods::*,
    types::{
        chat,
        input_file::{
            Animation, Audio, ChatPhoto, Document, EditableMedia, GroupMedia,
            Photo, Sticker, Video, VideoNote, Voice,
        },
        keyboard::inline,
        message,
        parameters::{ImplicitChatId, Text},
        user, LabeledPrice,
    },
};

/// Provides methods appliable to all messages.
pub trait ChatMethods<'a, C: 'static>: Message<C> {
    /// Deletes the photo of this chat.
    fn delete_chat_photo(&'a self) -> DeleteChatPhoto<'a, C> {
        self.bot().delete_chat_photo(self.chat().id)
    }

    /// Deletes the sticker set of this chat.
    fn delete_chat_sticker_set(&'a self) -> DeleteChatStickerSet<'a, C> {
        self.bot().delete_chat_sticker_set(self.chat().id)
    }

    /// Deletes a message in this chat.
    fn delete_message(
        &'a self,
        message_id: message::Id,
    ) -> DeleteMessage<'a, C> {
        self.bot().delete_message(self.chat().id, message_id)
    }

    /// Deletes the incoming message.
    fn delete_this_message(&'a self) -> DeleteMessage<'a, C> {
        self.delete_message(self.message_id())
    }

    /// Updates the caption of a message in this group.
    fn edit_message_caption(
        &'a self,
        message_id: message::Id,
        caption: impl Into<Text<'a>>,
    ) -> EditMessageCaption<'a, C> {
        self.bot()
            .edit_message_caption(self.chat().id, message_id, caption)
    }

    /// Updates a live location in this group.
    fn edit_message_location(
        &'a self,
        message_id: message::Id,
        location: (f64, f64),
    ) -> EditMessageLocation<'a, C> {
        self.bot()
            .edit_message_location(self.chat().id, message_id, location)
    }

    /// Updates the media of a message in this group.
    fn edit_message_media(
        &'a self,
        message_id: message::Id,
        media: impl Into<EditableMedia<'a>>,
    ) -> EditMessageMedia<'a, C> {
        self.bot()
            .edit_message_media(self.chat().id, message_id, media)
    }

    /// Updates the reply markup of a message in this group.
    fn edit_message_reply_markup(
        &'a self,
        message_id: message::Id,
        reply_markup: inline::Keyboard<'a>,
    ) -> EditMessageReplyMarkup<'a, C> {
        self.bot().edit_message_reply_markup(
            self.chat().id,
            message_id,
            reply_markup,
        )
    }

    /// Updates the text of a message in this group.
    fn edit_message_text(
        &'a self,
        message_id: message::Id,
        text: impl Into<Text<'a>>,
    ) -> EditMessageText<'a, C> {
        self.bot()
            .edit_message_text(self.chat().id, message_id, text)
    }

    /// Exports the invite link of this chat.
    fn export_chat_invite_link(&'a self) -> ExportChatInviteLink<'a, C> {
        self.bot().export_chat_invite_link(self.chat().id)
    }

    /// Forwards a message to this chat.
    fn forward_here(
        &'a self,
        from_chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
    ) -> ForwardMessage<'a, C> {
        self.bot()
            .forward_message(self.chat().id, from_chat_id, message_id)
    }

    /// Gets information about this chat.
    fn get_chat(&'a self) -> GetChat<'a, C> {
        self.bot().get_chat(self.chat().id)
    }

    /// Gets a list of admins of this chat.
    fn get_chat_administrators(&'a self) -> GetChatAdministrators<'a, C> {
        self.bot().get_chat_administrators(self.chat().id)
    }

    /// Gets information about a member of this chat.
    fn get_chat_member(&'a self, user_id: user::Id) -> GetChatMember<'a, C> {
        self.bot().get_chat_member(self.chat().id, user_id)
    }

    /// Gets the number of members in this chat.
    fn get_chat_members_count(&'a self) -> GetChatMembersCount<'a, C> {
        self.bot().get_chat_members_count(self.chat().id)
    }

    /// Gets infomation about high scores in a game sent in this chat.
    fn get_message_game_high_scores(
        &'a self,
        message_id: message::Id,
        user_id: user::Id,
    ) -> GetMessageGameHighScores<'a, C> {
        self.bot().get_message_game_high_scores(
            self.chat().id,
            message_id,
            user_id,
        )
    }

    /// Kicks a member of this chat.
    fn kick_chat_member(&'a self, user_id: user::Id) -> KickChatMember<'a, C> {
        self.bot().kick_chat_member(self.chat().id, user_id)
    }

    /// Leaves this chat.
    fn leave_chat(&'a self) -> LeaveChat<'a, C> {
        self.bot().leave_chat(self.chat().id)
    }

    /// Pins a message in this chat.
    fn pin_chat_message(
        &'a self,
        message_id: message::Id,
    ) -> PinChatMessage<'a, C> {
        self.bot().pin_chat_message(self.chat().id, message_id)
    }

    /// Promotes a member of this chat.
    fn promote_chat_member(
        &'a self,
        user_id: user::Id,
    ) -> PromoteChatMember<'a, C> {
        self.bot().promote_chat_member(self.chat().id, user_id)
    }

    /// Restricts a member of this chat.
    fn restrict_chat_member(
        &'a self,
        user_id: user::Id,
        permissions: chat::Permissions,
    ) -> RestrictChatMember<'a, C> {
        self.bot()
            .restrict_chat_member(self.chat().id, user_id, permissions)
    }

    /// Send an animation to this chat.
    fn send_animation(
        &'a self,
        animation: Animation<'a>,
    ) -> SendAnimation<'a, C> {
        self.bot().send_animation(self.chat().id, animation)
    }

    /// Sends an animation in reply to this message.
    fn send_animation_in_reply(
        &'a self,
        animation: Animation<'a>,
    ) -> SendAnimation<'a, C> {
        self.send_animation(animation)
            .reply_to_message_id(self.message_id())
    }

    /// Sends an audio to this chat.
    fn send_audio(&'a self, audio: Audio<'a>) -> SendAudio<'a, C> {
        self.bot().send_audio(self.chat().id, audio)
    }

    /// Sends an audio in reply to this message.
    fn send_audio_in_reply(&'a self, audio: Audio<'a>) -> SendAudio<'a, C> {
        self.send_audio(audio)
            .reply_to_message_id(self.message_id())
    }

    /// Sends an action to this group.
    fn send_chat_action(&'a self, action: chat::Action) -> SendChatAction<C> {
        self.bot().send_chat_action(self.chat().id, action)
    }

    /// Sends a contact to this group.
    fn send_contact(
        &'a self,
        phone_number: &'a str,
        first_name: &'a str,
    ) -> SendContact<'a, C> {
        self.bot()
            .send_contact(self.chat().id, phone_number, first_name)
    }

    /// Sends a contact in reply to this message.
    fn send_contact_in_reply(
        &'a self,
        phone_number: &'a str,
        first_name: &'a str,
    ) -> SendContact<'a, C> {
        self.send_contact(phone_number, first_name)
            .reply_to_message_id(self.message_id())
    }

    /// Sends a game to this chat.
    fn send_game(&'a self, game_short_name: &'a str) -> SendGame<'a, C> {
        self.bot().send_game(self.chat().id, game_short_name)
    }

    /// Sends a game in reply to this message.
    fn send_game_in_reply(
        &'a self,
        game_short_name: &'a str,
    ) -> SendGame<'a, C> {
        self.send_game(game_short_name)
            .reply_to_message_id(self.message_id())
    }

    /// Sends a document to this chat.
    fn send_document(&'a self, document: Document<'a>) -> SendDocument<'a, C> {
        self.bot().send_document(self.chat().id, document)
    }

    /// Sends a document in reply to this message.
    fn send_document_in_reply(
        &'a self,
        document: Document<'a>,
    ) -> SendDocument<'a, C> {
        self.send_document(document)
            .reply_to_message_id(self.message_id())
    }

    /// Sends an invoice to this chat.
    #[allow(clippy::too_many_arguments)]
    fn send_invoice(
        &'a self,
        title: &'a str,
        description: &'a str,
        payload: &'a str,
        provider_token: &'a str,
        start_parameter: &'a str,
        currency: &'a str,
        prices: &'a [LabeledPrice<'a>],
    ) -> SendInvoice<'a, C> {
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
    fn send_invoice_in_reply(
        &'a self,
        title: &'a str,
        description: &'a str,
        payload: &'a str,
        provider_token: &'a str,
        start_parameter: &'a str,
        currency: &'a str,
        prices: &'a [LabeledPrice<'a>],
    ) -> SendInvoice<'a, C> {
        self.send_invoice(
            title,
            description,
            payload,
            provider_token,
            start_parameter,
            currency,
            prices,
        )
        .reply_to_message_id(self.message_id())
    }

    /// Sends a location to this chat.
    fn send_location(&'a self, location: (f64, f64)) -> SendLocation<C> {
        self.bot().send_location(self.chat().id, location)
    }

    /// Sends a location in reply to this message.
    fn send_location_in_reply(
        &'a self,
        location: (f64, f64),
    ) -> SendLocation<C> {
        self.send_location(location)
            .reply_to_message_id(self.message_id())
    }

    /// Sends an album to this chat.
    fn send_media_group(
        &'a self,
        media: &'a [GroupMedia<'a>],
    ) -> SendMediaGroup<'a, C> {
        self.bot().send_media_group(self.chat().id, media)
    }

    /// Sends an album in reply to this message.
    fn send_media_group_in_reply(
        &'a self,
        media: &'a [GroupMedia<'a>],
    ) -> SendMediaGroup<'a, C> {
        self.send_media_group(media)
            .reply_to_message_id(self.message_id())
    }

    /// Sends a message to this chat.
    fn send_message(&'a self, text: impl Into<Text<'a>>) -> SendMessage<'a, C> {
        self.bot().send_message(self.chat().id, text)
    }

    /// Sends a message in reply to this message.
    fn send_message_in_reply(
        &'a self,
        text: impl Into<Text<'a>>,
    ) -> SendMessage<'a, C> {
        self.send_message(text)
            .reply_to_message_id(self.message_id())
    }

    /// Sends a photo to this chat.
    fn send_photo(&'a self, photo: Photo<'a>) -> SendPhoto<'a, C> {
        self.bot().send_photo(self.chat().id, photo)
    }

    /// Sends a photo in reply to this message.
    fn send_photo_in_reply(&'a self, photo: Photo<'a>) -> SendPhoto<'a, C> {
        self.send_photo(photo)
            .reply_to_message_id(self.message_id())
    }

    /// Sends a poll to this chat.
    fn send_poll(
        &'a self,
        question: &'a str,
        options: &'a [&'a str],
    ) -> SendPoll<'a, C> {
        self.bot().send_poll(self.chat().id, question, options)
    }

    /// Sends a poll in reply to this message.
    fn send_poll_in_reply(
        &'a self,
        question: &'a str,
        options: &'a [&'a str],
    ) -> SendPoll<'a, C> {
        self.send_poll(question, options)
            .reply_to_message_id(self.message_id())
    }

    /// Sends a sticker to this chat.
    fn send_sticker(&'a self, sticker: Sticker<'a>) -> SendSticker<'a, C> {
        self.bot().send_sticker(self.chat().id, sticker)
    }

    /// Sends a sticker in reply to this message.
    fn send_sticker_in_reply(
        &'a self,
        sticker: Sticker<'a>,
    ) -> SendSticker<'a, C> {
        self.send_sticker(sticker)
            .reply_to_message_id(self.message_id())
    }

    /// Sends a venue to this chat.
    fn send_venue(
        &'a self,
        location: (f64, f64),
        title: &'a str,
        address: &'a str,
    ) -> SendVenue<'a, C> {
        self.bot()
            .send_venue(self.chat().id, location, title, address)
    }

    /// Sends a venue in reply to this message.
    fn send_venue_in_reply(
        &'a self,
        location: (f64, f64),
        title: &'a str,
        address: &'a str,
    ) -> SendVenue<'a, C> {
        self.send_venue(location, title, address)
            .reply_to_message_id(self.message_id())
    }

    /// Sends a video to this chat.
    fn send_video(&'a self, video: Video<'a>) -> SendVideo<'a, C> {
        self.bot().send_video(self.chat().id, video)
    }

    /// Sends a video in reply to this message.
    fn send_video_in_reply(&'a self, video: Video<'a>) -> SendVideo<'a, C> {
        self.send_video(video)
            .reply_to_message_id(self.message_id())
    }

    /// Sends a video note to this chat.
    fn send_video_note(
        &'a self,
        video_note: VideoNote<'a>,
    ) -> SendVideoNote<'a, C> {
        self.bot().send_video_note(self.chat().id, video_note)
    }

    /// Sends a video note in reply to this message.
    fn send_video_note_in_reply(
        &'a self,
        video_note: VideoNote<'a>,
    ) -> SendVideoNote<'a, C> {
        self.send_video_note(video_note)
            .reply_to_message_id(self.message_id())
    }

    /// Sends a voice to this chat.
    fn send_voice(&'a self, voice: Voice<'a>) -> SendVoice<'a, C> {
        self.bot().send_voice(self.chat().id, voice)
    }

    /// Sends a voice in reply to this message.
    fn send_voice_in_reply(&'a self, voice: Voice<'a>) -> SendVoice<'a, C> {
        self.send_voice(voice)
            .reply_to_message_id(self.message_id())
    }

    /// Sets a custom title for an admin in this chat.
    fn set_chat_administrator_custom_title(
        &'a self,
        user_id: user::Id,
        custom_title: &'a str,
    ) -> SetChatAdministratorCustomTitle<'a, C> {
        self.bot().set_chat_administrator_custom_title(self.chat().id, user_id, custom_title)
    }

    /// Sets a new description of this chat.
    fn set_chat_description(
        &'a self,
        description: &'a str,
    ) -> SetChatDescription<'a, C> {
        self.bot().set_chat_description(self.chat().id, description)
    }

    /// Sets new permissions of this chat.
    fn set_chat_permissions(
        &'a self,
        permissions: chat::Permissions,
    ) -> SetChatPermissions<'a, C> {
        self.bot().set_chat_permissions(self.chat().id, permissions)
    }

    /// Sets a new photo of this chat.
    fn set_chat_photo(&'a self, photo: ChatPhoto<'a>) -> SetChatPhoto<'a, C> {
        self.bot().set_chat_photo(self.chat().id, photo)
    }

    /// Sets a new sticker set of this chat.
    fn set_chat_sticker_set(
        &'a self,
        sticker_set_name: &'a str,
    ) -> SetChatStickerSet<'a, C> {
        self.bot()
            .set_chat_sticker_set(self.chat().id, sticker_set_name)
    }

    /// Sets a new chat title of this chat.
    fn set_chat_title(&'a self, title: &'a str) -> SetChatTitle<'a, C> {
        self.bot().set_chat_title(self.chat().id, title)
    }

    /// Sets a new high score for a player who played a game in this chat.
    fn set_message_game_score(
        &'a self,
        message_id: message::Id,
        user_id: user::Id,
        score: u32,
    ) -> SetMessageGameScore<'a, C> {
        self.bot().set_message_game_score(
            self.chat().id,
            message_id,
            user_id,
            score,
        )
    }

    /// Unbans a member of this chat.
    fn unban_chat_member(
        &'a self,
        user_id: user::Id,
    ) -> UnbanChatMember<'a, C> {
        self.bot().unban_chat_member(self.chat().id, user_id)
    }

    /// Unpins the pinned message in this chat.
    fn unpin_chat_message(&'a self) -> UnpinChatMessage<'a, C> {
        self.bot().unpin_chat_message(self.chat().id)
    }
}

impl<'a, C: 'static, T: Message<C>> ChatMethods<'a, C> for T {}

use super::*;
use types::input_file::*;

/// Provides methods for calling API that infer your bot's token.
pub trait Methods<'a>: crate::Sealed {
    #[doc(hidden)]
    fn token(&self) -> &str;

    #[cfg(feature = "proxy")]
    #[doc(hidden)]
    fn get_proxy(&self) -> Option<proxy::Proxy>;

    #[cfg(feature = "proxy")]
    #[doc(hidden)]
    fn prepare_method<T: ProxyMethod>(&self, method: T) -> T {
        if let Some(proxy) = self.get_proxy() {
            method.proxy(proxy)
        } else {
            method
        }
    }

    #[cfg(not(feature = "proxy"))]
    #[doc(hidden)]
    fn prepare_method<T>(&self, method: T) -> T {
        method
    }

    /// Constructs a new `AddStickerToSet` inferring your bot's token.
    fn add_sticker_to_set(
        &'a self,
        user_id: i64,
        name: &'a str,
        png_sticker: &'a PngSticker<'a>,
        emojis: &'a str,
    ) -> methods::AddStickerToSet<'a> {
        self.prepare_method(methods::AddStickerToSet::new(
            self.token(),
            user_id,
            name,
            png_sticker,
            emojis,
        ))
    }

    /// Constructs a new `AnswerCallbackQuery` inferring your bot's token.
    fn answer_callback_query(
        &'a self,
        callback_query_id: &'a str,
        action: CallbackAnswerAction<'a>,
    ) -> methods::AnswerCallbackQuery<'a> {
        self.prepare_method(methods::AnswerCallbackQuery::new(
            self.token(),
            callback_query_id,
            action,
        ))
    }

    /// Constructs a new `CreateNewStickerSet` inferring your bot's token.
    fn create_new_sticker_set(
        &'a self,
        user_id: i64,
        name: &'a str,
        title: &'a str,
        png_sticker: &'a PngSticker<'a>,
        emojis: &'a str,
    ) -> methods::CreateNewStickerSet<'a> {
        self.prepare_method(methods::CreateNewStickerSet::new(
            self.token(),
            user_id,
            name,
            title,
            png_sticker,
            emojis,
        ))
    }

    /// Constructs a new `DeleteChatPhoto` inferring your bot's token.
    fn delete_chat_photo(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
    ) -> methods::DeleteChatPhoto<'a> {
        self.prepare_method(methods::DeleteChatPhoto::new(
            self.token(),
            chat_id,
        ))
    }

    /// Constructs a new `DeleteChatStickerSet` inferring your bot's token.
    fn delete_chat_sticker_set(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
    ) -> methods::DeleteChatStickerSet<'a> {
        self.prepare_method(methods::DeleteChatStickerSet::new(
            self.token(),
            chat_id,
        ))
    }

    /// Constructs a new `DeleteMessage` inferring your bot's token.
    fn delete_message(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
    ) -> methods::DeleteMessage<'a> {
        self.prepare_method(methods::DeleteMessage::new(
            self.token(),
            chat_id,
            message_id,
        ))
    }

    /// Constructs a new `DeleteStickerFromSet` inferring your bot's token.
    fn delete_sticker_from_set(
        &'a self,
        sticker: &'a str,
    ) -> methods::DeleteStickerFromSet<'a> {
        self.prepare_method(methods::DeleteStickerFromSet::new(
            self.token(),
            sticker,
        ))
    }

    /// Constructs a new `EditInlineCaption` inferring your bot's token.
    fn edit_inline_caption(
        &'a self,
        inline_message_id: &'a str,
        caption: &'a str,
    ) -> methods::EditInlineCaption<'a> {
        self.prepare_method(methods::EditInlineCaption::new(
            self.token(),
            inline_message_id,
            caption,
        ))
    }

    /// Constructs a new `EditInlineLocation` inferring your bot's token.
    fn edit_inline_location(
        &'a self,
        inline_message_id: &'a str,
        position: (f64, f64),
    ) -> methods::EditInlineLocation<'a> {
        self.prepare_method(methods::EditInlineLocation::new(
            self.token(),
            inline_message_id,
            position,
        ))
    }

    /// Constructs a new `EditInlineMedia` inferring your bot's token.
    fn edit_inline_media(
        &'a self,
        inline_message_id: &'a str,
        media: impl Into<EditableMedia<'a>>,
    ) -> methods::EditInlineMedia<'a> {
        self.prepare_method(methods::EditInlineMedia::new(
            self.token(),
            inline_message_id,
            media,
        ))
    }

    /// Constructs a new `EditInlineReplyMarkup` inferring your bot's token.
    fn edit_inline_reply_markup(
        &'a self,
        inline_message_id: &'a str,
        reply_markup: types::InlineKeyboard<'a>,
    ) -> methods::EditInlineReplyMarkup<'a> {
        self.prepare_method(methods::EditInlineReplyMarkup::new(
            self.token(),
            inline_message_id,
            reply_markup,
        ))
    }

    /// Constructs a new `EditInlineText` inferring your bot's token.
    fn edit_inline_text(
        &'a self,
        inline_message_id: &'a str,
        text: &'a str,
    ) -> methods::EditInlineText<'a> {
        self.prepare_method(methods::EditInlineText::new(
            self.token(),
            inline_message_id,
            text,
        ))
    }

    /// Constructs a new `EditMessageCaption` inferring your bot's token.
    fn edit_message_caption(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
        caption: &'a str,
    ) -> methods::EditMessageCaption<'a> {
        self.prepare_method(methods::EditMessageCaption::new(
            self.token(),
            chat_id,
            message_id,
            caption,
        ))
    }

    /// Constructs a new `EditMessageLocation` inferring your bot's token.
    fn edit_message_location(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
        position: (f64, f64),
    ) -> methods::EditMessageLocation<'a> {
        self.prepare_method(methods::EditMessageLocation::new(
            self.token(),
            chat_id,
            message_id,
            position,
        ))
    }

    /// Constructs a new `EditMessageMedia` inferring your bot's token.
    fn edit_message_media(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
        media: impl Into<EditableMedia<'a>>,
    ) -> methods::EditMessageMedia<'a> {
        self.prepare_method(methods::EditMessageMedia::new(
            self.token(),
            chat_id,
            message_id,
            media,
        ))
    }

    /// Constructs a new `EditMessageReplyMarkup` inferring your bot's token.
    fn edit_message_reply_markup(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
        reply_markup: types::InlineKeyboard<'a>,
    ) -> methods::EditMessageReplyMarkup<'a> {
        self.prepare_method(methods::EditMessageReplyMarkup::new(
            self.token(),
            chat_id,
            message_id,
            reply_markup,
        ))
    }

    /// Constructs a new `EditMessageText` inferring your bot's token.
    fn edit_message_text(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
        text: &'a str,
    ) -> methods::EditMessageText<'a> {
        self.prepare_method(methods::EditMessageText::new(
            self.token(),
            chat_id,
            message_id,
            text,
        ))
    }

    /// Constructs a new `ExportChatInviteLink` inferring your bot's token.
    fn export_chat_invite_link(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
    ) -> methods::ExportChatInviteLink<'a> {
        self.prepare_method(methods::ExportChatInviteLink::new(
            self.token(),
            chat_id,
        ))
    }

    /// Constructs a new `ForwardMessage` inferring your bot's token.
    fn forward_message(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        from_chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
    ) -> methods::ForwardMessage<'a> {
        self.prepare_method(methods::ForwardMessage::new(
            self.token(),
            chat_id,
            from_chat_id,
            message_id,
        ))
    }

    /// Constructs a new `GetChat` inferring your bot's token.
    fn get_chat(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
    ) -> methods::GetChat<'a> {
        self.prepare_method(methods::GetChat::new(self.token(), chat_id))
    }

    /// Constructs a new `GetInlineGameHighScores` inferring your bot's token.
    fn get_inline_game_high_scores(
        &'a self,
        inline_message_id: &'a str,
        user_id: i64,
    ) -> methods::GetInlineGameHighScores<'a> {
        self.prepare_method(methods::GetInlineGameHighScores::new(
            self.token(),
            inline_message_id,
            user_id,
        ))
    }

    /// Constructs a new `GetChatAdministrators` inferring your bot's token.
    fn get_chat_administrators(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
    ) -> methods::GetChatAdministrators<'a> {
        self.prepare_method(methods::GetChatAdministrators::new(
            self.token(),
            chat_id,
        ))
    }

    /// Constructs a new `GetChatMember` inferring your bot's token.
    fn get_chat_member(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        user_id: i64,
    ) -> methods::GetChatMember<'a> {
        self.prepare_method(methods::GetChatMember::new(
            self.token(),
            chat_id,
            user_id,
        ))
    }

    /// Constructs a new `GetChatMembersCount` inferring your bot's token.
    fn get_chat_members_count(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
    ) -> methods::GetChatMembersCount<'a> {
        self.prepare_method(methods::GetChatMembersCount::new(
            self.token(),
            chat_id,
        ))
    }

    /// Constructs a new `GetMessageGameHighScores` inferring your bot's token.
    fn get_message_game_high_scores(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
        user_id: i64,
    ) -> methods::GetMessageGameHighScores<'a> {
        self.prepare_method(methods::GetMessageGameHighScores::new(
            self.token(),
            chat_id,
            message_id,
            user_id,
        ))
    }

    /// Constructs a new `GetMe` inferring your bot's token.
    fn get_me(&'a self) -> methods::GetMe<'a> {
        self.prepare_method(methods::GetMe::new(self.token()))
    }

    /// Constructs a new `GetStickerSet` inferring your bot's token.
    fn get_sticker_set(&'a self, name: &'a str) -> methods::GetStickerSet<'a> {
        self.prepare_method(methods::GetStickerSet::new(self.token(), name))
    }

    /// Constructs a new `GetUserProfilePhotos` inferring your bot's token.
    fn get_user_profile_photos(
        &'a self,
        user_id: i64,
    ) -> methods::GetUserProfilePhotos<'a> {
        self.prepare_method(methods::GetUserProfilePhotos::new(
            self.token(),
            user_id,
        ))
    }

    /// Constructs a new `GetWebhookInfo` inferring your bot's token.
    fn get_webhook_info(&'a self) -> methods::GetWebhookInfo<'a> {
        self.prepare_method(methods::GetWebhookInfo::new(self.token()))
    }

    /// Constructs a new `KickChatMember` inferring your bot's token.
    fn kick_chat_member(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        user_id: i64,
    ) -> methods::KickChatMember<'a> {
        self.prepare_method(methods::KickChatMember::new(
            self.token(),
            chat_id,
            user_id,
        ))
    }

    /// Constructs a new `LeaveChat` inferring your bot's token.
    fn leave_chat(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
    ) -> methods::LeaveChat<'a> {
        self.prepare_method(methods::LeaveChat::new(self.token(), chat_id))
    }

    /// Constructs a new `PinChatMessage` inferring your bot's token.
    fn pin_chat_message(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
    ) -> methods::PinChatMessage<'a> {
        self.prepare_method(methods::PinChatMessage::new(
            self.token(),
            chat_id,
            message_id,
        ))
    }

    /// Constructs a new `PromoteChatMember` inferring your bot's token.
    fn promote_chat_member(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        user_id: i64,
    ) -> methods::PromoteChatMember<'a> {
        self.prepare_method(methods::PromoteChatMember::new(
            self.token(),
            chat_id,
            user_id,
        ))
    }

    /// Constructs a new `RestrictChatMember` inferring your bot's token.
    fn restrict_chat_member(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        user_id: i64,
    ) -> methods::RestrictChatMember<'a> {
        self.prepare_method(methods::RestrictChatMember::new(
            self.token(),
            chat_id,
            user_id,
        ))
    }

    /// Constructs a new `SendAnimation` inferring your bot's token.
    fn send_animation(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        animation: &'a Animation<'a>,
    ) -> methods::SendAnimation<'a> {
        self.prepare_method(methods::SendAnimation::new(
            self.token(),
            chat_id,
            animation,
        ))
    }

    /// Constructs a new `SendAudio` inferring your bot's token.
    fn send_audio(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        audio: &'a Audio<'a>,
    ) -> methods::SendAudio<'a> {
        self.prepare_method(methods::SendAudio::new(
            self.token(),
            chat_id,
            audio,
        ))
    }

    /// Constructs a new `SendChatAction` inferring your bot's token.
    fn send_chat_action(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        action: types::ChatAction,
    ) -> methods::SendChatAction<'a> {
        self.prepare_method(methods::SendChatAction::new(
            self.token(),
            chat_id,
            action,
        ))
    }

    /// Constructs a new `SendContact` inferring your bot's token.
    fn send_contact(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        phone_number: &'a str,
        first_name: &'a str,
    ) -> methods::SendContact<'a> {
        self.prepare_method(methods::SendContact::new(
            self.token(),
            chat_id,
            phone_number,
            first_name,
        ))
    }

    /// Constructs a new `SendGame` inferring your bot's token.
    fn send_game(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        game_short_name: &'a str,
    ) -> methods::SendGame<'a> {
        self.prepare_method(methods::SendGame::new(
            self.token(),
            chat_id,
            game_short_name,
        ))
    }

    /// Constructs a new `SendDocument` inferring your bot's token.
    fn send_document(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        document: &'a Document<'a>,
    ) -> methods::SendDocument<'a> {
        self.prepare_method(methods::SendDocument::new(
            self.token(),
            chat_id,
            document,
        ))
    }

    /// Constructs a new `SendLocation` inferring your bot's token.
    fn send_location(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        position: (f64, f64),
    ) -> methods::SendLocation<'a> {
        self.prepare_method(methods::SendLocation::new(
            self.token(),
            chat_id,
            position,
        ))
    }

    /// Constructs a new `SendMediaGroup` inferring your bot's token.
    fn send_media_group(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        media: Vec<GroupMedia<'a>>,
    ) -> methods::SendMediaGroup<'a> {
        self.prepare_method(methods::SendMediaGroup::new(
            self.token(),
            chat_id,
            media,
        ))
    }

    /// Constructs a new `SendMessage` inferring your bot's token.
    fn send_message(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        text: &'a str,
    ) -> methods::SendMessage<'a> {
        self.prepare_method(methods::SendMessage::new(
            self.token(),
            chat_id,
            text,
        ))
    }

    /// Constructs a new `SendPhoto` inferring your bot's token.
    fn send_photo(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        photo: &'a Photo<'a>,
    ) -> methods::SendPhoto<'a> {
        self.prepare_method(methods::SendPhoto::new(
            self.token(),
            chat_id,
            photo,
        ))
    }

    /// Constructs a new `SendPoll` inferring your bot's token.
    fn send_poll(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        question: &'a str,
        options: &'a [&'a str],
    ) -> methods::SendPoll<'a> {
        self.prepare_method(methods::SendPoll::new(
            self.token(),
            chat_id,
            question,
            options,
        ))
    }

    /// Constructs a new `SendSticker` inferring your bot's token.
    fn send_sticker(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        sticker: &'a Sticker<'a>,
    ) -> methods::SendSticker<'a> {
        self.prepare_method(methods::SendSticker::new(
            self.token(),
            chat_id,
            sticker,
        ))
    }

    /// Constructs a new `SendVenue` inferring your bot's token.
    fn send_venue(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        position: (f64, f64),
        title: &'a str,
        address: &'a str,
    ) -> methods::SendVenue<'a> {
        self.prepare_method(methods::SendVenue::new(
            self.token(),
            chat_id,
            position,
            title,
            address,
        ))
    }

    /// Constructs a new `SendVideoNote` inferring your bot's token.
    fn send_video_note(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        video_note: &'a VideoNote<'a>,
    ) -> methods::SendVideoNote<'a> {
        self.prepare_method(methods::SendVideoNote::new(
            self.token(),
            chat_id,
            video_note,
        ))
    }

    /// Constructs a new `SendVideo` inferring your bot's token.
    fn send_video(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        video: &'a Video<'a>,
    ) -> methods::SendVideo<'a> {
        self.prepare_method(methods::SendVideo::new(
            self.token(),
            chat_id,
            video,
        ))
    }

    /// Constructs a new `SendVoice` inferring your bot's token.
    fn send_voice(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        voice: &'a Voice<'a>,
    ) -> methods::SendVoice<'a> {
        self.prepare_method(methods::SendVoice::new(
            self.token(),
            chat_id,
            voice,
        ))
    }

    /// Constructs a new `SetChatDescription` inferring your bot's token.
    fn set_chat_description(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        description: &'a str,
    ) -> methods::SetChatDescription<'a> {
        self.prepare_method(methods::SetChatDescription::new(
            self.token(),
            chat_id,
            description,
        ))
    }

    /// Constructs a new `SetChatPhoto` inferring your bot's token.
    fn set_chat_photo(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        photo: &'a ChatPhoto<'a>,
    ) -> methods::SetChatPhoto<'a> {
        self.prepare_method(methods::SetChatPhoto::new(
            self.token(),
            chat_id,
            photo,
        ))
    }

    /// Constructs a new `SetChatStickerSet` inferring your bot's token.
    fn set_chat_sticker_set(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        sticker_set_name: &'a str,
    ) -> methods::SetChatStickerSet<'a> {
        self.prepare_method(methods::SetChatStickerSet::new(
            self.token(),
            chat_id,
            sticker_set_name,
        ))
    }

    /// Constructs a new `SetChatTitle` inferring your bot's token.
    fn set_chat_title(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        title: &'a str,
    ) -> methods::SetChatTitle<'a> {
        self.prepare_method(methods::SetChatTitle::new(
            self.token(),
            chat_id,
            title,
        ))
    }

    /// Constructs a new `SetInlineGameScore` inferring your bot's token.
    fn set_inline_game_score(
        &'a self,
        inline_message_id: &'a str,
        user_id: i64,
        score: u32,
    ) -> methods::SetInlineGameScore<'a> {
        self.prepare_method(methods::SetInlineGameScore::new(
            self.token(),
            inline_message_id,
            user_id,
            score,
        ))
    }

    /// Constructs a new `SetMessageGameScore` inferring your bot's token.
    fn set_message_game_score(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
        user_id: i64,
        score: u32,
    ) -> methods::SetMessageGameScore<'a> {
        self.prepare_method(methods::SetMessageGameScore::new(
            self.token(),
            chat_id,
            message_id,
            user_id,
            score,
        ))
    }

    /// Constructs a new `SetStickerPositionInSet` inferring your bot's token.
    fn set_sticker_position_in_set(
        &'a self,
        sticker: &'a str,
        position: u32,
    ) -> methods::SetStickerPositionInSet<'a> {
        self.prepare_method(methods::SetStickerPositionInSet::new(
            self.token(),
            sticker,
            position,
        ))
    }

    /// Constructs a new `StopInlineLocation` inferring your bot's token.
    fn stop_inline_location(
        &'a self,
        inline_message_id: &'a str,
    ) -> methods::StopInlineLocation<'a> {
        self.prepare_method(methods::StopInlineLocation::new(
            self.token(),
            inline_message_id,
        ))
    }

    /// Constructs a new `StopMessageLocation` inferring your bot's token.
    fn stop_message_location(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
    ) -> methods::StopMessageLocation<'a> {
        self.prepare_method(methods::StopMessageLocation::new(
            self.token(),
            chat_id,
            message_id,
        ))
    }

    /// Constructs a new `StopPoll` inferring your bot's token.
    fn stop_poll(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
    ) -> methods::StopPoll<'a> {
        self.prepare_method(methods::StopPoll::new(
            self.token(),
            chat_id,
            message_id,
        ))
    }

    /// Constructs a new `UnbanChatMember` inferring your bot's token.
    fn unban_chat_member(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        user_id: i64,
    ) -> methods::UnbanChatMember<'a> {
        self.prepare_method(methods::UnbanChatMember::new(
            self.token(),
            chat_id,
            user_id,
        ))
    }

    /// Constructs a new `UnpinChatMessage` inferring your bot's token.
    fn unpin_chat_message(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
    ) -> methods::UnpinChatMessage<'a> {
        self.prepare_method(methods::UnpinChatMessage::new(
            self.token(),
            chat_id,
        ))
    }

    /// Constructs a new `UploadStickerFile` inferring your bot's token.
    fn upload_sticker_file(
        &'a self,
        user_id: i64,
        png_sticker: &'a [u8],
    ) -> methods::UploadStickerFile<'a> {
        self.prepare_method(methods::UploadStickerFile::new(
            self.token(),
            user_id,
            png_sticker,
        ))
    }
}

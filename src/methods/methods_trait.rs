use super::*;
use types::input_file::*;

/// Provides API methods that infer the bot's token.
pub trait Methods<'a> {
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

    /// Constructs a new [`AddStickerToSet`] inferring `token`.
    ///
    /// [`AddStickerToSet`]: ./struct.AddStickerToSet.html
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

    /// Constructs a new [`CreateNewStickerSet`] inferring `token`.
    ///
    /// [`CreateNewStickerSet`]: ./struct.CreateNewStickerSet.html
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

    /// Constructs a new [`DeleteMessage`] inferring `token`.
    ///
    /// [`DeleteMessage`]: ./struct.DeleteMessage.html
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

    /// Constructs a new [`DeleteStickerFromSet`] inferring `token`.
    ///
    /// [`DeleteStickerFromSet`]: ./struct.DeleteStickerFromSet.html
    fn delete_sticker_from_set(
        &'a self,
        sticker: &'a str,
    ) -> methods::DeleteStickerFromSet<'a> {
        self.prepare_method(methods::DeleteStickerFromSet::new(
            self.token(),
            sticker,
        ))
    }

    /// Constructs a new [`EditInlineCaption`] inferring `token`.
    ///
    /// [`EditInlineCaption`]: ./struct.EditInlineCaption.html
    fn edit_inline_caption(
        &'a self,
        inline_message_id: u32,
        caption: &'a str,
    ) -> methods::EditInlineCaption<'a> {
        self.prepare_method(methods::EditInlineCaption::new(
            self.token(),
            inline_message_id,
            caption,
        ))
    }

    /// Constructs a new [`EditInlineLocation`] inferring `token`.
    ///
    /// [`EditInlineLocation`]: ./struct.EditInlineLocation.html
    fn edit_inline_location(
        &'a self,
        inline_message_id: u32,
        position: (f64, f64),
    ) -> methods::EditInlineLocation<'a> {
        self.prepare_method(methods::EditInlineLocation::new(
            self.token(),
            inline_message_id,
            position,
        ))
    }

    /// Constructs a new [`EditInlineMedia`] inferring `token`.
    ///
    /// [`EditInlineMedia`]: ./struct.EditInlineMedia.html
    fn edit_inline_media(
        &'a self,
        inline_message_id: u32,
        media: impl Into<EditableMedia<'a>>,
    ) -> methods::EditInlineMedia<'a> {
        self.prepare_method(methods::EditInlineMedia::new(
            self.token(),
            inline_message_id,
            media,
        ))
    }

    /// Constructs a new [`EditInlineReplyMarkup`] inferring `token`.
    ///
    /// [`EditInlineReplyMarkup`]: ./struct.EditInlineReplyMarkup.html
    fn edit_inline_reply_markup(
        &'a self,
        inline_message_id: u32,
        reply_markup: types::InlineKeyboard<'a>,
    ) -> methods::EditInlineReplyMarkup<'a> {
        self.prepare_method(methods::EditInlineReplyMarkup::new(
            self.token(),
            inline_message_id,
            reply_markup,
        ))
    }

    /// Constructs a new [`EditInlineText`] inferring `token`.
    ///
    /// [`EditInlineText`]: ./struct.EditInlineText.html
    fn edit_inline_text(
        &'a self,
        inline_message_id: u32,
        text: &'a str,
    ) -> methods::EditInlineText<'a> {
        self.prepare_method(methods::EditInlineText::new(
            self.token(),
            inline_message_id,
            text,
        ))
    }

    /// Constructs a new [`EditMessageCaption`] inferring `token`.
    ///
    /// [`EditMessageCaption`]: ./struct.EditMessageCaption.html
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

    /// Constructs a new [`EditMessageLocation`] inferring `token`.
    ///
    /// [`EditMessageLocation`]: ./struct.EditMessageLocation.html
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

    /// Constructs a new [`EditMessageMedia`] inferring `token`.
    ///
    /// [`EditMessageMedia`]: ./struct.EditMessageMedia.html
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

    /// Constructs a new [`EditMessageReplyMarkup`] inferring `token`.
    ///
    /// [`EditMessageReplyMarkup`]: ./struct.EditMessageReplyMarkup.html
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

    /// Constructs a new [`EditMessageText`] inferring `token`.
    ///
    /// [`EditMessageText`]: ./struct.EditMessageText.html
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

    /// Constructs a new [`ForwardMessage`] inferring `token`.
    ///
    /// [`ForwardMessage`]: ./struct.ForwardMessage.html
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

    /// Constructs a new [`GetMe`] inferring `token`.
    ///
    /// [`GetMe`]: ./struct.GetMe.html
    fn get_me(&'a self) -> methods::GetMe<'a> {
        self.prepare_method(methods::GetMe::new(self.token()))
    }

    /// Constructs a new [`GetStickerSet`] inferring `token`.
    ///
    /// [`GetStickerSet`]: ./struct.GetStickerSet.html
    fn get_sticker_set(&'a self, name: &'a str) -> methods::GetStickerSet<'a> {
        self.prepare_method(methods::GetStickerSet::new(self.token(), name))
    }

    /// Constructs a new [`GetUserProfilePhotos`] inferring `token`.
    ///
    /// [`GetUserProfilePhotos`]: ./struct.GetUserProfilePhotos.html
    fn get_user_profile_photos(
        &'a self,
        user_id: i64,
    ) -> methods::GetUserProfilePhotos<'a> {
        self.prepare_method(methods::GetUserProfilePhotos::new(
            self.token(),
            user_id,
        ))
    }

    /// Constructs a new [`GetWebhookInfo`] inferring `token`.
    ///
    /// [`GetWebhookInfo`]: ./struct.GetWebhookInfo.html
    fn get_webhook_info(&'a self) -> methods::GetWebhookInfo<'a> {
        self.prepare_method(methods::GetWebhookInfo::new(self.token()))
    }

    /// Constructs a new [`KickChatMember`] inferring `token`.
    ///
    /// [`KickChatMember`]: ./struct.KickChatMember.html
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

    /// Constructs a new [`PromoteChatMember`] inferring `token`.
    ///
    /// [`PromoteChatMember`]: ./struct.PromoteChatMember.html
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

    /// Constructs a new [`RestrictChatMember`] inferring `token`.
    ///
    /// [`RestrictChatMember`]: ./struct.RestrictChatMember.html
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

    /// Constructs a new [`SendAnimation`] inferring `token`.
    ///
    /// [`SendAnimation`]: ./struct.SendAnimation.html
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

    /// Constructs a new [`SendAudio`] inferring `token`.
    ///
    /// [`SendAudio`]: ./struct.SendAudio.html
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

    /// Constructs a new [`SendChatAction`] inferring `token`.
    ///
    /// [`SendChatAction`]: ./struct.SendChatAction.html
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

    /// Constructs a new [`SendContact`] inferring `token`.
    ///
    /// [`SendContact`]: ./struct.SendContact.html
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

    /// Constructs a new [`SendDocument`] inferring `token`.
    ///
    /// [`SendDocument`]: ./struct.SendDocument.html
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

    /// Constructs a new [`SendLocation`] inferring `token`.
    ///
    /// [`SendLocation`]: ./struct.SendLocation.html
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

    /// Constructs a new [`SendMediaGroup`] inferring `token`.
    ///
    /// [`SendMediaGroup`]: ./struct.SendMediaGroup.html
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

    /// Constructs a new [`SendMessage`] inferring `token`.
    ///
    /// [`SendMessage`]: ./struct.SendMessage.html
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

    /// Constructs a new [`SendPhoto`] inferring `token`.
    ///
    /// [`SendPhoto`]: ./struct.SendPhoto.html
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

    /// Constructs a new [`SendPoll`] inferring `token`.
    ///
    /// [`SendPoll`]: ./struct.SendPoll.html
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

    /// Constructs a new [`SendSticker`] inferring `token`.
    ///
    /// [`SendSticker`]: ./struct.SendSticker.html
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

    /// Constructs a new [`SendVenue`] inferring `token`.
    ///
    /// [`SendVenue`]: ./struct.SendVenue.html
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

    /// Constructs a new [`SendVideoNote`] inferring `token`.
    ///
    /// [`SendVideoNote`]: ./struct.SendVideoNote.html
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

    /// Constructs a new [`SendVideo`] inferring `token`.
    ///
    /// [`SendVideo`]: ./struct.SendVideo.html
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

    /// Constructs a new [`SendVoice`] inferring `token`.
    ///
    /// [`SendVoice`]: ./struct.SendVoice.html
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

    /// Constructs a new [`SetStickerPositionInSet`] inferring `token`.
    ///
    /// [`SetStickerPositionInSet`]: ./struct.SetStickerPositionInSet.html
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

    /// Constructs a new [`StopInlineLocation`] inferring `token`.
    ///
    /// [`StopInlineLocation`]: ./struct.StopInlineLocation.html
    fn stop_inline_location(
        &'a self,
        inline_message_id: u32,
    ) -> methods::StopInlineLocation<'a> {
        self.prepare_method(methods::StopInlineLocation::new(
            self.token(),
            inline_message_id,
        ))
    }

    /// Constructs a new [`StopMessageLocation`] inferring `token`.
    ///
    /// [`StopMessageLocation`]: ./struct.StopMessageLocation.html
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

    /// Constructs a new [`StopPoll`] inferring `token`.
    ///
    /// [`StopPoll`]: ./struct.StopPoll.html
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

    /// Constructs a new [`UnbanChatMember`] inferring `token`.
    ///
    /// [`UnbanChatMember`]: ./struct.UnbanChatMember.html
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

    /// Constructs a new [`UploadStickerFile`] inferring `token`.
    ///
    /// [`UploadStickerFile`]: ./struct.UploadStickerFile.html
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

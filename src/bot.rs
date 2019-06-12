use super::*;
use crate::{event_loop::EventLoop, methods::*, types::input_file::*};

/// Provides methods to call the Bots API.
///
/// A `Bot` provides all methods from the [methods] module, inferring the token:
///
/// ```no_run
/// use tbot::prelude::*;
///
/// let bot = tbot::bot!("BOT_TOKEN");
///
/// let me = bot
///     .get_me()
///     .into_future()
///     .map(|me| {
///         dbg!(me);
///     })
///     .map_err(|err| {
///         dbg!(err);
///     });
///
/// tbot::run(me);
/// ```
///
/// Besides, a `Bot` is used to construct an [`EventLoop`] — a struct
/// responsible for configuring handlers and listening to updates.
///
/// [`EventLoop`]: ./event_loop/struct.EventLoop.html
#[derive(Debug, Clone)]
pub struct Bot {
    pub(crate) token: Token,
    #[cfg(feature = "proxy")]
    pub(crate) proxy: Option<proxy::Proxy>,
}

impl Bot {
    /// Constructs a new `Bot`.
    pub const fn new(token: Token) -> Self {
        Self {
            token,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }

    /// Constructs a new `Bot`, extracting the token from the environment at
    /// _runtime_.
    ///
    /// If you need to extract the token at _compile time_, use [`bot!`].
    ///
    /// [`bot!`]: ./macro.bot.html
    ///
    /// # Example
    ///
    /// ```no_run
    /// use tbot::{Bot, prelude::*};
    ///
    /// let mut bot = Bot::from_env("BOT_TOKEN");
    ///
    /// let me = bot
    ///     .get_me()
    ///     .into_future()
    ///     .map(|me| {
    ///         dbg!(me);
    ///     })
    ///     .map_err(|err| {
    ///         dbg!(err);
    ///     });
    ///
    /// tbot::run(me);
    /// ```
    pub fn from_env(env_var: &'static str) -> Self {
        let token = std::env::var(env_var).unwrap_or_else(|_| {
            panic!("\n[tbot] Bot's token in {} was not specified\n", env_var)
        });

        Self::new(Token::new(token))
    }

    /// Sets a proxy through which requests to Telegram will be sent.
    #[cfg(feature = "proxy")]
    pub fn proxy(&mut self, proxy: proxy::Proxy) {
        self.proxy = Some(proxy);
    }

    /// Constructs an `EventLoop`.
    pub fn event_loop(self) -> EventLoop {
        EventLoop::new(self)
    }

    #[cfg(feature = "proxy")]
    fn prepare_method<T: ProxyMethod>(&self, method: T) -> T {
        if let Some(proxy) = &self.proxy {
            method.proxy(proxy.clone())
        } else {
            method
        }
    }

    #[cfg(not(feature = "proxy"))]
    const fn prepare_method<T>(&self, method: T) -> T {
        method
    }

    /// Constructs a new `AddStickerToSet` inferring your bot's token.
    pub fn add_sticker_to_set<'a>(
        &'a self,
        user_id: i64,
        name: &'a str,
        png_sticker: &'a PngSticker<'a>,
        emojis: &'a str,
    ) -> methods::AddStickerToSet<'a> {
        self.prepare_method(methods::AddStickerToSet::new(
            self.token.clone(),
            user_id,
            name,
            png_sticker,
            emojis,
        ))
    }

    /// Constructs a new `AnswerCallbackQuery` inferring your bot's token.
    pub fn answer_callback_query<'a>(
        &'a self,
        callback_query_id: &'a str,
        action: CallbackAnswerAction<'a>,
    ) -> methods::AnswerCallbackQuery<'a> {
        self.prepare_method(methods::AnswerCallbackQuery::new(
            self.token.clone(),
            callback_query_id,
            action,
        ))
    }

    /// Constructs a new `CreateNewStickerSet` inferring your bot's token.
    pub fn create_new_sticker_set<'a>(
        &'a self,
        user_id: i64,
        name: &'a str,
        title: &'a str,
        png_sticker: &'a PngSticker<'a>,
        emojis: &'a str,
    ) -> methods::CreateNewStickerSet<'a> {
        self.prepare_method(methods::CreateNewStickerSet::new(
            self.token.clone(),
            user_id,
            name,
            title,
            png_sticker,
            emojis,
        ))
    }

    /// Constructs a new `DeleteChatPhoto` inferring your bot's token.
    pub fn delete_chat_photo<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
    ) -> methods::DeleteChatPhoto<'a> {
        self.prepare_method(methods::DeleteChatPhoto::new(
            self.token.clone(),
            chat_id,
        ))
    }

    /// Constructs a new `DeleteChatStickerSet` inferring your bot's token.
    pub fn delete_chat_sticker_set<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
    ) -> methods::DeleteChatStickerSet<'a> {
        self.prepare_method(methods::DeleteChatStickerSet::new(
            self.token.clone(),
            chat_id,
        ))
    }

    /// Constructs a new `DeleteMessage` inferring your bot's token.
    pub fn delete_message<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
    ) -> methods::DeleteMessage<'a> {
        self.prepare_method(methods::DeleteMessage::new(
            self.token.clone(),
            chat_id,
            message_id,
        ))
    }

    /// Constructs a new `DeleteStickerFromSet` inferring your bot's token.
    pub fn delete_sticker_from_set<'a>(
        &'a self,
        sticker: &'a str,
    ) -> methods::DeleteStickerFromSet<'a> {
        self.prepare_method(methods::DeleteStickerFromSet::new(
            self.token.clone(),
            sticker,
        ))
    }

    /// Constructs a new `EditInlineCaption` inferring your bot's token.
    pub fn edit_inline_caption<'a>(
        &'a self,
        inline_message_id: &'a str,
        caption: &'a str,
    ) -> methods::EditInlineCaption<'a> {
        self.prepare_method(methods::EditInlineCaption::new(
            self.token.clone(),
            inline_message_id,
            caption,
        ))
    }

    /// Constructs a new `EditInlineLocation` inferring your bot's token.
    pub fn edit_inline_location<'a>(
        &'a self,
        inline_message_id: &'a str,
        position: (f64, f64),
    ) -> methods::EditInlineLocation<'a> {
        self.prepare_method(methods::EditInlineLocation::new(
            self.token.clone(),
            inline_message_id,
            position,
        ))
    }

    /// Constructs a new `EditInlineMedia` inferring your bot's token.
    pub fn edit_inline_media<'a>(
        &'a self,
        inline_message_id: &'a str,
        media: impl Into<EditableMedia<'a>>,
    ) -> methods::EditInlineMedia<'a> {
        self.prepare_method(methods::EditInlineMedia::new(
            self.token.clone(),
            inline_message_id,
            media,
        ))
    }

    /// Constructs a new `EditInlineReplyMarkup` inferring your bot's token.
    pub fn edit_inline_reply_markup<'a>(
        &'a self,
        inline_message_id: &'a str,
        reply_markup: types::InlineKeyboard<'a>,
    ) -> methods::EditInlineReplyMarkup<'a> {
        self.prepare_method(methods::EditInlineReplyMarkup::new(
            self.token.clone(),
            inline_message_id,
            reply_markup,
        ))
    }

    /// Constructs a new `EditInlineText` inferring your bot's token.
    pub fn edit_inline_text<'a>(
        &'a self,
        inline_message_id: &'a str,
        text: &'a str,
    ) -> methods::EditInlineText<'a> {
        self.prepare_method(methods::EditInlineText::new(
            self.token.clone(),
            inline_message_id,
            text,
        ))
    }

    /// Constructs a new `EditMessageCaption` inferring your bot's token.
    pub fn edit_message_caption<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
        caption: &'a str,
    ) -> methods::EditMessageCaption<'a> {
        self.prepare_method(methods::EditMessageCaption::new(
            self.token.clone(),
            chat_id,
            message_id,
            caption,
        ))
    }

    /// Constructs a new `EditMessageLocation` inferring your bot's token.
    pub fn edit_message_location<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
        position: (f64, f64),
    ) -> methods::EditMessageLocation<'a> {
        self.prepare_method(methods::EditMessageLocation::new(
            self.token.clone(),
            chat_id,
            message_id,
            position,
        ))
    }

    /// Constructs a new `EditMessageMedia` inferring your bot's token.
    pub fn edit_message_media<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
        media: impl Into<EditableMedia<'a>>,
    ) -> methods::EditMessageMedia<'a> {
        self.prepare_method(methods::EditMessageMedia::new(
            self.token.clone(),
            chat_id,
            message_id,
            media,
        ))
    }

    /// Constructs a new `EditMessageReplyMarkup` inferring your bot's token.
    pub fn edit_message_reply_markup<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
        reply_markup: types::InlineKeyboard<'a>,
    ) -> methods::EditMessageReplyMarkup<'a> {
        self.prepare_method(methods::EditMessageReplyMarkup::new(
            self.token.clone(),
            chat_id,
            message_id,
            reply_markup,
        ))
    }

    /// Constructs a new `EditMessageText` inferring your bot's token.
    pub fn edit_message_text<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
        text: &'a str,
    ) -> methods::EditMessageText<'a> {
        self.prepare_method(methods::EditMessageText::new(
            self.token.clone(),
            chat_id,
            message_id,
            text,
        ))
    }

    /// Constructs a new `ExportChatInviteLink` inferring your bot's token.
    pub fn export_chat_invite_link<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
    ) -> methods::ExportChatInviteLink<'a> {
        self.prepare_method(methods::ExportChatInviteLink::new(
            self.token.clone(),
            chat_id,
        ))
    }

    /// Constructs a new `ForwardMessage` inferring your bot's token.
    pub fn forward_message<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        from_chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
    ) -> methods::ForwardMessage<'a> {
        self.prepare_method(methods::ForwardMessage::new(
            self.token.clone(),
            chat_id,
            from_chat_id,
            message_id,
        ))
    }

    /// Constructs a new `GetChat` inferring your bot's token.
    pub fn get_chat<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
    ) -> methods::GetChat<'a> {
        self.prepare_method(methods::GetChat::new(self.token.clone(), chat_id))
    }

    /// Constructs a new `GetInlineGameHighScores` inferring your bot's token.
    pub fn get_inline_game_high_scores<'a>(
        &'a self,
        inline_message_id: &'a str,
        user_id: i64,
    ) -> methods::GetInlineGameHighScores<'a> {
        self.prepare_method(methods::GetInlineGameHighScores::new(
            self.token.clone(),
            inline_message_id,
            user_id,
        ))
    }

    /// Constructs a new `GetChatAdministrators` inferring your bot's token.
    pub fn get_chat_administrators<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
    ) -> methods::GetChatAdministrators<'a> {
        self.prepare_method(methods::GetChatAdministrators::new(
            self.token.clone(),
            chat_id,
        ))
    }

    /// Constructs a new `GetChatMember` inferring your bot's token.
    pub fn get_chat_member<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        user_id: i64,
    ) -> methods::GetChatMember<'a> {
        self.prepare_method(methods::GetChatMember::new(
            self.token.clone(),
            chat_id,
            user_id,
        ))
    }

    /// Constructs a new `GetChatMembersCount` inferring your bot's token.
    pub fn get_chat_members_count<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
    ) -> methods::GetChatMembersCount<'a> {
        self.prepare_method(methods::GetChatMembersCount::new(
            self.token.clone(),
            chat_id,
        ))
    }

    /// Constructs a new `GetMessageGameHighScores` inferring your bot's token.
    pub fn get_message_game_high_scores<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
        user_id: i64,
    ) -> methods::GetMessageGameHighScores<'a> {
        self.prepare_method(methods::GetMessageGameHighScores::new(
            self.token.clone(),
            chat_id,
            message_id,
            user_id,
        ))
    }

    /// Constructs a new `GetMe` inferring your bot's token.
    pub fn get_me(&self) -> methods::GetMe {
        self.prepare_method(methods::GetMe::new(self.token.clone()))
    }

    /// Constructs a new `GetStickerSet` inferring your bot's token.
    pub fn get_sticker_set<'a>(
        &'a self,
        name: &'a str,
    ) -> methods::GetStickerSet<'a> {
        self.prepare_method(methods::GetStickerSet::new(
            self.token.clone(),
            name,
        ))
    }

    /// Constructs a new `GetUserProfilePhotos` inferring your bot's token.
    pub fn get_user_profile_photos(
        &self,
        user_id: i64,
    ) -> methods::GetUserProfilePhotos {
        self.prepare_method(methods::GetUserProfilePhotos::new(
            self.token.clone(),
            user_id,
        ))
    }

    /// Constructs a new `GetWebhookInfo` inferring your bot's token.
    pub fn get_webhook_info(&self) -> methods::GetWebhookInfo {
        self.prepare_method(methods::GetWebhookInfo::new(self.token.clone()))
    }

    /// Constructs a new `KickChatMember` inferring your bot's token.
    pub fn kick_chat_member<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        user_id: i64,
    ) -> methods::KickChatMember<'a> {
        self.prepare_method(methods::KickChatMember::new(
            self.token.clone(),
            chat_id,
            user_id,
        ))
    }

    /// Constructs a new `LeaveChat` inferring your bot's token.
    pub fn leave_chat<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
    ) -> methods::LeaveChat<'a> {
        self.prepare_method(methods::LeaveChat::new(
            self.token.clone(),
            chat_id,
        ))
    }

    /// Constructs a new `PinChatMessage` inferring your bot's token.
    pub fn pin_chat_message<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
    ) -> methods::PinChatMessage<'a> {
        self.prepare_method(methods::PinChatMessage::new(
            self.token.clone(),
            chat_id,
            message_id,
        ))
    }

    /// Constructs a new `PromoteChatMember` inferring your bot's token.
    pub fn promote_chat_member<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        user_id: i64,
    ) -> methods::PromoteChatMember<'a> {
        self.prepare_method(methods::PromoteChatMember::new(
            self.token.clone(),
            chat_id,
            user_id,
        ))
    }

    /// Constructs a new `RestrictChatMember` inferring your bot's token.
    pub fn restrict_chat_member<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        user_id: i64,
    ) -> methods::RestrictChatMember<'a> {
        self.prepare_method(methods::RestrictChatMember::new(
            self.token.clone(),
            chat_id,
            user_id,
        ))
    }

    /// Constructs a new `SendAnimation` inferring your bot's token.
    pub fn send_animation<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        animation: &'a Animation<'a>,
    ) -> methods::SendAnimation<'a> {
        self.prepare_method(methods::SendAnimation::new(
            self.token.clone(),
            chat_id,
            animation,
        ))
    }

    /// Constructs a new `SendAudio` inferring your bot's token.
    pub fn send_audio<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        audio: &'a Audio<'a>,
    ) -> methods::SendAudio<'a> {
        self.prepare_method(methods::SendAudio::new(
            self.token.clone(),
            chat_id,
            audio,
        ))
    }

    /// Constructs a new `SendChatAction` inferring your bot's token.
    pub fn send_chat_action<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        action: types::ChatAction,
    ) -> methods::SendChatAction<'a> {
        self.prepare_method(methods::SendChatAction::new(
            self.token.clone(),
            chat_id,
            action,
        ))
    }

    /// Constructs a new `SendContact` inferring your bot's token.
    pub fn send_contact<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        phone_number: &'a str,
        first_name: &'a str,
    ) -> methods::SendContact<'a> {
        self.prepare_method(methods::SendContact::new(
            self.token.clone(),
            chat_id,
            phone_number,
            first_name,
        ))
    }

    /// Constructs a new `SendGame` inferring your bot's token.
    pub fn send_game<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        game_short_name: &'a str,
    ) -> methods::SendGame<'a> {
        self.prepare_method(methods::SendGame::new(
            self.token.clone(),
            chat_id,
            game_short_name,
        ))
    }

    /// Constructs a new `SendDocument` inferring your bot's token.
    pub fn send_document<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        document: &'a Document<'a>,
    ) -> methods::SendDocument<'a> {
        self.prepare_method(methods::SendDocument::new(
            self.token.clone(),
            chat_id,
            document,
        ))
    }

    /// Constructs a new `SendLocation` inferring your bot's token.
    pub fn send_location<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        position: (f64, f64),
    ) -> methods::SendLocation<'a> {
        self.prepare_method(methods::SendLocation::new(
            self.token.clone(),
            chat_id,
            position,
        ))
    }

    /// Constructs a new `SendMediaGroup` inferring your bot's token.
    pub fn send_media_group<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        media: Vec<GroupMedia<'a>>,
    ) -> methods::SendMediaGroup<'a> {
        self.prepare_method(methods::SendMediaGroup::new(
            self.token.clone(),
            chat_id,
            media,
        ))
    }

    /// Constructs a new `SendMessage` inferring your bot's token.
    pub fn send_message<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        text: &'a str,
    ) -> methods::SendMessage<'a> {
        self.prepare_method(methods::SendMessage::new(
            self.token.clone(),
            chat_id,
            text,
        ))
    }

    /// Constructs a new `SendPhoto` inferring your bot's token.
    pub fn send_photo<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        photo: &'a Photo<'a>,
    ) -> methods::SendPhoto<'a> {
        self.prepare_method(methods::SendPhoto::new(
            self.token.clone(),
            chat_id,
            photo,
        ))
    }

    /// Constructs a new `SendPoll` inferring your bot's token.
    pub fn send_poll<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        question: &'a str,
        options: &'a [&'a str],
    ) -> methods::SendPoll<'a> {
        self.prepare_method(methods::SendPoll::new(
            self.token.clone(),
            chat_id,
            question,
            options,
        ))
    }

    /// Constructs a new `SendSticker` inferring your bot's token.
    pub fn send_sticker<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        sticker: &'a Sticker<'a>,
    ) -> methods::SendSticker<'a> {
        self.prepare_method(methods::SendSticker::new(
            self.token.clone(),
            chat_id,
            sticker,
        ))
    }

    /// Constructs a new `SendVenue` inferring your bot's token.
    pub fn send_venue<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        position: (f64, f64),
        title: &'a str,
        address: &'a str,
    ) -> methods::SendVenue<'a> {
        self.prepare_method(methods::SendVenue::new(
            self.token.clone(),
            chat_id,
            position,
            title,
            address,
        ))
    }

    /// Constructs a new `SendVideoNote` inferring your bot's token.
    pub fn send_video_note<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        video_note: &'a VideoNote<'a>,
    ) -> methods::SendVideoNote<'a> {
        self.prepare_method(methods::SendVideoNote::new(
            self.token.clone(),
            chat_id,
            video_note,
        ))
    }

    /// Constructs a new `SendVideo` inferring your bot's token.
    pub fn send_video<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        video: &'a Video<'a>,
    ) -> methods::SendVideo<'a> {
        self.prepare_method(methods::SendVideo::new(
            self.token.clone(),
            chat_id,
            video,
        ))
    }

    /// Constructs a new `SendVoice` inferring your bot's token.
    pub fn send_voice<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        voice: &'a Voice<'a>,
    ) -> methods::SendVoice<'a> {
        self.prepare_method(methods::SendVoice::new(
            self.token.clone(),
            chat_id,
            voice,
        ))
    }

    /// Constructs a new `SetChatDescription` inferring your bot's token.
    pub fn set_chat_description<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        description: &'a str,
    ) -> methods::SetChatDescription<'a> {
        self.prepare_method(methods::SetChatDescription::new(
            self.token.clone(),
            chat_id,
            description,
        ))
    }

    /// Constructs a new `SetChatPhoto` inferring your bot's token.
    pub fn set_chat_photo<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        photo: &'a ChatPhoto<'a>,
    ) -> methods::SetChatPhoto<'a> {
        self.prepare_method(methods::SetChatPhoto::new(
            self.token.clone(),
            chat_id,
            photo,
        ))
    }

    /// Constructs a new `SetChatStickerSet` inferring your bot's token.
    pub fn set_chat_sticker_set<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        sticker_set_name: &'a str,
    ) -> methods::SetChatStickerSet<'a> {
        self.prepare_method(methods::SetChatStickerSet::new(
            self.token.clone(),
            chat_id,
            sticker_set_name,
        ))
    }

    /// Constructs a new `SetChatTitle` inferring your bot's token.
    pub fn set_chat_title<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        title: &'a str,
    ) -> methods::SetChatTitle<'a> {
        self.prepare_method(methods::SetChatTitle::new(
            self.token.clone(),
            chat_id,
            title,
        ))
    }

    /// Constructs a new `SetInlineGameScore` inferring your bot's token.
    pub fn set_inline_game_score<'a>(
        &'a self,
        inline_message_id: &'a str,
        user_id: i64,
        score: u32,
    ) -> methods::SetInlineGameScore<'a> {
        self.prepare_method(methods::SetInlineGameScore::new(
            self.token.clone(),
            inline_message_id,
            user_id,
            score,
        ))
    }

    /// Constructs a new `SetMessageGameScore` inferring your bot's token.
    pub fn set_message_game_score<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
        user_id: i64,
        score: u32,
    ) -> methods::SetMessageGameScore<'a> {
        self.prepare_method(methods::SetMessageGameScore::new(
            self.token.clone(),
            chat_id,
            message_id,
            user_id,
            score,
        ))
    }

    /// Constructs a new `SetStickerPositionInSet` inferring your bot's token.
    pub fn set_sticker_position_in_set<'a>(
        &'a self,
        sticker: &'a str,
        position: u32,
    ) -> methods::SetStickerPositionInSet<'a> {
        self.prepare_method(methods::SetStickerPositionInSet::new(
            self.token.clone(),
            sticker,
            position,
        ))
    }

    /// Constructs a new `StopInlineLocation` inferring your bot's token.
    pub fn stop_inline_location<'a>(
        &'a self,
        inline_message_id: &'a str,
    ) -> methods::StopInlineLocation<'a> {
        self.prepare_method(methods::StopInlineLocation::new(
            self.token.clone(),
            inline_message_id,
        ))
    }

    /// Constructs a new `StopMessageLocation` inferring your bot's token.
    pub fn stop_message_location<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
    ) -> methods::StopMessageLocation<'a> {
        self.prepare_method(methods::StopMessageLocation::new(
            self.token.clone(),
            chat_id,
            message_id,
        ))
    }

    /// Constructs a new `StopPoll` inferring your bot's token.
    pub fn stop_poll<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
    ) -> methods::StopPoll<'a> {
        self.prepare_method(methods::StopPoll::new(
            self.token.clone(),
            chat_id,
            message_id,
        ))
    }

    /// Constructs a new `UnbanChatMember` inferring your bot's token.
    pub fn unban_chat_member<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
        user_id: i64,
    ) -> methods::UnbanChatMember<'a> {
        self.prepare_method(methods::UnbanChatMember::new(
            self.token.clone(),
            chat_id,
            user_id,
        ))
    }

    /// Constructs a new `UnpinChatMessage` inferring your bot's token.
    pub fn unpin_chat_message<'a>(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
    ) -> methods::UnpinChatMessage<'a> {
        self.prepare_method(methods::UnpinChatMessage::new(
            self.token.clone(),
            chat_id,
        ))
    }

    /// Constructs a new `UploadStickerFile` inferring your bot's token.
    pub fn upload_sticker_file<'a>(
        &'a self,
        user_id: i64,
        png_sticker: &'a [u8],
    ) -> methods::UploadStickerFile<'a> {
        self.prepare_method(methods::UploadStickerFile::new(
            self.token.clone(),
            user_id,
            png_sticker,
        ))
    }
}

/// Constructs a new [`Bot`], extracting the token from the environment at
/// _compile time_.
///
/// If you need to extract the token at _runtime_, use [`Bot::from_env`].
///
/// [`Bot`]: ./struct.Bot.html
/// [`Bot::from_env`]: ./struct.Bot.html#method.from_env
///
/// # Example
///
/// ```
/// use tbot::prelude::*;
///
/// let mut bot = tbot::bot!("BOT_TOKEN");
///
/// let me = bot
///     .get_me()
///     .into_future()
///     .map(|me| {
///         dbg!(me);
///     })
///     .map_err(|err| {
///         dbg!(err);
///     });
///
/// tbot::run(me);
/// ```
#[macro_export]
macro_rules! bot {
    ($var:literal) => {{
        let token = env!($var).to_string();
        $crate::Bot::new($crate::Token::new(token))
    }};
    ($var:literal,) => {
        $crate::bot!($var)
    };
    () => {
        compile_error!("the macro must be invoked as `bot!(\"<VAR_NAME>\")`")
    };
    ($($x:tt)+) => {
        $crate::bot!()
    };
}

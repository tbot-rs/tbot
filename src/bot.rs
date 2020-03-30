use crate::{
    connectors::{self, Connector},
    download_file, errors,
    event_loop::EventLoop,
    methods::*,
    state::StatefulEventLoop,
    types::{
        callback, chat,
        file::{id::AsFileId, File},
        inline_message_id, inline_query,
        input_file::{
            Animation, Audio, ChatPhoto, Document, EditableMedia, GroupMedia,
            Photo, PngSticker, Sticker, Video, VideoNote, Voice,
        },
        keyboard::inline,
        message,
        parameters::{
            BotCommand, CallbackAction, ImplicitChatId, Poll, Text, UpdateKind,
        },
        passport, pre_checkout_query, shipping, user, LabeledPrice,
    },
    Token,
};
use std::sync::Arc;

/// Provides methods to call the Bots API.
///
/// A `Bot` lets you call methods from the [`methods`] module.
///
/// ```no_run
/// # async fn foo() {
/// let bot = tbot::from_env!("BOT_TOKEN");
/// let me = bot.get_me().call().await.unwrap();
/// dbg!(me);
/// # }
/// ```
///
/// Besides, a `Bot` is used to construct an [`EventLoop`] — a struct
/// responsible for configuring handlers and listening to updates.
///
/// [`EventLoop`]: ./event_loop/struct.EventLoop.html
/// [`methods`]: ./methods/index.html
#[derive(Debug, Clone)]
#[must_use]
pub struct Bot<C> {
    pub(crate) token: Token,
    pub(crate) client: Arc<crate::internal::Client<C>>,
}

impl Bot<connectors::Https> {
    /// Constructs a new `Bot`.
    pub fn new(token: String) -> Self {
        Self {
            token: Token::new(token),
            client: Arc::new(connectors::default()),
        }
    }

    /// Constructs a new `Bot`, extracting the token from the environment at
    /// _runtime_.
    ///
    /// If you need to extract the token at _compile time_, use [`from_env!`].
    ///
    /// [`from_env!`]: ./macro.bot.html
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn foo() {
    /// let bot = tbot::Bot::from_env("BOT_TOKEN");
    /// let me = bot.get_me().call().await.unwrap();
    /// dbg!(me);
    /// # }
    /// ```
    pub fn from_env(env_var: &'static str) -> Self {
        Self::new(extract_token(env_var))
    }
}

impl<C> Bot<C> {
    /// Constructs an `EventLoop`.
    pub fn event_loop(self) -> EventLoop<C> {
        EventLoop::new(self)
    }

    /// Constructs a stateful event loop.
    pub fn stateful_event_loop<S>(self, state: S) -> StatefulEventLoop<C, S>
    where
        S: Send + Sync + 'static,
    {
        StatefulEventLoop::new(EventLoop::new(self), state)
    }

    /// Adds a new sticker to an existing sticker set.
    pub fn add_sticker_to_set<'a>(
        &'a self,
        user_id: user::Id,
        name: &'a str,
        png_sticker: PngSticker<'a>,
        emojis: &'a str,
    ) -> AddStickerToSet<'a, C> {
        AddStickerToSet::new(
            &self.client,
            self.token.as_ref(),
            user_id,
            name,
            png_sticker,
            emojis,
        )
    }

    pub(crate) fn answer_callback_query<'a>(
        &'a self,
        callback_query_id: callback::query::id::Ref<'a>,
        action: CallbackAction<'a>,
    ) -> AnswerCallbackQuery<'a, C> {
        AnswerCallbackQuery::new(
            &self.client,
            self.token.as_ref(),
            callback_query_id,
            action,
        )
    }

    pub(crate) fn answer_inline_query<'a>(
        &'a self,
        inline_query_id: inline_query::id::Ref<'a>,
        results: &'a [inline_query::Result<'a>],
    ) -> AnswerInlineQuery<'a, C> {
        AnswerInlineQuery::new(
            &self.client,
            self.token.as_ref(),
            inline_query_id,
            results,
        )
    }

    pub(crate) fn answer_pre_checkout_query<'a>(
        &'a self,
        pre_checkout_query_id: pre_checkout_query::id::Ref<'a>,
        result: Result<(), &'a str>,
    ) -> AnswerPreCheckoutQuery<'a, C> {
        AnswerPreCheckoutQuery::new(
            &self.client,
            self.token.as_ref(),
            pre_checkout_query_id,
            result,
        )
    }

    pub(crate) fn answer_shipping_query<'a>(
        &'a self,
        shipping_query_id: shipping::query::id::Ref<'a>,
        result: Result<&'a [shipping::Option<'a>], &'a str>,
    ) -> AnswerShippingQuery<'a, C> {
        AnswerShippingQuery::new(
            &self.client,
            self.token.as_ref(),
            shipping_query_id,
            result,
        )
    }

    /// Creates a new sticker set.
    pub fn create_new_sticker_set<'a>(
        &'a self,
        user_id: user::Id,
        name: &'a str,
        title: &'a str,
        png_sticker: PngSticker<'a>,
        emojis: &'a str,
    ) -> CreateNewStickerSet<'a, C> {
        CreateNewStickerSet::new(
            &self.client,
            self.token.as_ref(),
            user_id,
            name,
            title,
            png_sticker,
            emojis,
        )
    }

    /// Deletes a chat's photo.
    pub fn delete_chat_photo<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
    ) -> DeleteChatPhoto<'a, C> {
        DeleteChatPhoto::new(&self.client, self.token.as_ref(), chat_id)
    }

    /// Deletes a chat's sticker set.
    pub fn delete_chat_sticker_set<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
    ) -> DeleteChatStickerSet<'a, C> {
        DeleteChatStickerSet::new(&self.client, self.token.as_ref(), chat_id)
    }

    /// Deletes a message from a chat.
    pub fn delete_message<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
    ) -> DeleteMessage<'a, C> {
        DeleteMessage::new(
            &self.client,
            self.token.as_ref(),
            chat_id,
            message_id,
        )
    }

    /// Deletes a sticker from a sticker set.
    pub fn delete_sticker_from_set<'a>(
        &'a self,
        sticker: &'a str,
    ) -> DeleteStickerFromSet<'a, C> {
        DeleteStickerFromSet::new(&self.client, self.token.as_ref(), sticker)
    }

    pub(crate) fn delete_webhook(&self) -> DeleteWebhook<'_, C> {
        DeleteWebhook::new(&self.client, self.token.as_ref())
    }

    /// Edits the caption of a media message sent via the inline mode.
    pub fn edit_inline_caption<'a>(
        &'a self,
        inline_message_id: inline_message_id::Ref<'a>,
        caption: impl Into<Text<'a>>,
    ) -> EditInlineCaption<'a, C> {
        EditInlineCaption::new(
            &self.client,
            self.token.as_ref(),
            inline_message_id,
            caption,
        )
    }

    /// Edits a live location sent via the inline mode.
    pub fn edit_inline_location<'a>(
        &'a self,
        inline_message_id: inline_message_id::Ref<'a>,
        position: (f64, f64),
    ) -> EditInlineLocation<'a, C> {
        EditInlineLocation::new(
            &self.client,
            self.token.as_ref(),
            inline_message_id,
            position,
        )
    }

    /// Edits the media of a message sent via the inline mode.
    pub fn edit_inline_media<'a>(
        &'a self,
        inline_message_id: inline_message_id::Ref<'a>,
        media: impl Into<EditableMedia<'a>>,
    ) -> EditInlineMedia<'a, C> {
        EditInlineMedia::new(
            &self.client,
            self.token.as_ref(),
            inline_message_id,
            media,
        )
    }

    /// Edits the inline keyboard of a message sent via the inline mode.
    pub fn edit_inline_reply_markup<'a>(
        &'a self,
        inline_message_id: inline_message_id::Ref<'a>,
        reply_markup: inline::Keyboard<'a>,
    ) -> EditInlineReplyMarkup<'a, C> {
        EditInlineReplyMarkup::new(
            &self.client,
            self.token.as_ref(),
            inline_message_id,
            reply_markup,
        )
    }

    /// Edits the text of a message sent via the inline mode.
    pub fn edit_inline_text<'a>(
        &'a self,
        inline_message_id: inline_message_id::Ref<'a>,
        text: impl Into<Text<'a>>,
    ) -> EditInlineText<'a, C> {
        EditInlineText::new(
            &self.client,
            self.token.as_ref(),
            inline_message_id,
            text,
        )
    }

    /// Edits the caption of a media message sent by the bot itself.
    pub fn edit_message_caption<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
        caption: impl Into<Text<'a>>,
    ) -> EditMessageCaption<'a, C> {
        EditMessageCaption::new(
            &self.client,
            self.token.as_ref(),
            chat_id,
            message_id,
            caption,
        )
    }

    /// Edits a live location sent by the bot itself.
    pub fn edit_message_location<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
        position: (f64, f64),
    ) -> EditMessageLocation<'a, C> {
        EditMessageLocation::new(
            &self.client,
            self.token.as_ref(),
            chat_id,
            message_id,
            position,
        )
    }

    /// Edits a live location sent by the bot itself.
    pub fn edit_message_media<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
        media: impl Into<EditableMedia<'a>>,
    ) -> EditMessageMedia<'a, C> {
        EditMessageMedia::new(
            &self.client,
            self.token.as_ref(),
            chat_id,
            message_id,
            media,
        )
    }

    /// Edits the inline keyboard of a message sent by the bot itself.
    pub fn edit_message_reply_markup<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
        reply_markup: inline::Keyboard<'a>,
    ) -> EditMessageReplyMarkup<'a, C> {
        EditMessageReplyMarkup::new(
            &self.client,
            self.token.as_ref(),
            chat_id,
            message_id,
            reply_markup,
        )
    }

    /// Edits the text of a message sent by the bot itself.
    pub fn edit_message_text<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
        text: impl Into<Text<'a>>,
    ) -> EditMessageText<'a, C> {
        EditMessageText::new(
            &self.client,
            self.token.as_ref(),
            chat_id,
            message_id,
            text,
        )
    }

    /// Exports a chat's invite link.
    pub fn export_chat_invite_link<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
    ) -> ExportChatInviteLink<'a, C> {
        ExportChatInviteLink::new(&self.client, self.token.as_ref(), chat_id)
    }

    /// Forwards a message.
    pub fn forward_message<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        from_chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
    ) -> ForwardMessage<'a, C> {
        ForwardMessage::new(
            &self.client,
            self.token.as_ref(),
            chat_id,
            from_chat_id,
            message_id,
        )
    }

    /// Gets information about a chat.
    pub fn get_chat<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
    ) -> GetChat<'a, C> {
        GetChat::new(&self.client, self.token.as_ref(), chat_id)
    }

    /// Gets information about a file.
    pub fn get_file<'a>(
        &'a self,
        file_id: &'a impl AsFileId,
    ) -> GetFile<'a, C> {
        GetFile::new(&self.client, self.token.as_ref(), file_id)
    }

    /// Gets an excerpt from the high score table of a game sent via the inline
    /// mode.
    pub fn get_inline_game_high_scores<'a>(
        &'a self,
        inline_message_id: inline_message_id::Ref<'a>,
        user_id: user::Id,
    ) -> GetInlineGameHighScores<'a, C> {
        GetInlineGameHighScores::new(
            &self.client,
            self.token.as_ref(),
            inline_message_id,
            user_id,
        )
    }

    /// Gets information about a chat's admins.
    pub fn get_chat_administrators<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
    ) -> GetChatAdministrators<'a, C> {
        GetChatAdministrators::new(&self.client, self.token.as_ref(), chat_id)
    }

    /// Gets information about a chat's member.
    pub fn get_chat_member<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        user_id: user::Id,
    ) -> GetChatMember<'a, C> {
        GetChatMember::new(&self.client, self.token.as_ref(), chat_id, user_id)
    }

    /// Gets a chat's member count.
    pub fn get_chat_members_count<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
    ) -> GetChatMembersCount<'a, C> {
        GetChatMembersCount::new(&self.client, self.token.as_ref(), chat_id)
    }

    /// Gets an excerpt from the high score table of a game sent by the bot
    /// itself.
    pub fn get_message_game_high_scores<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
        user_id: user::Id,
    ) -> GetMessageGameHighScores<'a, C> {
        GetMessageGameHighScores::new(
            &self.client,
            self.token.as_ref(),
            chat_id,
            message_id,
            user_id,
        )
    }

    /// Gets information about the bot.
    pub fn get_me(&self) -> GetMe<'_, C> {
        GetMe::new(&self.client, self.token.as_ref())
    }

    /// Gets the list of the bot's commands.
    pub fn get_my_commands(&self) -> GetMyCommands<'_, C> {
        GetMyCommands::new(&self.client, self.token.as_ref())
    }

    /// Gets a sticker set by its name.
    pub fn get_sticker_set<'a>(
        &'a self,
        name: &'a str,
    ) -> GetStickerSet<'a, C> {
        GetStickerSet::new(&self.client, self.token.as_ref(), name)
    }

    pub(crate) fn get_updates<'a>(
        &'a self,
        offset: Option<isize>,
        limit: Option<u8>,
        timeout: Option<u64>,
        allowed_updates: Option<&'a [UpdateKind]>,
    ) -> GetUpdates<'a, C> {
        GetUpdates::new(
            &self.client,
            self.token.as_ref(),
            offset,
            limit,
            timeout,
            allowed_updates,
        )
    }

    /// Gets a user's profile photos.
    pub fn get_user_profile_photos(
        &self,
        user_id: user::Id,
    ) -> GetUserProfilePhotos<'_, C> {
        GetUserProfilePhotos::new(&self.client, self.token.as_ref(), user_id)
    }

    /// Gets information about the bot's webhook.
    pub fn get_webhook_info(&self) -> GetWebhookInfo<'_, C> {
        GetWebhookInfo::new(&self.client, self.token.as_ref())
    }

    /// Kicks a member out of a chat.
    pub fn kick_chat_member<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        user_id: user::Id,
    ) -> KickChatMember<'a, C> {
        KickChatMember::new(&self.client, self.token.as_ref(), chat_id, user_id)
    }

    /// Leaves a chat.
    pub fn leave_chat<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
    ) -> LeaveChat<'a, C> {
        LeaveChat::new(&self.client, self.token.as_ref(), chat_id)
    }

    /// Pins a message in a chat.
    pub fn pin_chat_message<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
    ) -> PinChatMessage<'a, C> {
        PinChatMessage::new(
            &self.client,
            self.token.as_ref(),
            chat_id,
            message_id,
        )
    }

    /// Promotes a chat member to an admin.
    pub fn promote_chat_member<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        user_id: user::Id,
    ) -> PromoteChatMember<'a, C> {
        PromoteChatMember::new(
            &self.client,
            self.token.as_ref(),
            chat_id,
            user_id,
        )
    }

    /// Restricts a chat member.
    pub fn restrict_chat_member<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        user_id: user::Id,
        permissions: chat::Permissions,
    ) -> RestrictChatMember<'a, C> {
        RestrictChatMember::new(
            &self.client,
            self.token.as_ref(),
            chat_id,
            user_id,
            permissions,
        )
    }

    /// Sends an animation.
    pub fn send_animation<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        animation: Animation<'a>,
    ) -> SendAnimation<'a, C> {
        SendAnimation::new(
            &self.client,
            self.token.as_ref(),
            chat_id,
            animation,
        )
    }

    /// Sends an audio.
    pub fn send_audio<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        audio: Audio<'a>,
    ) -> SendAudio<'a, C> {
        SendAudio::new(&self.client, self.token.as_ref(), chat_id, audio)
    }

    /// Sends a chat action.
    pub fn send_chat_action<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        action: chat::Action,
    ) -> SendChatAction<'a, C> {
        SendChatAction::new(&self.client, self.token.as_ref(), chat_id, action)
    }

    /// Sends a contact.
    pub fn send_contact<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        phone_number: &'a str,
        first_name: &'a str,
    ) -> SendContact<'a, C> {
        SendContact::new(
            &self.client,
            self.token.as_ref(),
            chat_id,
            phone_number,
            first_name,
        )
    }

    /// Sends a game.
    pub fn send_game<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        game_short_name: &'a str,
    ) -> SendGame<'a, C> {
        SendGame::new(
            &self.client,
            self.token.as_ref(),
            chat_id,
            game_short_name,
        )
    }

    /// Sends a dice.
    pub fn send_dice<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
    ) -> SendDice<'a, C> {
        SendDice::new(&self.client, self.token.as_ref(), chat_id)
    }

    /// Sends a document.
    pub fn send_document<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        document: Document<'a>,
    ) -> SendDocument<'a, C> {
        SendDocument::new(&self.client, self.token.as_ref(), chat_id, document)
    }

    /// Sends an invoice.
    #[allow(clippy::too_many_arguments)]
    pub fn send_invoice<'a>(
        &'a self,
        chat_id: impl Into<chat::Id>,
        title: &'a str,
        description: &'a str,
        payload: &'a str,
        provider_token: &'a str,
        start_parameter: &'a str,
        currency: &'a str,
        prices: &'a [LabeledPrice<'a>],
    ) -> SendInvoice<'a, C> {
        SendInvoice::new(
            &self.client,
            self.token.as_ref(),
            chat_id,
            title,
            description,
            payload,
            provider_token,
            start_parameter,
            currency,
            prices,
        )
    }

    /// Sends a location.
    pub fn send_location<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        position: (f64, f64),
    ) -> SendLocation<'a, C> {
        SendLocation::new(&self.client, self.token.as_ref(), chat_id, position)
    }

    /// Sends an album.
    pub fn send_media_group<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        media: &'a [GroupMedia<'a>],
    ) -> SendMediaGroup<'a, C> {
        SendMediaGroup::new(&self.client, self.token.as_ref(), chat_id, media)
    }

    /// Sends a text message.
    pub fn send_message<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        text: impl Into<Text<'a>>,
    ) -> SendMessage<'a, C> {
        SendMessage::new(&self.client, self.token.as_ref(), chat_id, text)
    }

    /// Sends a photo.
    pub fn send_photo<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        photo: Photo<'a>,
    ) -> SendPhoto<'a, C> {
        SendPhoto::new(&self.client, self.token.as_ref(), chat_id, photo)
    }

    /// Sends a poll.
    pub fn send_poll<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        poll: &'a Poll<'a>,
    ) -> SendPoll<'a, C> {
        SendPoll::new(&self.client, self.token.as_ref(), chat_id, poll)
    }

    /// Sends a sticker.
    pub fn send_sticker<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        sticker: Sticker<'a>,
    ) -> SendSticker<'a, C> {
        SendSticker::new(&self.client, self.token.as_ref(), chat_id, sticker)
    }

    /// Sends a venue.
    pub fn send_venue<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        position: (f64, f64),
        title: &'a str,
        address: &'a str,
    ) -> SendVenue<'a, C> {
        SendVenue::new(
            &self.client,
            self.token.as_ref(),
            chat_id,
            position,
            title,
            address,
        )
    }

    /// Sends a video note.
    pub fn send_video_note<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        video_note: VideoNote<'a>,
    ) -> SendVideoNote<'a, C> {
        SendVideoNote::new(
            &self.client,
            self.token.as_ref(),
            chat_id,
            video_note,
        )
    }

    /// Sends a video.
    pub fn send_video<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        video: Video<'a>,
    ) -> SendVideo<'a, C> {
        SendVideo::new(&self.client, self.token.as_ref(), chat_id, video)
    }

    /// Sends a voice.
    pub fn send_voice<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        voice: Voice<'a>,
    ) -> SendVoice<'a, C> {
        SendVoice::new(&self.client, self.token.as_ref(), chat_id, voice)
    }

    /// Sets a custom title for an admin in a chat.
    pub fn set_chat_administrator_custom_title<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        user_id: user::Id,
        custom_title: &'a str,
    ) -> SetChatAdministratorCustomTitle<'a, C> {
        SetChatAdministratorCustomTitle::new(
            &self.client,
            self.token.as_ref(),
            chat_id,
            user_id,
            custom_title,
        )
    }

    /// Sets a chat's description.
    pub fn set_chat_description<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        description: &'a str,
    ) -> SetChatDescription<'a, C> {
        SetChatDescription::new(
            &self.client,
            self.token.as_ref(),
            chat_id,
            description,
        )
    }

    /// Sets a group's global permissions.
    pub fn set_chat_permissions<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        permissions: chat::Permissions,
    ) -> SetChatPermissions<'a, C> {
        SetChatPermissions::new(
            &self.client,
            self.token.as_ref(),
            chat_id,
            permissions,
        )
    }

    /// Sets a chat's photo.
    pub fn set_chat_photo<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        photo: ChatPhoto<'a>,
    ) -> SetChatPhoto<'a, C> {
        SetChatPhoto::new(&self.client, self.token.as_ref(), chat_id, photo)
    }

    /// Sets a group's sticker set.
    pub fn set_chat_sticker_set<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        sticker_set_name: &'a str,
    ) -> SetChatStickerSet<'a, C> {
        SetChatStickerSet::new(
            &self.client,
            self.token.as_ref(),
            chat_id,
            sticker_set_name,
        )
    }

    /// Sets a group's title.
    pub fn set_chat_title<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        title: &'a str,
    ) -> SetChatTitle<'a, C> {
        SetChatTitle::new(&self.client, self.token.as_ref(), chat_id, title)
    }

    /// Sets a user's new high score in a game sent via the inline mode.
    pub fn set_inline_game_score<'a>(
        &'a self,
        inline_message_id: inline_message_id::Ref<'a>,
        user_id: user::Id,
        score: u32,
    ) -> SetInlineGameScore<'a, C> {
        SetInlineGameScore::new(
            &self.client,
            self.token.as_ref(),
            inline_message_id,
            user_id,
            score,
        )
    }

    /// Sets a user's new high score in a game sent by the bot itself.
    pub fn set_message_game_score<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
        user_id: user::Id,
        score: u32,
    ) -> SetMessageGameScore<'a, C> {
        SetMessageGameScore::new(
            &self.client,
            self.token.as_ref(),
            chat_id,
            message_id,
            user_id,
            score,
        )
    }

    /// Sets the list of the bot's commands.
    pub fn set_my_commands<'a>(
        &'a self,
        commands: &'a [BotCommand<'a>],
    ) -> SetMyCommands<'a, C> {
        SetMyCommands::new(&self.client, self.token.as_ref(), commands)
    }

    /// Reports passport errors to the user.
    pub fn set_passport_data_errors<'a>(
        &'a self,
        user_id: user::Id,
        errors: &'a [passport::element::Error<'a>],
    ) -> SetPassportDataErrors<'a, C> {
        SetPassportDataErrors::new(
            &self.client,
            self.token.as_ref(),
            user_id,
            errors,
        )
    }

    /// Changes a sticker's position in a sticker set.
    pub fn set_sticker_position_in_set<'a>(
        &'a self,
        sticker: &'a str,
        position: u32,
    ) -> SetStickerPositionInSet<'a, C> {
        SetStickerPositionInSet::new(
            &self.client,
            self.token.as_ref(),
            sticker,
            position,
        )
    }

    pub(crate) fn set_webhook<'a>(
        &'a self,
        url: &'a str,
        certificate: Option<&'a str>,
        max_connections: Option<u8>,
        allowed_updates: Option<&'a [UpdateKind]>,
    ) -> SetWebhook<'a, C> {
        SetWebhook::new(
            &self.client,
            self.token.as_ref(),
            url,
            certificate,
            max_connections,
            allowed_updates,
        )
    }

    /// Stops a live location sent via the inline mode.
    pub fn stop_inline_location<'a>(
        &'a self,
        inline_message_id: inline_message_id::Ref<'a>,
    ) -> StopInlineLocation<'a, C> {
        StopInlineLocation::new(
            &self.client,
            self.token.as_ref(),
            inline_message_id,
        )
    }

    /// Stops a live location sent by the bot itself.
    pub fn stop_message_location<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
    ) -> StopMessageLocation<'a, C> {
        StopMessageLocation::new(
            &self.client,
            self.token.as_ref(),
            chat_id,
            message_id,
        )
    }

    /// Stops a poll.
    pub fn stop_poll<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
    ) -> StopPoll<'a, C> {
        StopPoll::new(&self.client, self.token.as_ref(), chat_id, message_id)
    }

    /// Lifts all restrictions from a group's member.
    pub fn unban_chat_member<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        user_id: user::Id,
    ) -> UnbanChatMember<'a, C> {
        UnbanChatMember::new(
            &self.client,
            self.token.as_ref(),
            chat_id,
            user_id,
        )
    }

    /// Unpins a chat message.
    pub fn unpin_chat_message<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
    ) -> UnpinChatMessage<'a, C> {
        UnpinChatMessage::new(&self.client, self.token.as_ref(), chat_id)
    }

    /// Uploads a sticker file.
    pub fn upload_sticker_file<'a>(
        &'a self,
        user_id: user::Id,
        png_sticker: &'a [u8],
    ) -> UploadStickerFile<'a, C> {
        UploadStickerFile::new(
            &self.client,
            self.token.as_ref(),
            user_id,
            png_sticker,
        )
    }
}

impl<C: Connector> Bot<C> {
    /// Constructs a `Bot` with a custom connector.
    pub fn with_connector(token: String, connector: C) -> Self {
        Self {
            token: Token::new(token),
            client: Arc::new(connectors::create_client(connector)),
        }
    }

    /// Like [`Bot::from_env`] with a custom connector.
    ///
    /// [`Bot::from_env`]: #method.from_env
    pub fn from_env_with_connector(
        env_var: &'static str,
        connector: C,
    ) -> Self {
        Self::with_connector(extract_token(env_var), connector)
    }

    /// Downloads a file.
    pub async fn download_file(
        &self,
        file: &File,
    ) -> Result<Vec<u8>, errors::Download> {
        download_file(&self.client, self.token.as_ref(), file).await
    }
}

/// Constructs a new [`Bot`], extracting the token from the environment at
/// _compile time_.
///
/// You can provide a connector as the second parameter,
/// e.g. `from_env!("...", connector)`.
///
/// If you need to extract the token at _runtime_, use [`Bot::from_env`].
///
/// [`Bot`]: ./struct.Bot.html
/// [`Bot::from_env`]: ./struct.Bot.html#method.from_env
///
/// # Example
///
/// ```no_run
/// # async fn foo() {
/// let mut bot = tbot::from_env!("BOT_TOKEN");
/// let me = bot.get_me().call().await.unwrap();
/// dbg!(me);
/// # }
/// ```
#[macro_export]
macro_rules! from_env {
    ($var:literal) => {{
        let token = env!($var).to_string();
        $crate::Bot::new(token)
    }};
    ($var:literal,) => {
        $crate::from_env!($var)
    };
    ($var:literal, $connector:expr) => {{
        let token = env!($var).to_string();
        $crate::Bot::with_connector(token, $connector)
    }};
    ($var:literal, $connector:expr,) => {
        $crate::from_env!($var, $connector)
    };
    () => {
        compile_error!(
            "the macro must be invoked as `from_env!(\"<VAR_NAME>\")` or \
             `from_env!(\"<VAR_NAME>\", connector)`"
        )
    };
    ($($x:tt)+) => {
        $crate::from_env!()
    };
}

fn extract_token(env_var: &'static str) -> String {
    std::env::var(env_var).unwrap_or_else(|_| {
        panic!("\n[tbot] Bot's token in {} was not specified\n", env_var)
    })
}

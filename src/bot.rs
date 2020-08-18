// Type out about 80 method names? No, thanks
#![allow(clippy::wildcard_imports)]
use crate::{
    connectors::Client,
    download_file, errors,
    event_loop::EventLoop,
    internal::Sealed,
    methods::*,
    proxy::Proxy,
    state::StatefulEventLoop,
    types::{
        callback, chat,
        file::{id::AsFileId, File},
        inline_message_id, inline_query,
        input_file::{
            Animation, Audio, ChatPhoto, Document, EditableMedia, GroupMedia,
            Photo, Sticker, StickerForStickerSet, StickerSetThumb, Video,
            VideoNote, Voice,
        },
        keyboard::inline,
        message,
        parameters::{
            Any, BotCommand, CallbackAction, ImplicitChatId, Text, UpdateKind,
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
pub struct Bot {
    pub(crate) inner: Arc<InnerBot>,
}

#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct InnerBot {
    token: Token,
    client: Client,
}

impl InnerBot {
    pub fn token(&self) -> &str {
        &self.token.0
    }

    pub const fn client(&self) -> &Client {
        &self.client
    }
}

impl Bot {
    /// Constructs a new `Bot`.
    pub fn new(token: String) -> Self {
        Self {
            inner: Arc::new(InnerBot {
                token: Token(token),
                client: Client::https(),
            }),
        }
    }

    /// Constructs a `Bot` with the provided proxy.
    pub fn with_proxy(token: String, proxy: impl Into<Proxy>) -> Self {
        let proxy: Proxy = proxy.into();

        Self {
            inner: Arc::new(InnerBot {
                token: Token(token),
                client: proxy.into(),
            }),
        }
    }

    /// Like [`Bot::from_env`], but with a provided proxy.
    ///
    /// [`Bot::from_env`]: #method.from_env
    pub fn from_env_with_proxy(
        env_var: &'static str,
        proxy: impl Into<Proxy>,
    ) -> Self {
        Self::with_proxy(extract_token(env_var), proxy)
    }

    /// Downloads a file.
    pub async fn download_file(
        &self,
        file: &File,
    ) -> Result<Vec<u8>, errors::Download> {
        download_file(&self.inner, file).await
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

    /// Constructs an `EventLoop`.
    pub fn event_loop(self) -> EventLoop {
        EventLoop::new(self)
    }

    /// Constructs a stateful event loop.
    pub fn stateful_event_loop<S>(self, state: S) -> StatefulEventLoop<S>
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
        png_sticker: impl Into<StickerForStickerSet<'a>>,
        emojis: &'a str,
    ) -> AddStickerToSet<'a> {
        AddStickerToSet::new(&self.inner, user_id, name, png_sticker, emojis)
    }

    pub(crate) fn answer_callback_query<'a>(
        &'a self,
        callback_query_id: callback::query::id::Ref<'a>,
        action: CallbackAction<'a>,
    ) -> AnswerCallbackQuery<'a> {
        AnswerCallbackQuery::new(&self.inner, callback_query_id, action)
    }

    pub(crate) fn answer_inline_query<'a>(
        &'a self,
        inline_query_id: inline_query::id::Ref<'a>,
        results: &'a [inline_query::Result<'a>],
    ) -> AnswerInlineQuery<'a> {
        AnswerInlineQuery::new(&self.inner, inline_query_id, results)
    }

    pub(crate) fn answer_pre_checkout_query<'a>(
        &'a self,
        pre_checkout_query_id: pre_checkout_query::id::Ref<'a>,
        result: Result<(), &'a str>,
    ) -> AnswerPreCheckoutQuery<'a> {
        AnswerPreCheckoutQuery::new(&self.inner, pre_checkout_query_id, result)
    }

    pub(crate) fn answer_shipping_query<'a>(
        &'a self,
        shipping_query_id: shipping::query::id::Ref<'a>,
        result: Result<&'a [shipping::Option<'a>], &'a str>,
    ) -> AnswerShippingQuery<'a> {
        AnswerShippingQuery::new(&self.inner, shipping_query_id, result)
    }

    /// Creates a new sticker set.
    pub fn create_new_sticker_set<'a>(
        &'a self,
        user_id: user::Id,
        name: &'a str,
        title: &'a str,
        png_sticker: impl Into<StickerForStickerSet<'a>>,
        emojis: &'a str,
    ) -> CreateNewStickerSet<'a> {
        CreateNewStickerSet::new(
            &self.inner,
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
    ) -> DeleteChatPhoto<'a> {
        DeleteChatPhoto::new(&self.inner, chat_id)
    }

    /// Deletes a chat's sticker set.
    pub fn delete_chat_sticker_set<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
    ) -> DeleteChatStickerSet<'a> {
        DeleteChatStickerSet::new(&self.inner, chat_id)
    }

    /// Deletes a message from a chat.
    pub fn delete_message<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
    ) -> DeleteMessage<'a> {
        DeleteMessage::new(&self.inner, chat_id, message_id)
    }

    /// Deletes a sticker from a sticker set.
    pub fn delete_sticker_from_set<'a>(
        &'a self,
        sticker: &'a str,
    ) -> DeleteStickerFromSet<'a> {
        DeleteStickerFromSet::new(&self.inner, sticker)
    }

    pub(crate) fn delete_webhook(&self) -> DeleteWebhook<'_> {
        DeleteWebhook::new(&self.inner)
    }

    /// Edits the caption of a media message sent via the inline mode.
    pub fn edit_inline_caption<'a>(
        &'a self,
        inline_message_id: inline_message_id::Ref<'a>,
        caption: impl Into<Text<'a>>,
    ) -> EditInlineCaption<'a> {
        EditInlineCaption::new(&self.inner, inline_message_id, caption)
    }

    /// Edits a live location sent via the inline mode.
    pub fn edit_inline_location<'a>(
        &'a self,
        inline_message_id: inline_message_id::Ref<'a>,
        position: (f64, f64),
    ) -> EditInlineLocation<'a> {
        EditInlineLocation::new(&self.inner, inline_message_id, position)
    }

    /// Edits the media of a message sent via the inline mode.
    pub fn edit_inline_media<'a>(
        &'a self,
        inline_message_id: inline_message_id::Ref<'a>,
        media: impl Into<EditableMedia<'a>>,
    ) -> EditInlineMedia<'a> {
        EditInlineMedia::new(&self.inner, inline_message_id, media)
    }

    /// Edits the inline keyboard of a message sent via the inline mode.
    pub fn edit_inline_reply_markup<'a>(
        &'a self,
        inline_message_id: inline_message_id::Ref<'a>,
        reply_markup: inline::Keyboard<'a>,
    ) -> EditInlineReplyMarkup<'a> {
        EditInlineReplyMarkup::new(&self.inner, inline_message_id, reply_markup)
    }

    /// Edits the text of a message sent via the inline mode.
    pub fn edit_inline_text<'a>(
        &'a self,
        inline_message_id: inline_message_id::Ref<'a>,
        text: impl Into<Text<'a>>,
    ) -> EditInlineText<'a> {
        EditInlineText::new(&self.inner, inline_message_id, text)
    }

    /// Edits the caption of a media message sent by the bot itself.
    pub fn edit_message_caption<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
        caption: impl Into<Text<'a>>,
    ) -> EditMessageCaption<'a> {
        EditMessageCaption::new(&self.inner, chat_id, message_id, caption)
    }

    /// Edits a live location sent by the bot itself.
    pub fn edit_message_location<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
        position: (f64, f64),
    ) -> EditMessageLocation<'a> {
        EditMessageLocation::new(&self.inner, chat_id, message_id, position)
    }

    /// Edits a live location sent by the bot itself.
    pub fn edit_message_media<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
        media: impl Into<EditableMedia<'a>>,
    ) -> EditMessageMedia<'a> {
        EditMessageMedia::new(&self.inner, chat_id, message_id, media)
    }

    /// Edits the inline keyboard of a message sent by the bot itself.
    pub fn edit_message_reply_markup<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
        reply_markup: inline::Keyboard<'a>,
    ) -> EditMessageReplyMarkup<'a> {
        EditMessageReplyMarkup::new(
            &self.inner,
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
    ) -> EditMessageText<'a> {
        EditMessageText::new(&self.inner, chat_id, message_id, text)
    }

    /// Exports a chat's invite link.
    pub fn export_chat_invite_link<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
    ) -> ExportChatInviteLink<'a> {
        ExportChatInviteLink::new(&self.inner, chat_id)
    }

    /// Forwards a message.
    pub fn forward_message<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        from_chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
    ) -> ForwardMessage<'a> {
        ForwardMessage::new(&self.inner, chat_id, from_chat_id, message_id)
    }

    /// Gets information about a chat.
    pub fn get_chat<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
    ) -> GetChat<'a> {
        GetChat::new(&self.inner, chat_id)
    }

    /// Gets information about a file.
    pub fn get_file<'a>(&'a self, file_id: &'a impl AsFileId) -> GetFile<'a> {
        GetFile::new(&self.inner, file_id)
    }

    /// Gets an excerpt from the high score table of a game sent via the inline
    /// mode.
    pub fn get_inline_game_high_scores<'a>(
        &'a self,
        inline_message_id: inline_message_id::Ref<'a>,
        user_id: user::Id,
    ) -> GetInlineGameHighScores<'a> {
        GetInlineGameHighScores::new(&self.inner, inline_message_id, user_id)
    }

    /// Gets information about a chat's admins.
    pub fn get_chat_administrators<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
    ) -> GetChatAdministrators<'a> {
        GetChatAdministrators::new(&self.inner, chat_id)
    }

    /// Gets information about a chat's member.
    pub fn get_chat_member<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        user_id: user::Id,
    ) -> GetChatMember<'a> {
        GetChatMember::new(&self.inner, chat_id, user_id)
    }

    /// Gets a chat's member count.
    pub fn get_chat_members_count<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
    ) -> GetChatMembersCount<'a> {
        GetChatMembersCount::new(&self.inner, chat_id)
    }

    /// Gets an excerpt from the high score table of a game sent by the bot
    /// itself.
    pub fn get_message_game_high_scores<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
        user_id: user::Id,
    ) -> GetMessageGameHighScores<'a> {
        GetMessageGameHighScores::new(&self.inner, chat_id, message_id, user_id)
    }

    /// Gets information about the bot.
    pub fn get_me(&self) -> GetMe<'_> {
        GetMe::new(&self.inner)
    }

    /// Gets the list of the bot's commands.
    pub fn get_my_commands(&self) -> GetMyCommands<'_> {
        GetMyCommands::new(&self.inner)
    }

    /// Gets a sticker set by its name.
    pub fn get_sticker_set<'a>(&'a self, name: &'a str) -> GetStickerSet<'a> {
        GetStickerSet::new(&self.inner, name)
    }

    pub(crate) fn get_updates<'a>(
        &'a self,
        offset: Option<isize>,
        limit: Option<u8>,
        timeout: Option<u64>,
        allowed_updates: Option<&'a [UpdateKind]>,
    ) -> GetUpdates<'a> {
        GetUpdates::new(&self.inner, offset, limit, timeout, allowed_updates)
    }

    /// Gets a user's profile photos.
    pub fn get_user_profile_photos(
        &self,
        user_id: user::Id,
    ) -> GetUserProfilePhotos<'_> {
        GetUserProfilePhotos::new(&self.inner, user_id)
    }

    /// Gets information about the bot's webhook.
    pub fn get_webhook_info(&self) -> GetWebhookInfo<'_> {
        GetWebhookInfo::new(&self.inner)
    }

    /// Kicks a member out of a chat.
    pub fn kick_chat_member<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        user_id: user::Id,
    ) -> KickChatMember<'a> {
        KickChatMember::new(&self.inner, chat_id, user_id)
    }

    /// Leaves a chat.
    pub fn leave_chat<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
    ) -> LeaveChat<'a> {
        LeaveChat::new(&self.inner, chat_id)
    }

    /// Pins a message in a chat.
    pub fn pin_chat_message<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
    ) -> PinChatMessage<'a> {
        PinChatMessage::new(&self.inner, chat_id, message_id)
    }

    /// Promotes a chat member to an admin.
    pub fn promote_chat_member<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        user_id: user::Id,
    ) -> PromoteChatMember<'a> {
        PromoteChatMember::new(&self.inner, chat_id, user_id)
    }

    /// Restricts a chat member.
    pub fn restrict_chat_member<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        user_id: user::Id,
        permissions: chat::Permissions,
    ) -> RestrictChatMember<'a> {
        RestrictChatMember::new(&self.inner, chat_id, user_id, permissions)
    }

    /// Sends an animation.
    pub fn send_animation<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        animation: Animation<'a>,
    ) -> SendAnimation<'a> {
        SendAnimation::new(&self.inner, chat_id, animation)
    }

    /// Sends an audio.
    pub fn send_audio<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        audio: Audio<'a>,
    ) -> SendAudio<'a> {
        SendAudio::new(&self.inner, chat_id, audio)
    }

    /// Sends a chat action.
    pub fn send_chat_action<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        action: chat::Action,
    ) -> SendChatAction<'a> {
        SendChatAction::new(&self.inner, chat_id, action)
    }

    /// Sends a contact.
    pub fn send_contact<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        phone_number: &'a str,
        first_name: &'a str,
    ) -> SendContact<'a> {
        SendContact::new(&self.inner, chat_id, phone_number, first_name)
    }

    /// Sends a game.
    pub fn send_game<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        game_short_name: &'a str,
    ) -> SendGame<'a> {
        SendGame::new(&self.inner, chat_id, game_short_name)
    }

    /// Sends a dice.
    pub fn send_dice<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
    ) -> SendDice<'a> {
        SendDice::new(&self.inner, chat_id)
    }

    /// Sends a document.
    pub fn send_document<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        document: Document<'a>,
    ) -> SendDocument<'a> {
        SendDocument::new(&self.inner, chat_id, document)
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
    ) -> SendInvoice<'a> {
        SendInvoice::new(
            &self.inner,
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
    ) -> SendLocation<'a> {
        SendLocation::new(&self.inner, chat_id, position)
    }

    /// Sends an album.
    pub fn send_media_group<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        media: &'a [GroupMedia<'a>],
    ) -> SendMediaGroup<'a> {
        SendMediaGroup::new(&self.inner, chat_id, media)
    }

    /// Sends a text message.
    pub fn send_message<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        text: impl Into<Text<'a>>,
    ) -> SendMessage<'a> {
        SendMessage::new(&self.inner, chat_id, text)
    }

    /// Sends a photo.
    pub fn send_photo<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        photo: Photo<'a>,
    ) -> SendPhoto<'a> {
        SendPhoto::new(&self.inner, chat_id, photo)
    }

    /// Sends a poll.
    pub fn send_poll<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        poll: &'a Any<'a>,
    ) -> SendPoll<'a> {
        SendPoll::new(&self.inner, chat_id, poll)
    }

    /// Sends a sticker.
    pub fn send_sticker<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        sticker: Sticker<'a>,
    ) -> SendSticker<'a> {
        SendSticker::new(&self.inner, chat_id, sticker)
    }

    /// Sends a venue.
    pub fn send_venue<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        position: (f64, f64),
        title: &'a str,
        address: &'a str,
    ) -> SendVenue<'a> {
        SendVenue::new(&self.inner, chat_id, position, title, address)
    }

    /// Sends a video note.
    pub fn send_video_note<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        video_note: VideoNote<'a>,
    ) -> SendVideoNote<'a> {
        SendVideoNote::new(&self.inner, chat_id, video_note)
    }

    /// Sends a video.
    pub fn send_video<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        video: Video<'a>,
    ) -> SendVideo<'a> {
        SendVideo::new(&self.inner, chat_id, video)
    }

    /// Sends a voice.
    pub fn send_voice<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        voice: Voice<'a>,
    ) -> SendVoice<'a> {
        SendVoice::new(&self.inner, chat_id, voice)
    }

    /// Sets a custom title for an admin in a chat.
    pub fn set_chat_administrator_custom_title<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        user_id: user::Id,
        custom_title: &'a str,
    ) -> SetChatAdministratorCustomTitle<'a> {
        SetChatAdministratorCustomTitle::new(
            &self.inner,
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
    ) -> SetChatDescription<'a> {
        SetChatDescription::new(&self.inner, chat_id, description)
    }

    /// Sets a group's global permissions.
    pub fn set_chat_permissions<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        permissions: chat::Permissions,
    ) -> SetChatPermissions<'a> {
        SetChatPermissions::new(&self.inner, chat_id, permissions)
    }

    /// Sets a chat's photo.
    pub fn set_chat_photo<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        photo: ChatPhoto<'a>,
    ) -> SetChatPhoto<'a> {
        SetChatPhoto::new(&self.inner, chat_id, photo)
    }

    /// Sets a group's sticker set.
    pub fn set_chat_sticker_set<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        sticker_set_name: &'a str,
    ) -> SetChatStickerSet<'a> {
        SetChatStickerSet::new(&self.inner, chat_id, sticker_set_name)
    }

    /// Sets a group's title.
    pub fn set_chat_title<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        title: &'a str,
    ) -> SetChatTitle<'a> {
        SetChatTitle::new(&self.inner, chat_id, title)
    }

    /// Sets a user's new high score in a game sent via the inline mode.
    pub fn set_inline_game_score<'a>(
        &'a self,
        inline_message_id: inline_message_id::Ref<'a>,
        user_id: user::Id,
        score: u32,
    ) -> SetInlineGameScore<'a> {
        SetInlineGameScore::new(&self.inner, inline_message_id, user_id, score)
    }

    /// Sets a user's new high score in a game sent by the bot itself.
    pub fn set_message_game_score<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
        user_id: user::Id,
        score: u32,
    ) -> SetMessageGameScore<'a> {
        SetMessageGameScore::new(
            &self.inner,
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
    ) -> SetMyCommands<'a> {
        SetMyCommands::new(&self.inner, commands)
    }

    /// Reports passport errors to the user.
    pub fn set_passport_data_errors<'a>(
        &'a self,
        user_id: user::Id,
        errors: &'a [passport::element::Error<'a>],
    ) -> SetPassportDataErrors<'a> {
        SetPassportDataErrors::new(&self.inner, user_id, errors)
    }

    /// Changes a sticker's position in a sticker set.
    pub fn set_sticker_position_in_set<'a>(
        &'a self,
        sticker: &'a str,
        position: u32,
    ) -> SetStickerPositionInSet<'a> {
        SetStickerPositionInSet::new(&self.inner, sticker, position)
    }

    /// Sets the thumb of a sticker set.
    pub fn set_sticker_set_thumb<'a>(
        &'a self,
        user_id: user::Id,
        name: &'a str,
        thumb: Option<&'a StickerSetThumb<'a>>,
    ) -> SetStickerSetThumb<'a> {
        SetStickerSetThumb::new(&self.inner, user_id, name, thumb)
    }

    pub(crate) fn set_webhook<'a>(
        &'a self,
        url: &'a str,
        certificate: Option<&'a str>,
        max_connections: Option<u8>,
        allowed_updates: Option<&'a [UpdateKind]>,
    ) -> SetWebhook<'a> {
        SetWebhook::new(
            &self.inner,
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
    ) -> StopInlineLocation<'a> {
        StopInlineLocation::new(&self.inner, inline_message_id)
    }

    /// Stops a live location sent by the bot itself.
    pub fn stop_message_location<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
    ) -> StopMessageLocation<'a> {
        StopMessageLocation::new(&self.inner, chat_id, message_id)
    }

    /// Stops a poll.
    pub fn stop_poll<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
    ) -> StopPoll<'a> {
        StopPoll::new(&self.inner, chat_id, message_id)
    }

    /// Lifts all restrictions from a group's member.
    pub fn unban_chat_member<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        user_id: user::Id,
    ) -> UnbanChatMember<'a> {
        UnbanChatMember::new(&self.inner, chat_id, user_id)
    }

    /// Unpins a chat message.
    pub fn unpin_chat_message<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
    ) -> UnpinChatMessage<'a> {
        UnpinChatMessage::new(&self.inner, chat_id)
    }

    /// Uploads a sticker file.
    pub fn upload_sticker_file<'a>(
        &'a self,
        user_id: user::Id,
        png_sticker: &'a [u8],
    ) -> UploadStickerFile<'a> {
        UploadStickerFile::new(&self.inner, user_id, png_sticker)
    }
}

/// Constructs a new [`Bot`], extracting the token from the environment at
/// _compile time_.
///
/// You can provide the proxy as the second parameter,
/// e.g. `from_env!("...", proxy)`.
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
    ($var:literal, $proxy:expr) => {{
        let token = env!($var).to_string();
        $crate::Bot::with_proxy(token, $proxy)
    }};
    ($var:literal, $proxy:expr,) => {
        $crate::from_env!($var, $proxy)
    };
    () => {
        compile_error!(
            "the macro must be invoked as `from_env!(\"<VAR_NAME>\")` or \
             `from_env!(\"<VAR_NAME>\", proxy)`"
        )
    };
    ($($x:tt)+) => {
        $crate::from_env!()
    };
}

impl Sealed for Bot {}

fn extract_token(env_var: &'static str) -> String {
    std::env::var(env_var).unwrap_or_else(|_| {
        panic!("\n[tbot] Bot's token in {} was not specified\n", env_var)
    })
}

#[cfg(test)]
mod tests {
    use crate::proxy::https::{Intercept, Proxy};

    #[test]
    fn macro_compiles() {
        const PROXY: &str = "http://127.0.0.1:8080";

        let _ = from_env!("BOT_TOKEN");
        let _ = from_env!("BOT_TOKEN",);

        let proxy = Proxy::new(Intercept::All, PROXY.parse().unwrap());
        let _ = from_env!("BOT_TOKEN", proxy);

        let proxy = Proxy::new(Intercept::All, PROXY.parse().unwrap());
        let _ = from_env!("BOT_TOKEN", proxy,);
    }
}

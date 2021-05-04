//! The [`Bot`] struct, and other types used to construct it.
//!
//! See [`Bot`]'s documentation on how to interact with the API and how to
//! construct a [`Bot`].
//!
//! [`Bot`]: ./struct.Bot.html

// Type out about 80 method names? No, thanks
#![allow(clippy::wildcard_imports)]
use crate::{
    download_file, errors,
    event_loop::EventLoop,
    internal::Sealed,
    methods::*,
    state::StatefulEventLoop,
    types::{
        callback, chat,
        file::{self, File},
        inline_query,
        input_file::{
            Animation, Audio, ChatPhoto, Document, EditableMedia, MediaGroup,
            Photo, Sticker, StickerForStickerSet, StickerSetThumb, Video,
            VideoNote, Voice,
        },
        keyboard::inline,
        message,
        parameters::{
            poll, AllowedUpdates, BotCommand, CallbackAction, ImplicitChatId,
            Text,
        },
        passport, pre_checkout_query, shipping, user, InlineMessageId,
        LabeledPrice,
    },
};
use std::{borrow::Cow, net::IpAddr, num::NonZeroU32, sync::Arc};

mod builder;
mod inner_bot;

pub use builder::Builder;
pub use hyper::Uri;
pub(crate) use inner_bot::InnerBot;

/// A `Bot` is the entry point to interacting with the Bot API.
///
/// Using a `Bot`, you can call methods from the [`methods`] module:
///
/// ```no_run
/// # async fn foo() -> Result<(), tbot::errors::MethodCall> {
/// let bot = tbot::from_env!("BOT_TOKEN");
/// let me = bot.get_me().call().await?;
/// dbg!(me);
/// # Ok(())
/// # }
/// ```
///
/// [`methods`]: ../methods/index.html
///
/// A `Bot` is also used to construct an [`EventLoop`] — a struct
/// responsible for configuring handlers and listening to updates:
///
/// ```no_run
/// # async fn foo() -> Result<(), tbot::errors::PollingSetup> {
/// use tbot::prelude::*;
///
/// let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();
///
/// bot.start(|context| async move {
///    let result = context.send_message_in_reply("Hi!").call().await;
///
///     if let Err(error) = result {
///         eprintln!("Failed to send a message: {}", error);
///     }
/// });
///
/// bot.polling().start().await?;
/// # Ok(())
/// # }
/// ```
///
/// [`EventLoop`]: ../event_loop/struct.EventLoop.html
///
/// A `Bot` can be constructed in several ways:
///
/// - The [`from_env!`] macro constructs a bot extracting its token from
///   an enviroment variable at _compile time_;
/// - The [`Bot::from_env`] method constructs a bot extracting its token from
///   an environment variable at _runtime_;
/// - The [`Bot::new`] methods constructs a bot, and its token is provided
///   as a parameter;
/// - The [`bot::Builder`] provides advanced configuration of a bot. You can
///   set up an HTTPS/SOCKS5 proxy or your local Bot API server's URL using it.
///
/// The bot's internal data (its token, proxy and the URL where it makes
/// requests) is kept behind an [`Arc`]. It means that you can clone a `Bot`
/// cheaply to share it between tasks.
///
/// [`from_env!`]: ../macro.from_env.html
/// [`Bot::from_env`]: #method.from_env
/// [`bot::Builder`]: ./struct.Builder.html
/// [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
#[derive(Debug, Clone)]
#[must_use]
pub struct Bot {
    inner: Arc<InnerBot>,
}

impl Bot {
    /// Constructs a new `Bot`.
    ///
    /// This method is a shorthand for a common case. If you need advanced
    /// configuration, e.g. you want to set a proxy or use a local Bot API
    /// server, construct a `Bot` using a [`bot::Builder`].
    ///
    /// [`bot::Builder`]: ./struct.Builder.html
    pub fn new(token: String) -> Self {
        Builder::with_string_token(token).build()
    }

    /// Constructs a new `Bot`, extracting the token from the environment at
    /// _runtime_.
    ///
    /// If you need to extract the token at _compile time_, use [`from_env!`].
    ///
    /// This method is a shorthand for a common case. If you need advanced
    /// configuration, e.g. you want to set a proxy or use a local Bot API
    /// server, construct a `Bot` using a [`bot::Builder`].
    ///
    /// [`from_env!`]: ../macro.bot.html
    /// [`bot::Builder`]: ./struct.Builder.html
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn foo() {
    /// use tbot::Bot;
    ///
    /// let bot = Bot::from_env("BOT_TOKEN");
    /// let me = bot.get_me().call().await.unwrap();
    /// dbg!(me);
    /// # }
    /// ```
    pub fn from_env(env_var: &'static str) -> Self {
        Builder::with_env_token(env_var).build()
    }

    /// Downloads a file.
    ///
    /// If you use a self-hosted Bot API server, `file.path` may be an absolute
    /// local path. In this case, `tbot` reads the file at the given and returns
    /// it. If the server is running on another machine, you have to handle this
    /// case manually before calling this method.
    pub async fn download_file(
        &self,
        file: &File,
    ) -> Result<Vec<u8>, errors::Download> {
        download_file(&self.inner, file).await
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
    pub fn add_sticker_to_set(
        &self,
        user_id: user::Id,
        name: impl Into<String>,
        png_sticker: impl Into<StickerForStickerSet>,
        emojis: impl Into<String>,
    ) -> AddStickerToSet<'_> {
        AddStickerToSet::new(&self.inner, user_id, name, png_sticker, emojis)
    }

    pub(crate) fn answer_callback_query(
        &self,
        callback_query_id: callback::query::Id,
        action: Option<CallbackAction>,
    ) -> AnswerCallbackQuery<'_> {
        AnswerCallbackQuery::new(&self.inner, callback_query_id, action)
    }

    pub(crate) fn answer_inline_query<'a>(
        &'a self,
        inline_query_id: inline_query::Id,
        results: impl Into<Cow<'a, [inline_query::Result]>>,
    ) -> AnswerInlineQuery<'a> {
        AnswerInlineQuery::new(&self.inner, inline_query_id, results)
    }

    pub(crate) fn answer_pre_checkout_query(
        &self,
        pre_checkout_query_id: pre_checkout_query::Id,
        result: Result<(), impl Into<String>>,
    ) -> AnswerPreCheckoutQuery<'_> {
        AnswerPreCheckoutQuery::new(&self.inner, pre_checkout_query_id, result)
    }

    pub(crate) fn answer_shipping_query<'a>(
        &'a self,
        shipping_query_id: shipping::query::Id,
        result: Result<
            impl Into<Cow<'a, [shipping::Option]>>,
            impl Into<String>,
        >,
    ) -> AnswerShippingQuery<'a> {
        AnswerShippingQuery::new(&self.inner, shipping_query_id, result)
    }

    /// Copies a message.
    pub fn copy_message(
        &self,
        chat_id: impl ImplicitChatId,
        from_chat_id: impl ImplicitChatId,
        message_id: message::Id,
    ) -> CopyMessage<'_> {
        CopyMessage::new(&self.inner, chat_id, from_chat_id, message_id)
    }

    /// Creates a new sticker set.
    pub fn create_new_sticker_set(
        &self,
        user_id: user::Id,
        name: impl Into<String>,
        title: impl Into<String>,
        png_sticker: impl Into<StickerForStickerSet>,
        emojis: impl Into<String>,
    ) -> CreateNewStickerSet<'_> {
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
    pub fn delete_chat_photo(
        &self,
        chat_id: impl ImplicitChatId,
    ) -> DeleteChatPhoto<'_> {
        DeleteChatPhoto::new(&self.inner, chat_id)
    }

    /// Deletes a chat's sticker set.
    pub fn delete_chat_sticker_set(
        &self,
        chat_id: impl ImplicitChatId,
    ) -> DeleteChatStickerSet<'_> {
        DeleteChatStickerSet::new(&self.inner, chat_id)
    }

    /// Deletes a message from a chat.
    pub fn delete_message(
        &self,
        chat_id: impl ImplicitChatId,
        message_id: message::Id,
    ) -> DeleteMessage<'_> {
        DeleteMessage::new(&self.inner, chat_id, message_id)
    }

    /// Deletes a sticker from a sticker set.
    pub fn delete_sticker_from_set(
        &self,
        sticker: impl Into<String>,
    ) -> DeleteStickerFromSet<'_> {
        DeleteStickerFromSet::new(&self.inner, sticker)
    }

    pub(crate) fn delete_webhook(&self) -> DeleteWebhook<'_> {
        DeleteWebhook::new(&self.inner)
    }

    /// Edits the caption of a media message sent via the inline mode.
    pub fn edit_inline_caption(
        &self,
        inline_message_id: InlineMessageId,
        caption: impl Into<Text>,
    ) -> EditInlineCaption<'_> {
        EditInlineCaption::new(&self.inner, inline_message_id, caption)
    }

    /// Edits a live location sent via the inline mode.
    pub fn edit_inline_location(
        &self,
        inline_message_id: InlineMessageId,
        position: (f64, f64),
    ) -> EditInlineLocation<'_> {
        EditInlineLocation::new(&self.inner, inline_message_id, position)
    }

    /// Edits the media of a message sent via the inline mode.
    pub fn edit_inline_media(
        &self,
        inline_message_id: InlineMessageId,
        media: impl Into<EditableMedia>,
    ) -> EditInlineMedia<'_> {
        EditInlineMedia::new(&self.inner, inline_message_id, media)
    }

    /// Edits the inline keyboard of a message sent via the inline mode.
    pub fn edit_inline_reply_markup(
        &self,
        inline_message_id: InlineMessageId,
        reply_markup: inline::Keyboard,
    ) -> EditInlineReplyMarkup<'_> {
        EditInlineReplyMarkup::new(&self.inner, inline_message_id, reply_markup)
    }

    /// Edits the text of a message sent via the inline mode.
    pub fn edit_inline_text(
        &self,
        inline_message_id: InlineMessageId,
        text: impl Into<Text>,
    ) -> EditInlineText<'_> {
        EditInlineText::new(&self.inner, inline_message_id, text)
    }

    /// Edits the caption of a media message sent by the bot itself.
    pub fn edit_message_caption(
        &self,
        chat_id: impl ImplicitChatId,
        message_id: message::Id,
        caption: impl Into<Text>,
    ) -> EditMessageCaption<'_> {
        EditMessageCaption::new(&self.inner, chat_id, message_id, caption)
    }

    /// Edits a live location sent by the bot itself.
    pub fn edit_message_location(
        &self,
        chat_id: impl ImplicitChatId,
        message_id: message::Id,
        position: (f64, f64),
    ) -> EditMessageLocation<'_> {
        EditMessageLocation::new(&self.inner, chat_id, message_id, position)
    }

    /// Edits a live location sent by the bot itself.
    pub fn edit_message_media(
        &self,
        chat_id: impl ImplicitChatId,
        message_id: message::Id,
        media: impl Into<EditableMedia>,
    ) -> EditMessageMedia<'_> {
        EditMessageMedia::new(&self.inner, chat_id, message_id, media)
    }

    /// Edits the inline keyboard of a message sent by the bot itself.
    pub fn edit_message_reply_markup(
        &self,
        chat_id: impl ImplicitChatId,
        message_id: message::Id,
        reply_markup: inline::Keyboard,
    ) -> EditMessageReplyMarkup<'_> {
        EditMessageReplyMarkup::new(
            &self.inner,
            chat_id,
            message_id,
            reply_markup,
        )
    }

    /// Edits the text of a message sent by the bot itself.
    pub fn edit_message_text(
        &self,
        chat_id: impl ImplicitChatId,
        message_id: message::Id,
        text: impl Into<Text>,
    ) -> EditMessageText<'_> {
        EditMessageText::new(&self.inner, chat_id, message_id, text)
    }

    /// Exports a chat's invite link.
    pub fn export_chat_invite_link(
        &self,
        chat_id: impl ImplicitChatId,
    ) -> ExportChatInviteLink<'_> {
        ExportChatInviteLink::new(&self.inner, chat_id)
    }

    /// Forwards a message.
    pub fn forward_message(
        &self,
        chat_id: impl ImplicitChatId,
        from_chat_id: impl ImplicitChatId,
        message_id: message::Id,
    ) -> ForwardMessage<'_> {
        ForwardMessage::new(&self.inner, chat_id, from_chat_id, message_id)
    }

    /// Gets information about a chat.
    pub fn get_chat(&self, chat_id: impl ImplicitChatId) -> GetChat<'_> {
        GetChat::new(&self.inner, chat_id)
    }

    /// Gets information about a file.
    pub fn get_file(&self, file_id: file::Id) -> GetFile<'_> {
        GetFile::new(&self.inner, file_id)
    }

    /// Gets an excerpt from the high score table of a game sent via the inline
    /// mode.
    pub fn get_inline_game_high_scores(
        &self,
        inline_message_id: InlineMessageId,
        user_id: user::Id,
    ) -> GetInlineGameHighScores<'_> {
        GetInlineGameHighScores::new(&self.inner, inline_message_id, user_id)
    }

    /// Gets information about a chat's admins.
    pub fn get_chat_administrators(
        &self,
        chat_id: impl ImplicitChatId,
    ) -> GetChatAdministrators<'_> {
        GetChatAdministrators::new(&self.inner, chat_id)
    }

    /// Gets information about a chat's member.
    pub fn get_chat_member(
        &self,
        chat_id: impl ImplicitChatId,
        user_id: user::Id,
    ) -> GetChatMember<'_> {
        GetChatMember::new(&self.inner, chat_id, user_id)
    }

    /// Gets a chat's member count.
    pub fn get_chat_members_count(
        &self,
        chat_id: impl ImplicitChatId,
    ) -> GetChatMembersCount<'_> {
        GetChatMembersCount::new(&self.inner, chat_id)
    }

    /// Gets an excerpt from the high score table of a game sent by the bot
    /// itself.
    pub fn get_message_game_high_scores(
        &self,
        chat_id: impl ImplicitChatId,
        message_id: message::Id,
        user_id: user::Id,
    ) -> GetMessageGameHighScores<'_> {
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
    pub fn get_sticker_set(
        &self,
        name: impl Into<String>,
    ) -> GetStickerSet<'_> {
        GetStickerSet::new(&self.inner, name)
    }

    pub(crate) fn get_updates(
        &self,
        offset: Option<isize>,
        limit: Option<u8>,
        timeout: Option<u64>,
        allowed_updates: Option<AllowedUpdates>,
    ) -> GetUpdates<'_> {
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
    pub fn kick_chat_member(
        &self,
        chat_id: impl ImplicitChatId,
        user_id: user::Id,
    ) -> KickChatMember<'_> {
        KickChatMember::new(&self.inner, chat_id, user_id)
    }

    /// Leaves a chat.
    pub fn leave_chat(&self, chat_id: impl ImplicitChatId) -> LeaveChat<'_> {
        LeaveChat::new(&self.inner, chat_id)
    }

    /// Pins a message in a chat.
    pub fn pin_chat_message(
        &self,
        chat_id: impl ImplicitChatId,
        message_id: message::Id,
    ) -> PinChatMessage<'_> {
        PinChatMessage::new(&self.inner, chat_id, message_id)
    }

    /// Promotes a chat member to an admin.
    pub fn promote_chat_member(
        &self,
        chat_id: impl ImplicitChatId,
        user_id: user::Id,
    ) -> PromoteChatMember<'_> {
        PromoteChatMember::new(&self.inner, chat_id, user_id)
    }

    /// Restricts a chat member.
    pub fn restrict_chat_member(
        &self,
        chat_id: impl ImplicitChatId,
        user_id: user::Id,
        permissions: chat::Permissions,
    ) -> RestrictChatMember<'_> {
        RestrictChatMember::new(&self.inner, chat_id, user_id, permissions)
    }

    /// Sends an animation.
    pub fn send_animation(
        &self,
        chat_id: impl ImplicitChatId,
        animation: Animation,
    ) -> SendAnimation<'_> {
        SendAnimation::new(&self.inner, chat_id, animation)
    }

    /// Sends an audio.
    pub fn send_audio(
        &self,
        chat_id: impl ImplicitChatId,
        audio: Audio,
    ) -> SendAudio<'_> {
        SendAudio::new(&self.inner, chat_id, audio)
    }

    /// Sends a chat action.
    pub fn send_chat_action(
        &self,
        chat_id: impl ImplicitChatId,
        action: chat::Action,
    ) -> SendChatAction<'_> {
        SendChatAction::new(&self.inner, chat_id, action)
    }

    /// Sends a contact.
    pub fn send_contact(
        &self,
        chat_id: impl ImplicitChatId,
        phone_number: impl Into<String>,
        first_name: impl Into<String>,
    ) -> SendContact<'_> {
        SendContact::new(&self.inner, chat_id, phone_number, first_name)
    }

    /// Sends a game.
    pub fn send_game(
        &self,
        chat_id: impl ImplicitChatId,
        game_short_name: impl Into<String>,
    ) -> SendGame<'_> {
        SendGame::new(&self.inner, chat_id, game_short_name)
    }

    /// Sends a dice.
    pub fn send_dice(&self, chat_id: impl ImplicitChatId) -> SendDice<'_> {
        SendDice::new(&self.inner, chat_id)
    }

    /// Sends a document.
    pub fn send_document(
        &self,
        chat_id: impl ImplicitChatId,
        document: Document,
    ) -> SendDocument<'_> {
        SendDocument::new(&self.inner, chat_id, document)
    }

    /// Sends an invoice.
    #[allow(clippy::too_many_arguments)]
    pub fn send_invoice<'a>(
        &'a self,
        chat_id: impl Into<chat::Id>,
        title: impl Into<String>,
        description: impl Into<String>,
        payload: impl Into<String>,
        provider_token: impl Into<String>,
        start_parameter: impl Into<String>,
        currency: impl Into<String>,
        prices: impl Into<Cow<'a, [LabeledPrice]>>,
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
    pub fn send_location(
        &self,
        chat_id: impl ImplicitChatId,
        position: (f64, f64),
    ) -> SendLocation<'_> {
        SendLocation::new(&self.inner, chat_id, position)
    }

    /// Sends an album.
    pub fn send_media_group(
        &self,
        chat_id: impl ImplicitChatId,
        media: impl Into<MediaGroup>,
    ) -> SendMediaGroup<'_> {
        SendMediaGroup::new(&self.inner, chat_id, media)
    }

    /// Sends a text message.
    pub fn send_message(
        &self,
        chat_id: impl ImplicitChatId,
        text: impl Into<Text>,
    ) -> SendMessage<'_> {
        SendMessage::new(&self.inner, chat_id, text)
    }

    /// Sends a photo.
    pub fn send_photo(
        &self,
        chat_id: impl ImplicitChatId,
        photo: Photo,
    ) -> SendPhoto<'_> {
        SendPhoto::new(&self.inner, chat_id, photo)
    }

    /// Sends a poll.
    pub fn send_poll(
        &self,
        chat_id: impl ImplicitChatId,
        poll: poll::Any,
    ) -> SendPoll<'_> {
        SendPoll::new(&self.inner, chat_id, poll)
    }

    /// Sends a sticker.
    pub fn send_sticker(
        &self,
        chat_id: impl ImplicitChatId,
        sticker: Sticker,
    ) -> SendSticker<'_> {
        SendSticker::new(&self.inner, chat_id, sticker)
    }

    /// Sends a venue.
    pub fn send_venue(
        &self,
        chat_id: impl ImplicitChatId,
        position: (f64, f64),
        title: impl Into<String>,
        address: impl Into<String>,
    ) -> SendVenue<'_> {
        SendVenue::new(&self.inner, chat_id, position, title, address)
    }

    /// Sends a video note.
    pub fn send_video_note(
        &self,
        chat_id: impl ImplicitChatId,
        video_note: VideoNote,
    ) -> SendVideoNote<'_> {
        SendVideoNote::new(&self.inner, chat_id, video_note)
    }

    /// Sends a video.
    pub fn send_video(
        &self,
        chat_id: impl ImplicitChatId,
        video: Video,
    ) -> SendVideo<'_> {
        SendVideo::new(&self.inner, chat_id, video)
    }

    /// Sends a voice.
    pub fn send_voice(
        &self,
        chat_id: impl ImplicitChatId,
        voice: Voice,
    ) -> SendVoice<'_> {
        SendVoice::new(&self.inner, chat_id, voice)
    }

    /// Sets a custom title for an admin in a chat.
    pub fn set_chat_administrator_custom_title(
        &self,
        chat_id: impl ImplicitChatId,
        user_id: user::Id,
        custom_title: impl Into<String>,
    ) -> SetChatAdministratorCustomTitle<'_> {
        SetChatAdministratorCustomTitle::new(
            &self.inner,
            chat_id,
            user_id,
            custom_title,
        )
    }

    /// Sets a chat's description.
    pub fn set_chat_description(
        &self,
        chat_id: impl ImplicitChatId,
        description: impl Into<String>,
    ) -> SetChatDescription<'_> {
        SetChatDescription::new(&self.inner, chat_id, description)
    }

    /// Sets a group's global permissions.
    pub fn set_chat_permissions(
        &self,
        chat_id: impl ImplicitChatId,
        permissions: chat::Permissions,
    ) -> SetChatPermissions<'_> {
        SetChatPermissions::new(&self.inner, chat_id, permissions)
    }

    /// Sets a chat's photo.
    pub fn set_chat_photo(
        &self,
        chat_id: impl ImplicitChatId,
        photo: ChatPhoto,
    ) -> SetChatPhoto<'_> {
        SetChatPhoto::new(&self.inner, chat_id, photo)
    }

    /// Sets a group's sticker set.
    pub fn set_chat_sticker_set(
        &self,
        chat_id: impl ImplicitChatId,
        sticker_set_name: impl Into<String>,
    ) -> SetChatStickerSet<'_> {
        SetChatStickerSet::new(&self.inner, chat_id, sticker_set_name)
    }

    /// Sets a group's title.
    pub fn set_chat_title(
        &self,
        chat_id: impl ImplicitChatId,
        title: impl Into<String>,
    ) -> SetChatTitle<'_> {
        SetChatTitle::new(&self.inner, chat_id, title)
    }

    /// Sets a user's new high score in a game sent via the inline mode.
    pub fn set_inline_game_score(
        &self,
        inline_message_id: InlineMessageId,
        user_id: user::Id,
        score: u32,
    ) -> SetInlineGameScore<'_> {
        SetInlineGameScore::new(&self.inner, inline_message_id, user_id, score)
    }

    /// Sets a user's new high score in a game sent by the bot itself.
    pub fn set_message_game_score(
        &self,
        chat_id: impl ImplicitChatId,
        message_id: message::Id,
        user_id: user::Id,
        score: u32,
    ) -> SetMessageGameScore<'_> {
        SetMessageGameScore::new(
            &self.inner,
            chat_id,
            message_id,
            user_id,
            score,
        )
    }

    /// Sets the list of the bot's commands.
    pub fn set_my_commands(
        &self,
        commands: impl Into<Vec<BotCommand>>,
    ) -> SetMyCommands<'_> {
        SetMyCommands::new(&self.inner, commands)
    }

    /// Reports passport errors to the user.
    pub fn set_passport_data_errors(
        &self,
        user_id: user::Id,
        errors: impl Into<Vec<passport::element::Error>>,
    ) -> SetPassportDataErrors<'_> {
        SetPassportDataErrors::new(&self.inner, user_id, errors)
    }

    /// Changes a sticker's position in a sticker set.
    pub fn set_sticker_position_in_set(
        &self,
        sticker: impl Into<String>,
        position: u32,
    ) -> SetStickerPositionInSet<'_> {
        SetStickerPositionInSet::new(&self.inner, sticker, position)
    }

    /// Sets the thumb of a sticker set.
    pub fn set_sticker_set_thumb(
        &self,
        user_id: user::Id,
        name: impl Into<String>,
        thumb: Option<StickerSetThumb>,
    ) -> SetStickerSetThumb<'_> {
        SetStickerSetThumb::new(&self.inner, user_id, name, thumb)
    }

    pub(crate) fn set_webhook<'a>(
        &'a self,
        url: &'a str,
        ip_address: Option<IpAddr>,
        certificate: Option<&'a str>,
        max_connections: Option<NonZeroU32>,
        allowed_updates: Option<AllowedUpdates>,
        drop_pending_updates: bool,
    ) -> SetWebhook<'a> {
        SetWebhook::new(
            &self.inner,
            url,
            ip_address,
            certificate,
            max_connections,
            allowed_updates,
            drop_pending_updates,
        )
    }

    /// Stops a live location sent via the inline mode.
    pub fn stop_inline_location(
        &self,
        inline_message_id: InlineMessageId,
    ) -> StopInlineLocation<'_> {
        StopInlineLocation::new(&self.inner, inline_message_id)
    }

    /// Stops a live location sent by the bot itself.
    pub fn stop_message_location(
        &self,
        chat_id: impl ImplicitChatId,
        message_id: message::Id,
    ) -> StopMessageLocation<'_> {
        StopMessageLocation::new(&self.inner, chat_id, message_id)
    }

    /// Stops a poll.
    pub fn stop_poll(
        &self,
        chat_id: impl ImplicitChatId,
        message_id: message::Id,
    ) -> StopPoll<'_> {
        StopPoll::new(&self.inner, chat_id, message_id)
    }

    /// Lifts all restrictions from a group's member.
    pub fn unban_chat_member(
        &self,
        chat_id: impl ImplicitChatId,
        user_id: user::Id,
    ) -> UnbanChatMember<'_> {
        UnbanChatMember::new(&self.inner, chat_id, user_id)
    }

    /// Unpins all messages in a chat.
    pub fn unpin_all_chat_messages(
        &self,
        chat_id: impl ImplicitChatId,
    ) -> UnpinAllChatMessages<'_> {
        UnpinAllChatMessages::new(&self.inner, chat_id)
    }

    /// Unpins a chat message.
    pub fn unpin_chat_message(
        &self,
        chat_id: impl ImplicitChatId,
    ) -> UnpinChatMessage<'_> {
        UnpinChatMessage::new(&self.inner, chat_id)
    }

    /// Uploads a sticker file.
    pub fn upload_sticker_file<'a>(
        &'a self,
        user_id: user::Id,
        png_sticker: impl Into<Cow<'a, [u8]>>,
    ) -> UploadStickerFile<'a> {
        UploadStickerFile::new(&self.inner, user_id, png_sticker)
    }
}

/// Constructs a new [`Bot`], extracting the token from the environment at
/// _compile time_.
///
/// If you need to extract the token at _runtime_, use [`Bot::from_env`].
///
/// This macro is a shorthand for a common case. If you need advanced
/// configuration, e.g. you want to set a proxy or use a local Bot API
/// server, construct a `Bot` using a [`bot::Builder`] and extracting the token
/// using `String::from(env!("BOT_TOKEN"))`.
///
/// [`Bot`]: ./bot/struct.Bot.html
/// [`Bot::from_env`]: ./bot/struct.Bot.html#method.from_env
/// [`bot::Builder`]: ./bot/struct.Builder.html
///
/// # Example
///
/// ```no_run
/// # async fn foo() -> Result<(), tbot::errors::MethodCall> {
/// let mut bot = tbot::from_env!("BOT_TOKEN");
/// let me = bot.get_me().call().await?;
/// dbg!(me);
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! from_env {
    ($var:literal$(,)?) => {{
        let token = env!($var).to_string();
        $crate::Bot::new(token)
    }};
    ($($x:tt)*) => {
        compile_error!("the macro must be invoked as `from_env!(\"VAR_NAME\")`")
    };
}

impl Sealed for Bot {}

#[cfg(test)]
mod tests {
    #[test]
    fn macro_compiles() {
        let _ = from_env!("BOT_TOKEN");
        let _ = from_env!("BOT_TOKEN",);
    }
}

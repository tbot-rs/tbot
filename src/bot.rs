use super::*;
use crate::{
    event_loop::EventLoop,
    types::{
        callback, chat,
        file::{id::AsFileId, File},
        inline_query,
        input_file::*,
        keyboard::inline,
        message,
        parameters::{CallbackAction, ImplicitChatId, Text, Updates},
        passport, pre_checkout_query, shipping, user,
        value::{self, Bytes, InlineMessageId, Ref, Seq},
        LabeledPrice,
    },
};
use std::sync::Arc;

/// Provides methods to call the Bots API.
///
/// A `Bot` lets you call methods from the [`methods`] module.
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
/// [`methods`]: ./methods/index.html
#[derive(Debug, Clone)]
pub struct Bot<C> {
    pub(crate) token: Token,
    pub(crate) client: Arc<crate::internal::Client<C>>,
}

impl Bot<connectors::Https> {
    /// Constructs a new `Bot`.
    pub fn new(token: Token) -> Self {
        Self {
            token,
            client: Arc::new(connectors::default()),
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
        Self::new(extract_token(env_var))
    }
}

impl<C> Bot<C> {
    /// Constructs an `EventLoop`.
    pub fn event_loop(self) -> EventLoop<C> {
        EventLoop::new(self)
    }

    /// Constructs a new `AddStickerToSet` inferring your bot's token.
    pub fn add_sticker_to_set<'a>(
        &'a self,
        user_id: user::Id,
        name: impl Into<value::String<'a>>,
        png_sticker: impl Into<Ref<'a, PngSticker<'a>>>,
        emojis: impl Into<value::String<'a>>,
    ) -> methods::AddStickerToSet<'a, C> {
        methods::AddStickerToSet::new(
            &self.client,
            self.token.clone(),
            user_id,
            name,
            png_sticker,
            emojis,
        )
    }

    /// Constructs a new `AnswerCallbackQuery` inferring your bot's token.
    pub(crate) fn answer_callback_query<'a>(
        &'a self,
        callback_query_id: &'a callback::query::Id,
        action: impl Into<Ref<'a, CallbackAction<'a>>>,
    ) -> methods::AnswerCallbackQuery<'a, C> {
        methods::AnswerCallbackQuery::new(
            &self.client,
            self.token.clone(),
            callback_query_id,
            action,
        )
    }

    /// Constructs a new `AnswerInlineQuery` inferring your bot's token.
    pub(crate) fn answer_inline_query<'a>(
        &'a self,
        inline_query_id: &'a inline_query::Id,
        results: impl Into<Seq<'a, Ref<'a, inline_query::Result<'a>>>>,
    ) -> methods::AnswerInlineQuery<'a, C> {
        methods::AnswerInlineQuery::new(
            &self.client,
            self.token.clone(),
            inline_query_id,
            results,
        )
    }

    /// Constructs a new `AnswerPreCheckoutQuery` inferring your bot's token.
    pub(crate) fn answer_pre_checkout_query<'a>(
        &'a self,
        pre_checkout_query_id: &'a pre_checkout_query::Id,
        result: Result<(), value::String<'a>>,
    ) -> methods::AnswerPreCheckoutQuery<'a, C> {
        methods::AnswerPreCheckoutQuery::new(
            &self.client,
            self.token.clone(),
            pre_checkout_query_id,
            result,
        )
    }

    /// Constructs a new `AnswerShippingQuery` inferring your bot's token.
    pub(crate) fn answer_shipping_query<'a>(
        &'a self,
        shipping_query_id: &'a shipping::query::Id,
        result: Result<
            Seq<'a, Ref<'a, shipping::Option<'a>>>,
            value::String<'a>,
        >,
    ) -> methods::AnswerShippingQuery<'a, C> {
        methods::AnswerShippingQuery::new(
            &self.client,
            self.token.clone(),
            shipping_query_id,
            result,
        )
    }

    /// Constructs a new `CreateNewStickerSet` inferring your bot's token.
    pub fn create_new_sticker_set<'a>(
        &'a self,
        user_id: user::Id,
        name: impl Into<value::String<'a>>,
        title: impl Into<value::String<'a>>,
        png_sticker: impl Into<Ref<'a, PngSticker<'a>>>,
        emojis: impl Into<value::String<'a>>,
    ) -> methods::CreateNewStickerSet<'a, C> {
        methods::CreateNewStickerSet::new(
            &self.client,
            self.token.clone(),
            user_id,
            name,
            title,
            png_sticker,
            emojis,
        )
    }

    /// Constructs a new `DeleteChatPhoto` inferring your bot's token.
    pub fn delete_chat_photo<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
    ) -> methods::DeleteChatPhoto<'a, C> {
        methods::DeleteChatPhoto::new(&self.client, self.token.clone(), chat_id)
    }

    /// Constructs a new `DeleteChatStickerSet` inferring your bot's token.
    pub fn delete_chat_sticker_set<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
    ) -> methods::DeleteChatStickerSet<'a, C> {
        methods::DeleteChatStickerSet::new(
            &self.client,
            self.token.clone(),
            chat_id,
        )
    }

    /// Constructs a new `DeleteMessage` inferring your bot's token.
    pub fn delete_message<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
    ) -> methods::DeleteMessage<'a, C> {
        methods::DeleteMessage::new(
            &self.client,
            self.token.clone(),
            chat_id,
            message_id,
        )
    }

    /// Constructs a new `DeleteStickerFromSet` inferring your bot's token.
    pub fn delete_sticker_from_set<'a>(
        &'a self,
        sticker: impl Into<value::String<'a>>,
    ) -> methods::DeleteStickerFromSet<'a, C> {
        methods::DeleteStickerFromSet::new(
            &self.client,
            self.token.clone(),
            sticker,
        )
    }

    pub(crate) fn delete_webhook(&self) -> methods::DeleteWebhook<'_, C> {
        methods::DeleteWebhook::new(&self.client, self.token.clone())
    }

    /// Constructs a new `EditInlineCaption` inferring your bot's token.
    pub fn edit_inline_caption<'a>(
        &'a self,
        inline_message_id: impl Into<InlineMessageId<'a>>,
        caption: impl Into<Text<'a>>,
    ) -> methods::EditInlineCaption<'a, C> {
        methods::EditInlineCaption::new(
            &self.client,
            self.token.clone(),
            inline_message_id,
            caption,
        )
    }

    /// Constructs a new `EditInlineLocation` inferring your bot's token.
    pub fn edit_inline_location<'a>(
        &'a self,
        inline_message_id: impl Into<InlineMessageId<'a>>,
        position: (f64, f64),
    ) -> methods::EditInlineLocation<'a, C> {
        methods::EditInlineLocation::new(
            &self.client,
            self.token.clone(),
            inline_message_id,
            position,
        )
    }

    /// Constructs a new `EditInlineMedia` inferring your bot's token.
    pub fn edit_inline_media<'a>(
        &'a self,
        inline_message_id: impl Into<InlineMessageId<'a>>,
        media: impl Into<Ref<'a, EditableMedia<'a>>>,
    ) -> methods::EditInlineMedia<'a, C> {
        methods::EditInlineMedia::new(
            &self.client,
            self.token.clone(),
            inline_message_id,
            media,
        )
    }

    /// Constructs a new `EditInlineReplyMarkup` inferring your bot's token.
    pub fn edit_inline_reply_markup<'a>(
        &'a self,
        inline_message_id: impl Into<InlineMessageId<'a>>,
        reply_markup: impl Into<Ref<'a, inline::Keyboard<'a>>>,
    ) -> methods::EditInlineReplyMarkup<'a, C> {
        methods::EditInlineReplyMarkup::new(
            &self.client,
            self.token.clone(),
            inline_message_id,
            reply_markup,
        )
    }

    /// Constructs a new `EditInlineText` inferring your bot's token.
    pub fn edit_inline_text<'a>(
        &'a self,
        inline_message_id: impl Into<InlineMessageId<'a>>,
        text: impl Into<Text<'a>>,
    ) -> methods::EditInlineText<'a, C> {
        methods::EditInlineText::new(
            &self.client,
            self.token.clone(),
            inline_message_id,
            text,
        )
    }

    /// Constructs a new `EditMessageCaption` inferring your bot's token.
    pub fn edit_message_caption<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
        caption: impl Into<Text<'a>>,
    ) -> methods::EditMessageCaption<'a, C> {
        methods::EditMessageCaption::new(
            &self.client,
            self.token.clone(),
            chat_id,
            message_id,
            caption,
        )
    }

    /// Constructs a new `EditMessageLocation` inferring your bot's token.
    pub fn edit_message_location<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
        position: (f64, f64),
    ) -> methods::EditMessageLocation<'a, C> {
        methods::EditMessageLocation::new(
            &self.client,
            self.token.clone(),
            chat_id,
            message_id,
            position,
        )
    }

    /// Constructs a new `EditMessageMedia` inferring your bot's token.
    pub fn edit_message_media<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
        media: impl Into<Ref<'a, EditableMedia<'a>>>,
    ) -> methods::EditMessageMedia<'a, C> {
        methods::EditMessageMedia::new(
            &self.client,
            self.token.clone(),
            chat_id,
            message_id,
            media,
        )
    }

    /// Constructs a new `EditMessageReplyMarkup` inferring your bot's token.
    pub fn edit_message_reply_markup<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
        reply_markup: impl Into<Ref<'a, inline::Keyboard<'a>>>,
    ) -> methods::EditMessageReplyMarkup<'a, C> {
        methods::EditMessageReplyMarkup::new(
            &self.client,
            self.token.clone(),
            chat_id,
            message_id,
            reply_markup,
        )
    }

    /// Constructs a new `EditMessageText` inferring your bot's token.
    pub fn edit_message_text<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
        text: impl Into<Text<'a>>,
    ) -> methods::EditMessageText<'a, C> {
        methods::EditMessageText::new(
            &self.client,
            self.token.clone(),
            chat_id,
            message_id,
            text,
        )
    }

    /// Constructs a new `ExportChatInviteLink` inferring your bot's token.
    pub fn export_chat_invite_link<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
    ) -> methods::ExportChatInviteLink<'a, C> {
        methods::ExportChatInviteLink::new(
            &self.client,
            self.token.clone(),
            chat_id,
        )
    }

    /// Constructs a new `ForwardMessage` inferring your bot's token.
    pub fn forward_message<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        from_chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
    ) -> methods::ForwardMessage<'a, C> {
        methods::ForwardMessage::new(
            &self.client,
            self.token.clone(),
            chat_id,
            from_chat_id,
            message_id,
        )
    }

    /// Constructs a new `GetChat` inferring your bot's token.
    pub fn get_chat<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
    ) -> methods::GetChat<'a, C> {
        methods::GetChat::new(&self.client, self.token.clone(), chat_id)
    }

    /// Constructs a new `GetFile` inferring your bot's token.
    pub fn get_file<'a>(
        &'a self,
        file_id: impl AsFileId<'a>,
    ) -> methods::GetFile<'a, C> {
        methods::GetFile::new(&self.client, self.token.clone(), file_id)
    }

    /// Constructs a new `GetInlineGameHighScores` inferring your bot's token.
    pub fn get_inline_game_high_scores<'a>(
        &'a self,
        inline_message_id: impl Into<InlineMessageId<'a>>,
        user_id: user::Id,
    ) -> methods::GetInlineGameHighScores<'a, C> {
        methods::GetInlineGameHighScores::new(
            &self.client,
            self.token.clone(),
            inline_message_id,
            user_id,
        )
    }

    /// Constructs a new `GetChatAdministrators` inferring your bot's token.
    pub fn get_chat_administrators<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
    ) -> methods::GetChatAdministrators<'a, C> {
        methods::GetChatAdministrators::new(
            &self.client,
            self.token.clone(),
            chat_id,
        )
    }

    /// Constructs a new `GetChatMember` inferring your bot's token.
    pub fn get_chat_member<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        user_id: user::Id,
    ) -> methods::GetChatMember<'a, C> {
        methods::GetChatMember::new(
            &self.client,
            self.token.clone(),
            chat_id,
            user_id,
        )
    }

    /// Constructs a new `GetChatMembersCount` inferring your bot's token.
    pub fn get_chat_members_count<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
    ) -> methods::GetChatMembersCount<'a, C> {
        methods::GetChatMembersCount::new(
            &self.client,
            self.token.clone(),
            chat_id,
        )
    }

    /// Constructs a new `GetMessageGameHighScores` inferring your bot's token.
    pub fn get_message_game_high_scores<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
        user_id: user::Id,
    ) -> methods::GetMessageGameHighScores<'a, C> {
        methods::GetMessageGameHighScores::new(
            &self.client,
            self.token.clone(),
            chat_id,
            message_id,
            user_id,
        )
    }

    /// Constructs a new `GetMe` inferring your bot's token.
    pub fn get_me(&self) -> methods::GetMe<'_, C> {
        methods::GetMe::new(&self.client, self.token.clone())
    }

    /// Constructs a new `GetStickerSet` inferring your bot's token.
    pub fn get_sticker_set<'a>(
        &'a self,
        name: impl Into<value::String<'a>>,
    ) -> methods::GetStickerSet<'a, C> {
        methods::GetStickerSet::new(&self.client, self.token.clone(), name)
    }

    pub(crate) fn get_updates<'a>(
        &'a self,
        offset: Option<isize>,
        limit: Option<u8>,
        timeout: Option<u64>,
        allowed_updates: Option<&'a [Updates]>,
    ) -> methods::GetUpdates<'a, C> {
        methods::GetUpdates::new(
            &self.client,
            self.token.clone(),
            offset,
            limit,
            timeout,
            allowed_updates,
        )
    }

    /// Constructs a new `GetUserProfilePhotos` inferring your bot's token.
    pub fn get_user_profile_photos(
        &self,
        user_id: user::Id,
    ) -> methods::GetUserProfilePhotos<'_, C> {
        methods::GetUserProfilePhotos::new(
            &self.client,
            self.token.clone(),
            user_id,
        )
    }

    /// Constructs a new `GetWebhookInfo` inferring your bot's token.
    pub fn get_webhook_info(&self) -> methods::GetWebhookInfo<'_, C> {
        methods::GetWebhookInfo::new(&self.client, self.token.clone())
    }

    /// Constructs a new `KickChatMember` inferring your bot's token.
    pub fn kick_chat_member<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        user_id: user::Id,
    ) -> methods::KickChatMember<'a, C> {
        methods::KickChatMember::new(
            &self.client,
            self.token.clone(),
            chat_id,
            user_id,
        )
    }

    /// Constructs a new `LeaveChat` inferring your bot's token.
    pub fn leave_chat<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
    ) -> methods::LeaveChat<'a, C> {
        methods::LeaveChat::new(&self.client, self.token.clone(), chat_id)
    }

    /// Constructs a new `PinChatMessage` inferring your bot's token.
    pub fn pin_chat_message<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
    ) -> methods::PinChatMessage<'a, C> {
        methods::PinChatMessage::new(
            &self.client,
            self.token.clone(),
            chat_id,
            message_id,
        )
    }

    /// Constructs a new `PromoteChatMember` inferring your bot's token.
    pub fn promote_chat_member<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        user_id: user::Id,
    ) -> methods::PromoteChatMember<'a, C> {
        methods::PromoteChatMember::new(
            &self.client,
            self.token.clone(),
            chat_id,
            user_id,
        )
    }

    /// Constructs a new `RestrictChatMember` inferring your bot's token.
    pub fn restrict_chat_member<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        user_id: user::Id,
    ) -> methods::RestrictChatMember<'a, C> {
        methods::RestrictChatMember::new(
            &self.client,
            self.token.clone(),
            chat_id,
            user_id,
        )
    }

    /// Constructs a new `SendAnimation` inferring your bot's token.
    pub fn send_animation<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        animation: impl Into<Ref<'a, Animation<'a>>>,
    ) -> methods::SendAnimation<'a, C> {
        methods::SendAnimation::new(
            &self.client,
            self.token.clone(),
            chat_id,
            animation,
        )
    }

    /// Constructs a new `SendAudio` inferring your bot's token.
    pub fn send_audio<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        audio: impl Into<Ref<'a, Audio<'a>>>,
    ) -> methods::SendAudio<'a, C> {
        methods::SendAudio::new(
            &self.client,
            self.token.clone(),
            chat_id,
            audio,
        )
    }

    /// Constructs a new `SendChatAction` inferring your bot's token.
    pub fn send_chat_action<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        action: chat::Action,
    ) -> methods::SendChatAction<'a, C> {
        methods::SendChatAction::new(
            &self.client,
            self.token.clone(),
            chat_id,
            action,
        )
    }

    /// Constructs a new `SendContact` inferring your bot's token.
    pub fn send_contact<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        phone_number: impl Into<value::String<'a>>,
        first_name: impl Into<value::String<'a>>,
    ) -> methods::SendContact<'a, C> {
        methods::SendContact::new(
            &self.client,
            self.token.clone(),
            chat_id,
            phone_number,
            first_name,
        )
    }

    /// Constructs a new `SendGame` inferring your bot's token.
    pub fn send_game<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        game_short_name: impl Into<value::String<'a>>,
    ) -> methods::SendGame<'a, C> {
        methods::SendGame::new(
            &self.client,
            self.token.clone(),
            chat_id,
            game_short_name,
        )
    }

    /// Constructs a new `SendDocument` inferring your bot's token.
    pub fn send_document<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        document: impl Into<Ref<'a, Document<'a>>>,
    ) -> methods::SendDocument<'a, C> {
        methods::SendDocument::new(
            &self.client,
            self.token.clone(),
            chat_id,
            document,
        )
    }

    /// Constructs a new` SendInvoice` inerring your bot's token.
    #[allow(clippy::too_many_arguments)]
    pub fn send_invoice<'a>(
        &'a self,
        chat_id: impl Into<chat::Id>,
        title: impl Into<value::String<'a>>,
        description: impl Into<value::String<'a>>,
        payload: impl Into<value::String<'a>>,
        provider_token: impl Into<value::String<'a>>,
        start_parameter: impl Into<value::String<'a>>,
        currency: impl Into<value::String<'a>>,
        prices: impl Into<Seq<'a, Ref<'a, LabeledPrice<'a>>>>,
    ) -> methods::SendInvoice<'a, C> {
        methods::SendInvoice::new(
            &self.client,
            self.token.clone(),
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

    /// Constructs a new `SendLocation` inferring your bot's token.
    pub fn send_location<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        position: (f64, f64),
    ) -> methods::SendLocation<'a, C> {
        methods::SendLocation::new(
            &self.client,
            self.token.clone(),
            chat_id,
            position,
        )
    }

    /// Constructs a new `SendMediaGroup` inferring your bot's token.
    pub fn send_media_group<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        media: impl Into<Seq<'a, Ref<'a, GroupMedia<'a>>>>,
    ) -> methods::SendMediaGroup<'a, C> {
        methods::SendMediaGroup::new(
            &self.client,
            self.token.clone(),
            chat_id,
            media,
        )
    }

    /// Constructs a new `SendMessage` inferring your bot's token.
    pub fn send_message<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        text: impl Into<Text<'a>>,
    ) -> methods::SendMessage<'a, C> {
        methods::SendMessage::new(
            &self.client,
            self.token.clone(),
            chat_id,
            text,
        )
    }

    /// Constructs a new `SendPhoto` inferring your bot's token.
    pub fn send_photo<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        photo: impl Into<Ref<'a, Photo<'a>>>,
    ) -> methods::SendPhoto<'a, C> {
        methods::SendPhoto::new(
            &self.client,
            self.token.clone(),
            chat_id,
            photo,
        )
    }

    /// Constructs a new `SendPoll` inferring your bot's token.
    pub fn send_poll<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        question: impl Into<value::String<'a>>,
        options: impl Into<Seq<'a, value::String<'a>>>,
    ) -> methods::SendPoll<'a, C> {
        methods::SendPoll::new(
            &self.client,
            self.token.clone(),
            chat_id,
            question,
            options,
        )
    }

    /// Constructs a new `SendSticker` inferring your bot's token.
    pub fn send_sticker<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        sticker: impl Into<Ref<'a, Sticker<'a>>>,
    ) -> methods::SendSticker<'a, C> {
        methods::SendSticker::new(
            &self.client,
            self.token.clone(),
            chat_id,
            sticker,
        )
    }

    /// Constructs a new `SendVenue` inferring your bot's token.
    pub fn send_venue<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        position: (f64, f64),
        title: impl Into<value::String<'a>>,
        address: impl Into<value::String<'a>>,
    ) -> methods::SendVenue<'a, C> {
        methods::SendVenue::new(
            &self.client,
            self.token.clone(),
            chat_id,
            position,
            title,
            address,
        )
    }

    /// Constructs a new `SendVideoNote` inferring your bot's token.
    pub fn send_video_note<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        video_note: impl Into<Ref<'a, VideoNote<'a>>>,
    ) -> methods::SendVideoNote<'a, C> {
        methods::SendVideoNote::new(
            &self.client,
            self.token.clone(),
            chat_id,
            video_note,
        )
    }

    /// Constructs a new `SendVideo` inferring your bot's token.
    pub fn send_video<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        video: impl Into<Ref<'a, Video<'a>>>,
    ) -> methods::SendVideo<'a, C> {
        methods::SendVideo::new(
            &self.client,
            self.token.clone(),
            chat_id,
            video,
        )
    }

    /// Constructs a new `SendVoice` inferring your bot's token.
    pub fn send_voice<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        voice: impl Into<Ref<'a, Voice<'a>>>,
    ) -> methods::SendVoice<'a, C> {
        methods::SendVoice::new(
            &self.client,
            self.token.clone(),
            chat_id,
            voice,
        )
    }

    /// Constructs a new `SetChatDescription` inferring your bot's token.
    pub fn set_chat_description<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        description: impl Into<value::String<'a>>,
    ) -> methods::SetChatDescription<'a, C> {
        methods::SetChatDescription::new(
            &self.client,
            self.token.clone(),
            chat_id,
            description,
        )
    }

    /// Constructs a new `SetChatPhoto` inferring your bot's token.
    pub fn set_chat_photo<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        photo: impl Into<Ref<'a, ChatPhoto<'a>>>,
    ) -> methods::SetChatPhoto<'a, C> {
        methods::SetChatPhoto::new(
            &self.client,
            self.token.clone(),
            chat_id,
            photo,
        )
    }

    /// Constructs a new `SetChatStickerSet` inferring your bot's token.
    pub fn set_chat_sticker_set<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        sticker_set_name: impl Into<value::String<'a>>,
    ) -> methods::SetChatStickerSet<'a, C> {
        methods::SetChatStickerSet::new(
            &self.client,
            self.token.clone(),
            chat_id,
            sticker_set_name,
        )
    }

    /// Constructs a new `SetChatTitle` inferring your bot's token.
    pub fn set_chat_title<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        title: impl Into<value::String<'a>>,
    ) -> methods::SetChatTitle<'a, C> {
        methods::SetChatTitle::new(
            &self.client,
            self.token.clone(),
            chat_id,
            title,
        )
    }

    /// Constructs a new `SetInlineGameScore` inferring your bot's token.
    pub fn set_inline_game_score<'a>(
        &'a self,
        inline_message_id: impl Into<InlineMessageId<'a>>,
        user_id: user::Id,
        score: u32,
    ) -> methods::SetInlineGameScore<'a, C> {
        methods::SetInlineGameScore::new(
            &self.client,
            self.token.clone(),
            inline_message_id,
            user_id,
            score,
        )
    }

    /// Constructs a new `SetMessageGameScore` inferring your bot's token.
    pub fn set_message_game_score<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
        user_id: user::Id,
        score: u32,
    ) -> methods::SetMessageGameScore<'a, C> {
        methods::SetMessageGameScore::new(
            &self.client,
            self.token.clone(),
            chat_id,
            message_id,
            user_id,
            score,
        )
    }

    /// Constructs a new `SetPassportDataErrors` inferring your bot's token.
    pub fn set_passport_data_errors<'a>(
        &'a self,
        user_id: user::Id,
        errors: impl Into<Seq<'a, Ref<'a, passport::element::Error<'a>>>>,
    ) -> methods::SetPassportDataErrors<'a, C> {
        methods::SetPassportDataErrors::new(
            &self.client,
            self.token.clone(),
            user_id,
            errors,
        )
    }

    /// Constructs a new `SetStickerPositionInSet` inferring your bot's token.
    pub fn set_sticker_position_in_set<'a>(
        &'a self,
        sticker: impl Into<value::String<'a>>,
        position: u32,
    ) -> methods::SetStickerPositionInSet<'a, C> {
        methods::SetStickerPositionInSet::new(
            &self.client,
            self.token.clone(),
            sticker,
            position,
        )
    }

    pub(crate) fn set_webhook<'a>(
        &'a self,
        url: value::String<'a>,
        certificate: Option<value::String<'a>>,
        max_connections: Option<u8>,
        allowed_updates: Option<Seq<'a, Updates>>,
    ) -> methods::SetWebhook<'a, C> {
        methods::SetWebhook::new(
            &self.client,
            self.token.clone(),
            url,
            certificate,
            max_connections,
            allowed_updates,
        )
    }

    /// Constructs a new `StopInlineLocation` inferring your bot's token.
    pub fn stop_inline_location<'a>(
        &'a self,
        inline_message_id: impl Into<InlineMessageId<'a>>,
    ) -> methods::StopInlineLocation<'a, C> {
        methods::StopInlineLocation::new(
            &self.client,
            self.token.clone(),
            inline_message_id,
        )
    }

    /// Constructs a new `StopMessageLocation` inferring your bot's token.
    pub fn stop_message_location<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
    ) -> methods::StopMessageLocation<'a, C> {
        methods::StopMessageLocation::new(
            &self.client,
            self.token.clone(),
            chat_id,
            message_id,
        )
    }

    /// Constructs a new `StopPoll` inferring your bot's token.
    pub fn stop_poll<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
    ) -> methods::StopPoll<'a, C> {
        methods::StopPoll::new(
            &self.client,
            self.token.clone(),
            chat_id,
            message_id,
        )
    }

    /// Constructs a new `UnbanChatMember` inferring your bot's token.
    pub fn unban_chat_member<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
        user_id: user::Id,
    ) -> methods::UnbanChatMember<'a, C> {
        methods::UnbanChatMember::new(
            &self.client,
            self.token.clone(),
            chat_id,
            user_id,
        )
    }

    /// Constructs a new `UnpinChatMessage` inferring your bot's token.
    pub fn unpin_chat_message<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
    ) -> methods::UnpinChatMessage<'a, C> {
        methods::UnpinChatMessage::new(
            &self.client,
            self.token.clone(),
            chat_id,
        )
    }

    /// Constructs a new `UploadStickerFile` inferring your bot's token.
    pub fn upload_sticker_file<'a>(
        &'a self,
        user_id: user::Id,
        png_sticker: impl Into<Bytes<'a>>,
    ) -> methods::UploadStickerFile<'a, C> {
        methods::UploadStickerFile::new(
            &self.client,
            self.token.clone(),
            user_id,
            png_sticker,
        )
    }
}

impl<C> Bot<C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    /// Constructs a `Bot` with a custom connector.
    pub fn with_connector(token: Token, connector: C) -> Self {
        Self {
            token,
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
    pub fn download_file<'a>(
        &self,
        file: impl Into<Ref<'a, File>>,
    ) -> impl Future<Item = Vec<u8>, Error = errors::Download> {
        download_file(&self.client, &self.token, file.into().as_ref())
    }
}

/// Constructs a new [`Bot`], extracting the token from the environment at
/// _compile time_.
///
/// You can provide a connector as the second parameter,
/// e.g. `bot!("...", connector)`.
///
/// If you need to extract the token at _runtime_, use [`Bot::from_env`].
///
/// [`Bot`]: ./struct.Bot.html
/// [`Bot::from_env`]: ./struct.Bot.html#method.from_env
///
/// # Example
///
/// ```no_run
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
    ($var:literal, $connector:expr) => {{
        let token = env!($var).to_string();
        $crate::Bot::with_connector($crate::Token::new(token), $connector)
    }};
    ($var:literal, $connector:expr,) => {
        $crate::bot!($var, $connector)
    };
    () => {
        compile_error!(
            "the macro must be invoked as `bot!(\"<VAR_NAME>\")` or \
             `bot!(\"<VAR_NAME>\", connector)`"
        )
    };
    ($($x:tt)+) => {
        $crate::bot!()
    };
}

fn extract_token(env_var: &'static str) -> Token {
    Token::new(std::env::var(env_var).unwrap_or_else(|_| {
        panic!("\n[tbot] Bot's token in {} was not specified\n", env_var)
    }))
}

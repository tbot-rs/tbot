//! The event loop for handling bot updates.

use crate::{
    contexts,
    errors::{self, MethodCall},
    state::StatefulEventLoop,
    types::{
        self, callback,
        callback::Query,
        message::{
            self,
            text::{Entity, EntityKind, Text},
            Message,
        },
        update, BotCommand,
    },
    Bot,
};
use std::{collections::HashMap, future::Future, sync::Arc};
use tracing::{error, instrument, trace, warn};

#[macro_use]
mod handlers_macros;

mod polling;
pub mod webhook;

pub use {polling::Polling, webhook::Webhook};

type Handlers<T> = Vec<Box<T>>;
type Map<T> = HashMap<String, Handlers<T>>;

// Wish trait alises came out soon
type Handler<T> = dyn Fn(Arc<T>) + Send + Sync;

type AnimationHandler = Handler<contexts::Animation>;
type AudioHandler = Handler<contexts::Audio>;
type ChosenInlineHandler = Handler<contexts::ChosenInline>;
type CommandHandler = Handler<contexts::Command>;
type ConnectedWebsiteHandler = Handler<contexts::ConnectedWebsite>;
type ContactHandler = Handler<contexts::Contact>;
type CreatedGroupHandler = Handler<contexts::CreatedGroup>;
type MessageDataCallbackHandler = Handler<contexts::MessageDataCallback>;
type InlineDataCallbackHandler = Handler<contexts::InlineDataCallback>;
type DeletedChatPhotoHandler = Handler<contexts::DeletedChatPhoto>;
type DiceHandler = Handler<contexts::Dice>;
type DocumentHandler = Handler<contexts::Document>;
type EditedAnimationHandler = Handler<contexts::EditedAnimation>;
type EditedAudioHandler = Handler<contexts::EditedAudio>;
type EditedCommandHandler = Handler<contexts::EditedCommand>;
type EditedDocumentHandler = Handler<contexts::EditedDocument>;
type EditedLocationHandler = Handler<contexts::EditedLocation>;
type EditedPhotoHandler = Handler<contexts::EditedPhoto>;
type EditedTextHandler = Handler<contexts::EditedText>;
type EditedVideoHandler = Handler<contexts::EditedVideo>;
type MessageGameCallbackHandler = Handler<contexts::MessageGameCallback>;
type InlineGameCallbackHandler = Handler<contexts::InlineGameCallback>;
type GameHandler = Handler<contexts::Game>;
type InlineHandler = Handler<contexts::Inline>;
type InvoiceHandler = Handler<contexts::Invoice>;
type LeftMemberHandler = Handler<contexts::LeftMember>;
type LocationHandler = Handler<contexts::Location>;
type MigrationHandler = Handler<contexts::Migration>;
type NewChatPhotoHandler = Handler<contexts::NewChatPhoto>;
type NewChatTitleHandler = Handler<contexts::NewChatTitle>;
type NewMembersHandler = Handler<contexts::NewMembers>;
type PassportHandler = Handler<contexts::Passport>;
type PaymentHandler = Handler<contexts::Payment>;
type PhotoHandler = Handler<contexts::Photo>;
type PinnedMessageHandler = Handler<contexts::PinnedMessage>;
type PollHandler = Handler<contexts::Poll>;
type PollAnswerHandler = Handler<contexts::PollAnswer>;
type PreCheckoutHandler = Handler<contexts::PreCheckout>;
type ShippingHandler = Handler<contexts::Shipping>;
type StickerHandler = Handler<contexts::Sticker>;
type TextHandler = Handler<contexts::Text>;
type UnhandledHandler = Handler<contexts::Unhandled>;
type UpdatedPollHandler = Handler<contexts::UpdatedPoll>;
type UpdateHandler = Handler<contexts::Update>;
type VenueHandler = Handler<contexts::Venue>;
type VideoHandler = Handler<contexts::Video>;
type VideoNoteHandler = Handler<contexts::VideoNote>;
type VoiceHandler = Handler<contexts::Voice>;

/// Provides an event loop for handling Telegram updates.
///
/// With `EventLoop`, you can configure handlers and start listening to updates
/// via either [polling] or [webhook].
///
/// ```no_run
/// let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();
///
/// bot.text(|_| async { println!("Got a text message") });
///
/// bot.polling().start();
/// ```
///
/// `tbot` has many update handlers, such as [`text`] you have seen
/// in the example. You can find all of them below on this page.
///
/// [polling]: #method.polling
/// [webhook]: #method.webhook
/// [`text`]: #method.text
#[must_use]
pub struct EventLoop {
    bot: Bot,
    username: Option<String>,

    command_handlers: Map<CommandHandler>,
    command_description: HashMap<String, String>,
    edited_command_handlers: Map<EditedCommandHandler>,
    after_update_handlers: Handlers<UpdateHandler>,
    animation_handlers: Handlers<AnimationHandler>,
    audio_handlers: Handlers<AudioHandler>,
    before_update_handlers: Handlers<UpdateHandler>,
    chosen_inline_handlers: Handlers<ChosenInlineHandler>,
    contact_handlers: Handlers<ContactHandler>,
    connected_website_handlers: Handlers<ConnectedWebsiteHandler>,
    created_group_handlers: Handlers<CreatedGroupHandler>,
    deleted_chat_photo_handlers: Handlers<DeletedChatPhotoHandler>,
    dice_handlers: Handlers<DiceHandler>,
    document_handlers: Handlers<DocumentHandler>,
    edited_animation_handlers: Handlers<EditedAnimationHandler>,
    edited_audio_handlers: Handlers<EditedAudioHandler>,
    edited_document_handlers: Handlers<EditedDocumentHandler>,
    edited_location_handlers: Handlers<EditedLocationHandler>,
    edited_photo_handlers: Handlers<EditedPhotoHandler>,
    edited_text_handlers: Handlers<EditedTextHandler>,
    edited_video_handlers: Handlers<EditedVideoHandler>,
    game_handlers: Handlers<GameHandler>,
    inline_handlers: Handlers<InlineHandler>,
    inline_data_callback_handlers: Handlers<InlineDataCallbackHandler>,
    inline_game_callback_handlers: Handlers<InlineGameCallbackHandler>,
    invoice_handlers: Handlers<InvoiceHandler>,
    left_member_handlers: Handlers<LeftMemberHandler>,
    location_handlers: Handlers<LocationHandler>,
    message_data_callback_handlers: Handlers<MessageDataCallbackHandler>,
    message_game_callback_handlers: Handlers<MessageGameCallbackHandler>,
    migration_handlers: Handlers<MigrationHandler>,
    new_chat_photo_handlers: Handlers<NewChatPhotoHandler>,
    new_chat_title_handlers: Handlers<NewChatTitleHandler>,
    new_members_handlers: Handlers<NewMembersHandler>,
    passport_handlers: Handlers<PassportHandler>,
    payment_handlers: Handlers<PaymentHandler>,
    photo_handlers: Handlers<PhotoHandler>,
    pinned_message_handlers: Handlers<PinnedMessageHandler>,
    poll_handlers: Handlers<PollHandler>,
    poll_answer_handlers: Handlers<PollAnswerHandler>,
    pre_checkout_handlers: Handlers<PreCheckoutHandler>,
    shipping_handlers: Handlers<ShippingHandler>,
    sticker_handlers: Handlers<StickerHandler>,
    text_handlers: Handlers<TextHandler>,
    unhandled_handlers: Handlers<UnhandledHandler>,
    updated_poll_handlers: Handlers<UpdatedPollHandler>,
    venue_handlers: Handlers<VenueHandler>,
    video_handlers: Handlers<VideoHandler>,
    video_note_handlers: Handlers<VideoNoteHandler>,
    voice_handlers: Handlers<VoiceHandler>,
}

impl EventLoop {
    pub(crate) fn new(bot: Bot) -> Self {
        Self {
            bot,
            username: None,
            command_handlers: HashMap::new(),
            command_description: HashMap::new(),
            edited_command_handlers: HashMap::new(),
            after_update_handlers: Vec::new(),
            animation_handlers: Vec::new(),
            audio_handlers: Vec::new(),
            before_update_handlers: Vec::new(),
            chosen_inline_handlers: Vec::new(),
            contact_handlers: Vec::new(),
            connected_website_handlers: Vec::new(),
            created_group_handlers: Vec::new(),
            deleted_chat_photo_handlers: Vec::new(),
            dice_handlers: Vec::new(),
            document_handlers: Vec::new(),
            edited_animation_handlers: Vec::new(),
            edited_audio_handlers: Vec::new(),
            edited_document_handlers: Vec::new(),
            edited_location_handlers: Vec::new(),
            edited_photo_handlers: Vec::new(),
            edited_text_handlers: Vec::new(),
            edited_video_handlers: Vec::new(),
            game_handlers: Vec::new(),
            inline_handlers: Vec::new(),
            inline_data_callback_handlers: Vec::new(),
            inline_game_callback_handlers: Vec::new(),
            invoice_handlers: Vec::new(),
            left_member_handlers: Vec::new(),
            location_handlers: Vec::new(),
            message_data_callback_handlers: Vec::new(),
            message_game_callback_handlers: Vec::new(),
            migration_handlers: Vec::new(),
            new_chat_photo_handlers: Vec::new(),
            new_chat_title_handlers: Vec::new(),
            new_members_handlers: Vec::new(),
            passport_handlers: Vec::new(),
            payment_handlers: Vec::new(),
            photo_handlers: Vec::new(),
            pinned_message_handlers: Vec::new(),
            poll_handlers: Vec::new(),
            poll_answer_handlers: Vec::new(),
            pre_checkout_handlers: Vec::new(),
            shipping_handlers: Vec::new(),
            sticker_handlers: Vec::new(),
            text_handlers: Vec::new(),
            unhandled_handlers: Vec::new(),
            updated_poll_handlers: Vec::new(),
            venue_handlers: Vec::new(),
            video_handlers: Vec::new(),
            video_note_handlers: Vec::new(),
            voice_handlers: Vec::new(),
        }
    }

    /// Turns this event loop into a stateful one. Handlers added on this event
    /// loop are kept.
    pub fn into_stateful<S>(self, state: S) -> StatefulEventLoop<S>
    where
        S: Send + Sync + 'static,
    {
        StatefulEventLoop::new(self, state)
    }

    /// Sets the bot's username.
    ///
    /// The username is used when checking if a command such as
    /// `/command@username` was directed to the bot.
    pub fn username(&mut self, username: String) {
        self.username = Some(username);
    }

    /// Starts polling configuration.
    pub fn polling(self) -> Polling {
        Polling::new(self)
    }

    /// Starts webhook configuration.
    ///
    /// See our [wiki] to learn how to use webhook with `tbot`.
    ///
    /// [wiki]: https://gitlab.com/SnejUgal/tbot/wikis/How-to/How-to-use-webhooks
    pub fn webhook(self, url: &str, port: u16) -> Webhook<'_> {
        Webhook::new(self, url, port)
    }

    /// Adds a new handler for a command.
    ///
    /// Note that commands such as `/command@username` will be completely
    /// ignored unless you configure the event loop with your bot's username
    /// with either [`username`] or [`fetch_username`].
    ///
    /// [`username`]: #method.username
    /// [`fetch_username`]: #method.fetch_username
    pub fn command<H, F>(&mut self, command: &'static str, handler: H)
    where
        H: (Fn(Arc<contexts::Command>) -> F) + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.command_handlers
            .entry(command.to_string())
            .or_insert_with(Vec::new)
            .push(Box::new(move |context| {
                tokio::spawn(handler(context));
            }));
    }

    /// Adds a new handler for a command and sets its description.
    ///
    /// Note that commands such as `/command@username` will be completely
    /// ignored unless you configure the event loop with your bot's username
    /// with either [`username`] or [`fetch_username`].
    ///
    /// [`username`]: #method.username
    /// [`fetch_username`]: #method.fetch_username
    pub fn command_with_description<H, F>(
        &mut self,
        command: &'static str,
        description: &'static str,
        handler: H,
    ) where
        H: (Fn(Arc<contexts::Command>) -> F) + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.command_description
            .insert(command.to_string(), description.to_string());
        self.command(command, handler);
    }

    /// Adds a new handler for a sequence of commands.
    ///
    /// Note that commands such as `/command@username` will be completely
    /// ignored unless you configure the event loop with your bot's username
    /// with either [`username`] or [`fetch_username`].
    ///
    /// [`username`]: #method.username
    /// [`fetch_username`]: #method.fetch_username
    pub fn commands<Cm, H, F>(&mut self, commands: Cm, handler: H)
    where
        Cm: IntoIterator<Item = &'static str>,
        F: Future<Output = ()> + Send + 'static,
        H: (Fn(Arc<contexts::Command>) -> F) + Send + Sync + 'static,
    {
        let handler = Arc::new(handler);

        for command in commands {
            let handler = Arc::clone(&handler);
            self.command_handlers
                .entry(command.to_string())
                .or_insert_with(Vec::new)
                .push(Box::new(move |context| {
                    tokio::spawn(handler(context));
                }));
        }
    }

    fn will_handle_command(&self, command: &str) -> bool {
        self.command_handlers.contains_key(command)
    }

    fn run_command_handlers(
        &self,
        command: &str,
        context: &Arc<contexts::Command>,
    ) {
        if let Some(handlers) = self.command_handlers.get(command) {
            for handler in handlers {
                handler(context.clone());
            }
        }
    }

    /// Adds a new handler for the `/start` command.
    pub fn start<H, F>(&mut self, handler: H)
    where
        H: (Fn(Arc<contexts::Command>) -> F) + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.command("start", handler);
    }

    /// Adds a new handler for the `/start` command and sets its description.
    pub fn start_with_description<H, F>(
        &mut self,
        description: &'static str,
        handler: H,
    ) where
        H: (Fn(Arc<contexts::Command>) -> F) + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.command_with_description("start", description, handler);
    }

    /// Adds a new handler for the `/settings` command.
    pub fn settings<H, F>(&mut self, handler: H)
    where
        H: (Fn(Arc<contexts::Command>) -> F) + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.command("settings", handler);
    }

    /// Adds a new handler for the `/settings` command and sets its description.
    pub fn settings_with_description<H, F>(
        &mut self,
        description: &'static str,
        handler: H,
    ) where
        H: (Fn(Arc<contexts::Command>) -> F) + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.command_with_description("settings", description, handler);
    }

    /// Adds a new handler for the `/help` command.
    pub fn help<H, F>(&mut self, handler: H)
    where
        H: (Fn(Arc<contexts::Command>) -> F) + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.command("help", handler);
    }

    /// Adds a new handler for the `/help` command and sets its description.
    pub fn help_with_description<H, F>(
        &mut self,
        description: &'static str,
        handler: H,
    ) where
        H: (Fn(Arc<contexts::Command>) -> F) + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.command_with_description("help", description, handler);
    }

    /// Adds a new handler for an edited command.
    pub fn edited_command<H, F>(&mut self, command: &'static str, handler: H)
    where
        H: (Fn(Arc<contexts::EditedCommand>) -> F) + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.edited_command_handlers
            .entry(command.to_string())
            .or_insert_with(Vec::new)
            .push(Box::new(move |context| {
                tokio::spawn(handler(context));
            }));
    }

    /// Adds a new handler for an edited command from sequence of commands.
    pub fn edited_commands<Cm, H, F>(&mut self, commands: Cm, handler: H)
    where
        Cm: IntoIterator<Item = &'static str>,
        F: Future<Output = ()> + Send + 'static,
        H: (Fn(Arc<contexts::EditedCommand>) -> F) + Send + Sync + 'static,
    {
        let handler = Arc::new(handler);

        for command in commands {
            let handler = Arc::clone(&handler);
            self.edited_command_handlers
                .entry(command.to_string())
                .or_insert_with(Vec::new)
                .push(Box::new(move |context| {
                    tokio::spawn(handler(context));
                }));
        }
    }

    fn will_handle_edited_command(&self, command: &str) -> bool {
        self.edited_command_handlers.contains_key(command)
    }

    fn run_edited_command_handlers(
        &self,
        command: &str,
        context: &Arc<contexts::EditedCommand>,
    ) {
        if let Some(handlers) = self.edited_command_handlers.get(command) {
            for handler in handlers {
                handler(context.clone());
            }
        }
    }

    handler! {
        contexts::Update,
        /// Adds a new handler which is run after handling an update.
        after_update,
    }

    handler! {
        contexts::Animation,
        /// Adds a new handler for animations.
        animation,
    }

    handler! {
        contexts::Audio,
        /// Adds a new handler for audio.
        audio,
    }

    handler! {
        contexts::Update,
        /// Adds a new handler which is run before handling an update.
        before_update,
    }

    handler! {
        contexts::ChosenInline,
        /// Adds a new handler for chosen inline results.
        chosen_inline,
    }

    handler! {
        contexts::Contact,
        /// Adds a new handler for contacts.
        contact,
    }

    handler! {
        contexts::ConnectedWebsite,
        /// Adds a new handler for connected websites.
        connected_website,
    }

    handler! {
        contexts::CreatedGroup,
        /// Adds a new handler for created groups.
        created_group,
    }

    handler! {
        contexts::MessageDataCallback,
        /// Adds a new handler for data callbacks from chat messages.
        message_data_callback,
    }

    handler! {
        contexts::InlineDataCallback,
        /// Adds a new handler for data callbacks from inline messages.
        inline_data_callback,
    }

    handler! {
        contexts::DeletedChatPhoto,
        /// Adds a new handler for deleted chat photos.
        deleted_chat_photo,
    }

    handler! {
        contexts::Dice,
        /// Adds a new handler for dice.
        dice,
    }

    handler! {
        contexts::Document,
        /// Adds a new handler for documents.
        document,
    }

    handler! {
        contexts::EditedAnimation,
        /// Adds a new handler for edited animations.
        edited_animation,
    }

    handler! {
        contexts::EditedAudio,
        /// Adds a new handler for edited audio.
        edited_audio,
    }

    handler! {
        contexts::EditedDocument,
        /// Adds a new handler for edited documents.
        edited_document,
    }

    handler! {
        contexts::EditedLocation,
        /// Adds a new handler for edited locations.
        edited_location,
    }

    handler! {
        contexts::EditedPhoto,
        /// Adds a new handler for edited photos.
        edited_photo,
    }

    handler! {
        contexts::EditedText,
        /// Adds a new handler for edited text messages.
        edited_text,
    }

    handler! {
        contexts::EditedVideo,
        /// Adds a new handler for edited videos.
        edited_video,
    }

    handler! {
        contexts::MessageGameCallback,
        /// Adds a new handler for game callbacks from chat messages.
        message_game_callback,
    }

    handler! {
        contexts::InlineGameCallback,
        /// Adds a new handler for game callbacks from inline messages.
        inline_game_callback,
    }

    handler! {
        contexts::Game,
        /// Adds a new handler for game messages.
        game,
    }

    handler! {
        contexts::Inline,
        /// Adds a new handler for inline queries.
        inline,
    }

    handler! {
        contexts::Invoice,
        /// Adds a new handler for invoices.
        invoice,
    }

    handler! {
        contexts::LeftMember,
        /// Adds a new handler for left members.
        left_member,
    }

    handler! {
        contexts::Location,
        /// Adds a new handler for locations.
        location,
    }

    handler! {
        contexts::Migration,
        /// Adds a new handler for migrations.
        migration,
    }

    handler! {
        contexts::NewChatPhoto,
        /// Adds a new handler for new chat photos.
        new_chat_photo,
    }

    handler! {
        contexts::NewChatTitle,
        /// Adds a new handler for new chat titles.
        new_chat_title,
    }

    handler! {
        contexts::NewMembers,
        /// Adds a new handler for new members.
        new_members,
    }

    handler! {
        contexts::Passport,
        /// Adds a new handler for passport data.
        passport,
    }

    handler! {
        contexts::Payment,
        /// Adds a new handler for successful payments.
        payment,
    }

    handler! {
        contexts::Photo,
        /// Adds a new handler for photos.
        photo,
    }

    handler! {
        contexts::PinnedMessage,
        /// Adds a new handler for pinned messages.
        pinned_message,
    }

    handler! {
        contexts::Poll,
        /// Adds a new handler for poll messages.
        poll,
    }

    handler! {
        contexts::PreCheckout,
        /// Adds a new handler for pre-checkout queries.
        pre_checkout,
    }

    handler! {
        contexts::Shipping,
        /// Adds a new handler for shipping queries.
        shipping,
    }

    handler! {
        contexts::Sticker,
        /// Adds a new handler for stickers.
        sticker,
    }

    handler! {
        contexts::Text,
        /// Adds a new handler for text messages.
        text,
    }

    /// Adds a new handler for unhandled updates.
    pub fn unhandled<H, F>(&mut self, handler: H)
    where
        H: (Fn(Arc<contexts::Unhandled>) -> F) + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.unhandled_handlers.push(Box::new(move |context| {
            tokio::spawn(handler(context));
        }))
    }

    fn will_handle_unhandled(&self) -> bool {
        !self.unhandled_handlers.is_empty()
    }

    fn run_unhandled_handlers(&self, update: update::Kind) {
        let context =
            Arc::new(contexts::Unhandled::new(self.bot.clone(), update));

        for handler in &self.unhandled_handlers {
            handler(context.clone());
        }
    }

    handler! {
        contexts::UpdatedPoll,
        /// Adds a new handler for new states of polls.
        updated_poll,
    }

    handler! {
        contexts::PollAnswer,
        /// Adds a new handler for new answers in the poll.
        poll_answer,
    }

    handler! {
        contexts::Venue,
        /// Adds a new handler for venues.
        venue,
    }

    handler! {
        contexts::Video,
        /// Adds a new handler for videos.
        video,
    }

    handler! {
        contexts::VideoNote,
        /// Adds a new handler for video notes.
        video_note,
    }

    handler! {
        contexts::Voice,
        /// Adds a new handler for voice messages.
        voice,
    }

    #[instrument(skip(self, update))]
    fn handle_update(&self, update: types::Update) {
        trace!(?update);

        let update_context =
            Arc::new(contexts::Update::new(self.bot.clone(), update.id));

        self.run_before_update_handlers(update_context.clone());

        match update.kind {
            update::Kind::CallbackQuery(query) => match query {
                Query {
                    kind: callback::Kind::Data(data),
                    origin: callback::Origin::Message(message),
                    id,
                    from,
                    chat_instance,
                } if self.will_handle_message_data_callback() => {
                    let context = contexts::MessageDataCallback::new(
                        self.bot.clone(),
                        id,
                        from,
                        *message,
                        chat_instance,
                        data,
                    );
                    self.run_message_data_callback_handlers(Arc::new(context));
                }
                Query {
                    kind: callback::Kind::Data(data),
                    origin: callback::Origin::Inline(message_id),
                    id,
                    from,
                    chat_instance,
                } if self.will_handle_inline_data_callback() => {
                    let context = contexts::InlineDataCallback::new(
                        self.bot.clone(),
                        id,
                        from,
                        message_id,
                        chat_instance,
                        data,
                    );
                    self.run_inline_data_callback_handlers(Arc::new(context));
                }
                Query {
                    kind: callback::Kind::Game(game),
                    origin: callback::Origin::Message(message),
                    id,
                    from,
                    chat_instance,
                } if self.will_handle_message_game_callback() => {
                    let context = contexts::MessageGameCallback::new(
                        self.bot.clone(),
                        id,
                        from,
                        *message,
                        chat_instance,
                        game,
                    );
                    self.run_message_game_callback_handlers(Arc::new(context));
                }
                Query {
                    kind: callback::Kind::Game(game),
                    origin: callback::Origin::Inline(message_id),
                    id,
                    from,
                    chat_instance,
                } if self.will_handle_inline_game_callback() => {
                    let context = contexts::InlineGameCallback::new(
                        self.bot.clone(),
                        id,
                        from,
                        message_id,
                        chat_instance,
                        game,
                    );
                    self.run_inline_game_callback_handlers(Arc::new(context));
                }
                query if self.will_handle_unhandled() => {
                    let update = update::Kind::CallbackQuery(query);
                    self.run_unhandled_handlers(update);
                }
                Query {
                    kind: callback::Kind::Data(..),
                    origin: callback::Origin::Message(..),
                    ..
                }
                | Query {
                    kind: callback::Kind::Data(..),
                    origin: callback::Origin::Inline(..),
                    ..
                }
                | Query {
                    kind: callback::Kind::Game(..),
                    origin: callback::Origin::Message(..),
                    ..
                }
                | Query {
                    kind: callback::Kind::Game(..),
                    origin: callback::Origin::Inline(..),
                    ..
                } => (),
            },
            update::Kind::ChosenInlineResult(result)
                if self.will_handle_chosen_inline() =>
            {
                let context =
                    contexts::ChosenInline::new(self.bot.clone(), result);
                self.run_chosen_inline_handlers(Arc::new(context));
            }
            update::Kind::EditedMessage(message)
            | update::Kind::EditedChannelPost(message) => {
                self.handle_message_edit_update(message);
            }
            update::Kind::InlineQuery(query) if self.will_handle_inline() => {
                let context = contexts::Inline::new(self.bot.clone(), query);
                self.run_inline_handlers(Arc::new(context));
            }
            update::Kind::Message(message)
            | update::Kind::ChannelPost(message) => {
                self.handle_message_update(message);
            }
            update::Kind::PreCheckoutQuery(query)
                if self.will_handle_pre_checkout() =>
            {
                let context =
                    contexts::PreCheckout::new(self.bot.clone(), query);
                self.run_pre_checkout_handlers(Arc::new(context));
            }
            update::Kind::Poll(poll) if self.will_handle_updated_poll() => {
                let context =
                    contexts::UpdatedPoll::new(self.bot.clone(), poll);
                self.run_updated_poll_handlers(Arc::new(context));
            }
            update::Kind::PollAnswer(answer)
                if self.will_handle_poll_answer() =>
            {
                let context =
                    contexts::PollAnswer::new(self.bot.clone(), answer);
                self.run_poll_answer_handlers(Arc::new(context));
            }
            update::Kind::ShippingQuery(query)
                if self.will_handle_shipping() =>
            {
                let context = contexts::Shipping::new(self.bot.clone(), query);
                self.run_shipping_handlers(Arc::new(context));
            }
            update if self.will_handle_unhandled() => {
                self.run_unhandled_handlers(update);
            }
            update::Kind::ChosenInlineResult(..)
            | update::Kind::InlineQuery(..)
            | update::Kind::Poll(..)
            | update::Kind::PollAnswer(..)
            | update::Kind::PreCheckoutQuery(..)
            | update::Kind::ShippingQuery(..)
            | update::Kind::Unknown => (),
        }

        self.run_after_update_handlers(update_context);
    }

    #[allow(clippy::cognitive_complexity)]
    #[allow(clippy::too_many_lines)] // can't split the huge match
    fn handle_message_update(&self, message: types::Message) {
        let (data, kind) = message.split();

        match kind {
            message::Kind::Animation { animation, caption }
                if self.will_handle_animation() =>
            {
                let context = contexts::Animation::new(
                    self.bot.clone(),
                    data,
                    *animation,
                    caption,
                );
                self.run_animation_handlers(Arc::new(context));
            }
            message::Kind::Audio {
                audio,
                caption,
                media_group_id,
            } if self.will_handle_audio() => {
                let context = contexts::Audio::new(
                    self.bot.clone(),
                    data,
                    *audio,
                    caption,
                    media_group_id,
                );
                self.run_audio_handlers(Arc::new(context));
            }
            message::Kind::ChatPhotoDeleted
                if self.will_handle_deleted_chat_photo() =>
            {
                let context =
                    contexts::DeletedChatPhoto::new(self.bot.clone(), data);
                self.run_deleted_chat_photo_handlers(Arc::new(context));
            }
            message::Kind::ConnectedWebsite(website)
                if self.will_handle_connected_website() =>
            {
                let context = contexts::ConnectedWebsite::new(
                    self.bot.clone(),
                    data,
                    website,
                );
                self.run_connected_website_handlers(Arc::new(context));
            }
            message::Kind::Contact(contact) if self.will_handle_contact() => {
                let context =
                    contexts::Contact::new(self.bot.clone(), data, contact);
                self.run_contact_handlers(Arc::new(context));
            }
            message::Kind::Dice(dice) if self.will_handle_dice() => {
                let context = contexts::Dice::new(self.bot.clone(), data, dice);
                self.run_dice_handlers(Arc::new(context));
            }
            message::Kind::Document {
                document,
                caption,
                media_group_id,
            } if self.will_handle_document() => {
                let context = contexts::Document::new(
                    self.bot.clone(),
                    data,
                    *document,
                    caption,
                    media_group_id,
                );
                self.run_document_handlers(Arc::new(context));
            }
            message::Kind::Game(game) if self.will_handle_game() => {
                let context =
                    contexts::Game::new(self.bot.clone(), data, *game);
                self.run_game_handlers(Arc::new(context));
            }
            message::Kind::GroupCreated if self.will_handle_created_group() => {
                let context =
                    contexts::CreatedGroup::new(self.bot.clone(), data);
                self.run_created_group_handlers(Arc::new(context));
            }
            message::Kind::Invoice(invoice) if self.will_handle_invoice() => {
                let context =
                    contexts::Invoice::new(self.bot.clone(), data, invoice);
                self.run_invoice_handlers(Arc::new(context));
            }
            message::Kind::LeftChatMember(member)
                if self.will_handle_left_member() =>
            {
                let context =
                    contexts::LeftMember::new(self.bot.clone(), data, member);
                self.run_left_member_handlers(Arc::new(context));
            }
            message::Kind::Location(location)
                if self.will_handle_location() =>
            {
                let context =
                    contexts::Location::new(self.bot.clone(), data, location);
                self.run_location_handlers(Arc::new(context));
            }
            message::Kind::MigrateFrom(old_id)
                if self.will_handle_migration() =>
            {
                let context =
                    contexts::Migration::new(self.bot.clone(), data, old_id);
                self.run_migration_handlers(Arc::new(context));
            }
            message::Kind::MigrateTo(..) => (), // ignored on purpose
            message::Kind::NewChatMembers(members)
                if self.will_handle_new_members() =>
            {
                let context =
                    contexts::NewMembers::new(self.bot.clone(), data, members);
                self.run_new_members_handlers(Arc::new(context));
            }
            message::Kind::NewChatPhoto(photo)
                if self.will_handle_new_chat_photo() =>
            {
                let context =
                    contexts::NewChatPhoto::new(self.bot.clone(), data, photo);
                self.run_new_chat_photo_handlers(Arc::new(context));
            }
            message::Kind::NewChatTitle(title)
                if self.will_handle_new_chat_title() =>
            {
                let context =
                    contexts::NewChatTitle::new(self.bot.clone(), data, title);
                self.run_new_chat_title_handlers(Arc::new(context));
            }
            message::Kind::PassportData(passport_data)
                if self.will_handle_passport() =>
            {
                let context = contexts::Passport::new(
                    self.bot.clone(),
                    data,
                    passport_data,
                );
                self.run_passport_handlers(Arc::new(context));
            }
            message::Kind::Photo {
                photo,
                caption,
                media_group_id,
            } if self.will_handle_photo() => {
                let context = contexts::Photo::new(
                    self.bot.clone(),
                    data,
                    photo,
                    caption,
                    media_group_id,
                );
                self.run_photo_handlers(Arc::new(context));
            }
            message::Kind::Pinned(message)
                if self.will_handle_pinned_message() =>
            {
                let context = contexts::PinnedMessage::new(
                    self.bot.clone(),
                    data,
                    *message,
                );
                self.run_pinned_message_handlers(Arc::new(context));
            }
            message::Kind::Poll(poll) if self.will_handle_poll() => {
                let context = contexts::Poll::new(self.bot.clone(), data, poll);
                self.run_poll_handlers(Arc::new(context));
            }
            message::Kind::Sticker(sticker) if self.will_handle_sticker() => {
                let context =
                    contexts::Sticker::new(self.bot.clone(), data, *sticker);
                self.run_sticker_handlers(Arc::new(context));
            }
            message::Kind::SuccessfulPayment(payment)
                if self.will_handle_payment() =>
            {
                let context =
                    contexts::Payment::new(self.bot.clone(), data, *payment);
                self.run_payment_handlers(Arc::new(context));
            }
            message::Kind::Text(text) if is_command(&text) => {
                let (command, username) = parse_command(&text);

                if !self.is_for_this_bot(username) {
                    return;
                }

                if self.will_handle_command(&command) {
                    let text = trim_command(text);
                    let context = contexts::Command::new(
                        self.bot.clone(),
                        data,
                        text,
                        command.clone(),
                    );
                    self.run_command_handlers(&command, &Arc::new(context));
                } else if self.will_handle_unhandled() {
                    let kind = message::Kind::Text(text);
                    let message = Message::new(data, kind);
                    let update = update::Kind::Message(message);
                    self.run_unhandled_handlers(update);
                }
            }
            message::Kind::Text(text) if self.will_handle_text() => {
                let context = contexts::Text::new(self.bot.clone(), data, text);
                self.run_text_handlers(Arc::new(context));
            }
            message::Kind::Venue(venue) if self.will_handle_venue() => {
                let context =
                    contexts::Venue::new(self.bot.clone(), data, venue);
                self.run_venue_handlers(Arc::new(context));
            }
            message::Kind::Video {
                video,
                caption,
                media_group_id,
            } if self.will_handle_video() => {
                let context = contexts::Video::new(
                    self.bot.clone(),
                    data,
                    *video,
                    caption,
                    media_group_id,
                );
                self.run_video_handlers(Arc::new(context));
            }
            message::Kind::VideoNote(video_note)
                if self.will_handle_video_note() =>
            {
                let context = contexts::VideoNote::new(
                    self.bot.clone(),
                    data,
                    video_note,
                );
                self.run_video_note_handlers(Arc::new(context));
            }
            message::Kind::Voice { voice, caption }
                if self.will_handle_voice() =>
            {
                let context = contexts::Voice::new(
                    self.bot.clone(),
                    data,
                    voice,
                    caption,
                );
                self.run_voice_handlers(Arc::new(context));
            }
            message::Kind::SupergroupCreated
            | message::Kind::ChannelCreated => {
                warn!("Update not expected; skipping it")
            }
            kind if self.will_handle_unhandled() => {
                let message = Message::new(data, kind);
                let update = update::Kind::Message(message);
                self.run_unhandled_handlers(update);
            }
            message::Kind::Animation { .. }
            | message::Kind::Audio { .. }
            | message::Kind::ChatPhotoDeleted
            | message::Kind::ConnectedWebsite(..)
            | message::Kind::Contact(..)
            | message::Kind::Dice(..)
            | message::Kind::Document { .. }
            | message::Kind::Game(..)
            | message::Kind::GroupCreated
            | message::Kind::Invoice(..)
            | message::Kind::LeftChatMember(..)
            | message::Kind::Location(..)
            | message::Kind::MigrateFrom(..)
            | message::Kind::NewChatMembers(..)
            | message::Kind::NewChatPhoto(..)
            | message::Kind::NewChatTitle(..)
            | message::Kind::PassportData(..)
            | message::Kind::Photo { .. }
            | message::Kind::Pinned(..)
            | message::Kind::Poll(..)
            | message::Kind::Sticker(..)
            | message::Kind::SuccessfulPayment(..)
            | message::Kind::Text(..)
            | message::Kind::Venue(..)
            | message::Kind::Video { .. }
            | message::Kind::VideoNote(..)
            | message::Kind::Voice { .. }
            | message::Kind::Unknown => (),
        }
    }

    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)] // can't split the huge match
    fn handle_message_edit_update(&self, message: types::Message) {
        let (data, kind) = message.split();
        let edit_date = if let Some(edit_date) = data.edit_date {
            edit_date
        } else {
            error!("No `edit_date` on an edited message; skipping it");
            return;
        };

        match kind {
            message::Kind::Animation { animation, caption }
                if self.will_handle_edited_animation() =>
            {
                let context = contexts::EditedAnimation::new(
                    self.bot.clone(),
                    data,
                    edit_date,
                    *animation,
                    caption,
                );
                self.run_edited_animation_handlers(Arc::new(context));
            }
            message::Kind::Audio {
                audio,
                caption,
                media_group_id,
            } if self.will_handle_edited_audio() => {
                let context = contexts::EditedAudio::new(
                    self.bot.clone(),
                    data,
                    edit_date,
                    *audio,
                    caption,
                    media_group_id,
                );
                self.run_edited_audio_handlers(Arc::new(context));
            }
            message::Kind::Document {
                document,
                caption,
                media_group_id,
            } if self.will_handle_edited_document() => {
                let context = contexts::EditedDocument::new(
                    self.bot.clone(),
                    data,
                    edit_date,
                    *document,
                    caption,
                    media_group_id,
                );
                self.run_edited_document_handlers(Arc::new(context));
            }
            message::Kind::Location(location)
                if self.will_handle_edited_location() =>
            {
                let context = contexts::EditedLocation::new(
                    self.bot.clone(),
                    data,
                    edit_date,
                    location,
                );
                self.run_edited_location_handlers(Arc::new(context));
            }
            message::Kind::Photo {
                photo,
                caption,
                media_group_id,
            } if self.will_handle_edited_photo() => {
                let context = contexts::EditedPhoto::new(
                    self.bot.clone(),
                    data,
                    edit_date,
                    photo,
                    caption,
                    media_group_id,
                );
                self.run_edited_photo_handlers(Arc::new(context));
            }
            message::Kind::Text(text) if is_command(&text) => {
                let (command, username) = parse_command(&text);
                if !self.is_for_this_bot(username) {
                    return;
                }

                if self.will_handle_edited_command(&command) {
                    let text = trim_command(text);
                    let context = contexts::EditedCommand::new(
                        self.bot.clone(),
                        data,
                        edit_date,
                        text,
                        command.clone(),
                    );
                    self.run_edited_command_handlers(
                        &command,
                        &Arc::new(context),
                    );
                } else if self.will_handle_unhandled() {
                    let kind = message::Kind::Text(text);
                    let message = Message::new(data, kind);
                    let update = update::Kind::EditedMessage(message);
                    self.run_unhandled_handlers(update);
                }
            }
            message::Kind::Text(text) if self.will_handle_edited_text() => {
                let context = contexts::EditedText::new(
                    self.bot.clone(),
                    data,
                    edit_date,
                    text,
                );
                self.run_edited_text_handlers(Arc::new(context));
            }
            message::Kind::Video {
                video,
                caption,
                media_group_id,
            } if self.will_handle_edited_video() => {
                let context = contexts::EditedVideo::new(
                    self.bot.clone(),
                    data,
                    edit_date,
                    *video,
                    caption,
                    media_group_id,
                );
                self.run_edited_video_handlers(Arc::new(context));
            }

            message::Kind::Contact(..)
            | message::Kind::Dice(..)
            | message::Kind::Game(..)
            | message::Kind::Invoice(..)
            | message::Kind::Poll(..)
            | message::Kind::Sticker(..)
            | message::Kind::Venue(..)
            | message::Kind::VideoNote(..)
            | message::Kind::Voice { .. }
            | message::Kind::ChannelCreated
            | message::Kind::ChatPhotoDeleted
            | message::Kind::ConnectedWebsite(..)
            | message::Kind::GroupCreated
            | message::Kind::LeftChatMember(..)
            | message::Kind::MigrateFrom(..)
            | message::Kind::MigrateTo(..)
            | message::Kind::NewChatMembers(..)
            | message::Kind::NewChatPhoto(..)
            | message::Kind::NewChatTitle(..)
            | message::Kind::PassportData(..)
            | message::Kind::Pinned(..)
            | message::Kind::SuccessfulPayment(..)
            | message::Kind::SupergroupCreated => warn!(
                "Unexpected message kind received as an edited message; \
                skipping it"
            ),

            kind if self.will_handle_unhandled() => {
                let message = Message::new(data, kind);
                let update = update::Kind::EditedMessage(message);
                self.run_unhandled_handlers(update)
            }
            message::Kind::Animation { .. }
            | message::Kind::Audio { .. }
            | message::Kind::Document { .. }
            | message::Kind::Location(..)
            | message::Kind::Photo { .. }
            | message::Kind::Text(..)
            | message::Kind::Video { .. }
            | message::Kind::Unknown => (),
        }
    }

    fn is_for_this_bot(&self, username: Option<&str>) -> bool {
        username.map_or(true, |username| {
            self.username.as_ref().map(|x| x == username) == Some(true)
        })
    }

    pub(crate) async fn set_commands_descriptions(
        &self,
    ) -> Result<(), MethodCall> {
        if self.command_description.is_empty() {
            return Ok(());
        }

        let commands: Vec<_> = self
            .command_description
            .iter()
            .map(|(name, description)| BotCommand::new(name, description))
            .collect();

        self.bot.set_my_commands(commands).call().await?;

        Ok(())
    }

    /// Fetches the bot's username.
    ///
    /// The username is used when checking if a command such as
    /// `/command@username` was directed to the bot.
    pub async fn fetch_username(&mut self) -> Result<(), errors::MethodCall> {
        let me = self.bot.get_me().call().await?;

        let username = me
            .user
            .username
            .expect("[tbot] Expected the bot to have a username");
        self.username(username);

        Ok(())
    }
}

fn is_command(text: &Text) -> bool {
    text.entities.get(0).map(|entity| {
        entity.kind == EntityKind::BotCommand && entity.offset == 0
    }) == Some(true)
}

fn parse_command(text: &Text) -> (String, Option<&str>) {
    let mut iter =
        // As this function is only run when a message starts with `/`,
        // the first value will always be yielded.
        text.value.split_whitespace().next().unwrap()[1..].split('@');

    // `split` always yields the first value.
    let command = iter.next().unwrap();
    let username = iter.next();

    (command.to_string(), username)
}

fn trim_command(text: Text) -> Text {
    let mut entities = text.entities.into_iter();
    // As this function is only called when the message is a command, the first
    // entity will always exist.
    let command_entity = entities.next().unwrap();
    let old_length = text.value.chars().count();

    let value: String = text
        .value
        .chars()
        .skip(command_entity.length)
        .skip_while(|x| x.is_whitespace())
        .collect();
    let new_length = value.chars().count();

    let entities = entities
        .map(|entity| Entity {
            kind: entity.kind,
            length: entity.length,
            offset: entity.offset - (old_length - new_length),
        })
        .collect();

    Text { value, entities }
}

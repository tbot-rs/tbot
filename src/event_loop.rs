//! The event loop for handling bot updates.

#[allow(clippy::wildcard_imports)]
use crate::{
    contexts::{fields::Context, *},
    errors::{self, MethodCall},
    state::StatefulEventLoop,
    types::{
        self, callback,
        callback::Query,
        message::{
            self,
            text::{Entity, EntityKind},
            Message,
        },
        update, BotCommand,
    },
    Bot,
};
use std::{collections::HashMap, future::Future, sync::Arc};
use tracing::{error, instrument, trace, warn};
use type_map::concurrent::TypeMap;

#[macro_use]
mod handlers_macros;

mod polling;
pub mod webhook;

pub use {polling::Polling, webhook::Webhook};

// Wish trait alises came out soon
type Handler<T> = dyn Fn(Arc<T>) + Send + Sync;
type Handlers<T> = Vec<Box<Handler<T>>>;
type Map<T> = HashMap<String, Handlers<T>>;

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

    command_handlers: Map<Command>,
    command_description: HashMap<String, String>,
    edited_command_handlers: Map<EditedCommand>,
    update_handlers: TypeMap,
}

impl EventLoop {
    pub(crate) fn new(bot: Bot) -> Self {
        Self {
            bot,
            username: None,
            command_handlers: HashMap::new(),
            command_description: HashMap::new(),
            edited_command_handlers: HashMap::new(),
            update_handlers: TypeMap::new(),
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

    fn add_handler<C, H, F>(&mut self, handler: H)
    where
        C: Context,
        H: Fn(Arc<C>) -> F + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.update_handlers
            .entry::<Handlers<C>>()
            .or_insert_with(Vec::new)
            .push(Box::new(move |context| {
                tokio::spawn(handler(context));
            }));
    }

    fn will_handle<C: Context>(&self) -> bool {
        self.update_handlers.contains::<Handlers<C>>()
    }

    #[allow(clippy::needless_pass_by_value)]
    fn handle<C: Context>(&self, context: Arc<C>) {
        let handlers = match self.update_handlers.get::<Handlers<C>>() {
            Some(handlers) => handlers,
            None => return,
        };

        handlers
            .iter()
            .for_each(|handler| handler(Arc::clone(&context)));
    }

    /// Registers a new handler for a command.
    ///
    /// Note that commands such as `/command@username` will be completely
    /// ignored unless you configure the event loop with your bot's username
    /// with either [`username`] or [`fetch_username`].
    ///
    /// [`username`]: #method.username
    /// [`fetch_username`]: #method.fetch_username
    pub fn command<H, F>(&mut self, command: &'static str, handler: H)
    where
        H: (Fn(Arc<Command>) -> F) + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.command_handlers
            .entry(command.to_string())
            .or_insert_with(Vec::new)
            .push(Box::new(move |context| {
                tokio::spawn(handler(context));
            }));
    }

    /// Registers a new handler for a command and sets its description.
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
        H: (Fn(Arc<Command>) -> F) + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.command_description
            .insert(command.to_string(), description.to_string());
        self.command(command, handler);
    }

    /// Registers a new handler for a sequence of commands.
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
        H: (Fn(Arc<Command>) -> F) + Send + Sync + 'static,
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

    fn handle_command(&self, command: &str, context: &Arc<Command>) {
        if let Some(handlers) = self.command_handlers.get(command) {
            for handler in handlers {
                handler(context.clone());
            }
        }
    }

    /// Registers a new handler for the `/start` command.
    pub fn start<H, F>(&mut self, handler: H)
    where
        H: (Fn(Arc<Command>) -> F) + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.command("start", handler);
    }

    /// Registers a new handler for the `/start` command and sets its
    /// description.
    pub fn start_with_description<H, F>(
        &mut self,
        description: &'static str,
        handler: H,
    ) where
        H: (Fn(Arc<Command>) -> F) + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.command_with_description("start", description, handler);
    }

    /// Registers a new handler for the `/settings` command.
    pub fn settings<H, F>(&mut self, handler: H)
    where
        H: (Fn(Arc<Command>) -> F) + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.command("settings", handler);
    }

    /// Registers a new handler for the `/settings` command and sets its
    /// description.
    pub fn settings_with_description<H, F>(
        &mut self,
        description: &'static str,
        handler: H,
    ) where
        H: (Fn(Arc<Command>) -> F) + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.command_with_description("settings", description, handler);
    }

    /// Registers a new handler for the `/help` command.
    pub fn help<H, F>(&mut self, handler: H)
    where
        H: (Fn(Arc<Command>) -> F) + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.command("help", handler);
    }

    /// Registers a new handler for the `/help` command and sets its
    /// description.
    pub fn help_with_description<H, F>(
        &mut self,
        description: &'static str,
        handler: H,
    ) where
        H: (Fn(Arc<Command>) -> F) + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.command_with_description("help", description, handler);
    }

    /// Registers a new handler for an edited command.
    pub fn edited_command<H, F>(&mut self, command: &'static str, handler: H)
    where
        H: (Fn(Arc<EditedCommand>) -> F) + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.edited_command_handlers
            .entry(command.to_string())
            .or_insert_with(Vec::new)
            .push(Box::new(move |context| {
                tokio::spawn(handler(context));
            }));
    }

    /// Registers a new handler for an edited command from sequence of commands.
    pub fn edited_commands<Cm, H, F>(&mut self, commands: Cm, handler: H)
    where
        Cm: IntoIterator<Item = &'static str>,
        F: Future<Output = ()> + Send + 'static,
        H: (Fn(Arc<EditedCommand>) -> F) + Send + Sync + 'static,
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

    fn handle_edited_command(
        &self,
        command: &str,
        context: &Arc<EditedCommand>,
    ) {
        if let Some(handlers) = self.edited_command_handlers.get(command) {
            for handler in handlers {
                handler(context.clone());
            }
        }
    }

    handlers! {
        /// Registers a new handler for all incoming updates.
        ///
        /// `any_update` handlers are spawned before specialized handlers,
        /// for every update that could be deserialized (this means that
        /// processing new updates on old versions of `tbot` is not possible
        /// even via `any_update`).
        ///
        /// Note “spawned”: every handler is executed using [`tokio::spawn`],
        /// and `tbot` won't wait for them to finish. As such, `any_update` is
        /// not suitable for running some code before every specialized handler.
        ///
        /// Also, `any_update` does not affect [`unhandled`] in any way. It's
        /// executed if a _specialized_ handler corresponding to the incoming
        /// update wasn't registered.
        ///
        /// Registering specialized handler is still preferred. if at least
        /// one `any_update` handler is registered, `tbot` will have to clone
        /// every update in order to execute these and specialized handlers.
        ///
        /// [`unhandled`]: Self::unhandled
        any_update: AnyUpdate,
        /// Registers a new handler for animations.
        animation: Animation,
        /// Registers a new handler for audio.
        audio: Audio,
        /// Registers a new handler for chosen inline results.
        chosen_inline: ChosenInline,
        /// Registers a new handler for contacts.
        contact: Contact,
        /// Registers a new handler for connected websites.
        connected_website: ConnectedWebsite,
        /// Registers a new handler for created groups.
        created_group: CreatedGroup,
        /// Registers a new handler for data callbacks from chat messages.
        message_data_callback: MessageDataCallback,
        /// Registers a new handler for data callbacks from inline messages.
        inline_data_callback: InlineDataCallback,
        /// Registers a new handler for deleted chat photos.
        deleted_chat_photo: DeletedChatPhoto,
        /// Registers a new handler for dice.
        dice: Dice,
        /// Registers a new handler for documents.
        document: Document,
        /// Registers a new handler for edited animations.
        edited_animation: EditedAnimation,
        /// Registers a new handler for edited audio.
        edited_audio: EditedAudio,
        /// Registers a new handler for edited documents.
        edited_document: EditedDocument,
        /// Registers a new handler for edited locations.
        edited_location: EditedLocation,
        /// Registers a new handler for edited photos.
        edited_photo: EditedPhoto,
        /// Registers a new handler for edited text messages.
        edited_text: EditedText,
        /// Registers a new handler for edited videos.
        edited_video: EditedVideo,
        /// Registers a new handler for game callbacks from chat messages.
        message_game_callback: MessageGameCallback,
        /// Registers a new handler for game callbacks from inline messages.
        inline_game_callback: InlineGameCallback,
        /// Registers a new handler for game messages.
        game: Game,
        /// Registers a new handler for inline queries.
        inline: Inline,
        /// Registers a new handler for invoices.
        invoice: Invoice,
        /// Registers a new handler for left members.
        left_member: LeftMember,
        /// Registers a new handler for locations.
        location: Location,
        /// Registers a new handler for migrations.
        migration: Migration,
        /// Registers a new handler for new chat photos.
        new_chat_photo: NewChatPhoto,
        /// Registers a new handler for new chat titles.
        new_chat_title: NewChatTitle,
        /// Registers a new handler for new members.
        new_members: NewMembers,
        /// Registers a new handler for passport data.
        passport: Passport,
        /// Registers a new handler for successful payments.
        payment: Payment,
        /// Registers a new handler for photos.
        photo: Photo,
        /// Registers a new handler for pinned messages.
        pinned_message: PinnedMessage,
        /// Registers a new handler for poll messages.
        poll: Poll,
        /// Registers a new handler for pre-checkout queries.
        pre_checkout: PreCheckout,
        /// Registers a new handler for proximity alerts.
        proximity_alert: ProximityAlert,
        /// Registers a new handler for shipping queries.
        shipping: Shipping,
        /// Registers a new handler for stickers.
        sticker: Sticker,
        /// Registers a new handler for text messages.
        text: Text,
        /// Registers a new handler for unhandled updates.
        ///
        /// Note that regisering [`any_update`] handlers does not affect
        /// `unhandled` handlers in any way. An `unhandled` handler is spawned
        /// if a _specialized_ handler corresponding to the incoming update was
        /// not registered.
        ///
        /// [`any_update`]: Self::any_update
        unhandled: Unhandled,
        /// Registers a new handler for new states of polls.
        updated_poll: UpdatedPoll,
        /// Registers a new handler for new answers in the poll.
        poll_answer: PollAnswer,
        /// Registers a new handler for venues.
        venue: Venue,
        /// Registers a new handler for videos.
        video: Video,
        /// Registers a new handler for video notes.
        video_note: VideoNote,
        /// Registers a new handler for voice messages.
        voice: Voice,
        /// Registers a new handler for when users are invited to a voice chat.
        voice_chat_participants_invited: VoiceChatParticipantsInvited,
        /// Registers a new handler for when a voice chat is started.
        voice_chat_started: VoiceChatStarted,
    }

    fn handle_unhandled(&self, update: update::Kind) {
        let context = Arc::new(Unhandled::new(self.bot.clone(), update));
        self.handle(context);
    }

    #[instrument(skip(self, update))]
    fn handle_update(&self, update: types::Update) {
        trace!(?update);

        if self.will_handle::<AnyUpdate>() {
            let context = AnyUpdate::new(self.bot.clone(), update.clone());
            self.handle(Arc::new(context));
        }

        match update.kind {
            update::Kind::CallbackQuery(query) => match query {
                Query {
                    kind: callback::Kind::Data(data),
                    origin: callback::Origin::Message(message),
                    id,
                    from,
                    chat_instance,
                } if self.will_handle::<MessageDataCallback>() => {
                    let context = MessageDataCallback::new(
                        self.bot.clone(),
                        id,
                        from,
                        *message,
                        chat_instance,
                        data,
                    );
                    self.handle(Arc::new(context));
                }
                Query {
                    kind: callback::Kind::Data(data),
                    origin: callback::Origin::Inline(message_id),
                    id,
                    from,
                    chat_instance,
                } if self.will_handle::<InlineDataCallback>() => {
                    let context = InlineDataCallback::new(
                        self.bot.clone(),
                        id,
                        from,
                        message_id,
                        chat_instance,
                        data,
                    );
                    self.handle(Arc::new(context));
                }
                Query {
                    kind: callback::Kind::Game(game),
                    origin: callback::Origin::Message(message),
                    id,
                    from,
                    chat_instance,
                } if self.will_handle::<MessageGameCallback>() => {
                    let context = MessageGameCallback::new(
                        self.bot.clone(),
                        id,
                        from,
                        *message,
                        chat_instance,
                        game,
                    );
                    self.handle(Arc::new(context));
                }
                Query {
                    kind: callback::Kind::Game(game),
                    origin: callback::Origin::Inline(message_id),
                    id,
                    from,
                    chat_instance,
                } if self.will_handle::<InlineGameCallback>() => {
                    let context = InlineGameCallback::new(
                        self.bot.clone(),
                        id,
                        from,
                        message_id,
                        chat_instance,
                        game,
                    );
                    self.handle(Arc::new(context));
                }
                query if self.will_handle::<Unhandled>() => {
                    let update = update::Kind::CallbackQuery(query);
                    self.handle_unhandled(update);
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
                if self.will_handle::<ChosenInline>() =>
            {
                let context = ChosenInline::new(self.bot.clone(), result);
                self.handle(Arc::new(context));
            }
            update::Kind::EditedMessage(message)
            | update::Kind::EditedChannelPost(message) => {
                self.handle_message_edit_update(message);
            }
            update::Kind::InlineQuery(query)
                if self.will_handle::<Inline>() =>
            {
                let context = Inline::new(self.bot.clone(), query);
                self.handle(Arc::new(context));
            }
            update::Kind::Message(message)
            | update::Kind::ChannelPost(message) => {
                self.handle_message_update(message);
            }
            update::Kind::PreCheckoutQuery(query)
                if self.will_handle::<PreCheckout>() =>
            {
                let context = PreCheckout::new(self.bot.clone(), query);
                self.handle(Arc::new(context));
            }
            update::Kind::Poll(poll) if self.will_handle::<UpdatedPoll>() => {
                let context = UpdatedPoll::new(self.bot.clone(), poll);
                self.handle(Arc::new(context));
            }
            update::Kind::PollAnswer(answer)
                if self.will_handle::<PollAnswer>() =>
            {
                let context = PollAnswer::new(self.bot.clone(), answer);
                self.handle(Arc::new(context));
            }
            update::Kind::ShippingQuery(query)
                if self.will_handle::<Shipping>() =>
            {
                let context = Shipping::new(self.bot.clone(), query);
                self.handle(Arc::new(context));
            }
            update if self.will_handle::<Unhandled>() => {
                self.handle_unhandled(update);
            }
            update::Kind::ChosenInlineResult(..)
            | update::Kind::InlineQuery(..)
            | update::Kind::Poll(..)
            | update::Kind::PollAnswer(..)
            | update::Kind::PreCheckoutQuery(..)
            | update::Kind::ShippingQuery(..)
            | update::Kind::Unknown => (),
        }
    }

    #[allow(clippy::cognitive_complexity)]
    #[allow(clippy::too_many_lines)] // can't split the huge match
    fn handle_message_update(&self, message: types::Message) {
        let (data, kind) = message.split();

        match kind {
            message::Kind::Animation { animation, caption }
                if self.will_handle::<Animation>() =>
            {
                let context =
                    Animation::new(self.bot.clone(), data, *animation, caption);
                self.handle(Arc::new(context));
            }
            message::Kind::Audio {
                audio,
                caption,
                media_group_id,
            } if self.will_handle::<Audio>() => {
                let context = Audio::new(
                    self.bot.clone(),
                    data,
                    *audio,
                    caption,
                    media_group_id,
                );
                self.handle(Arc::new(context));
            }
            message::Kind::ChatPhotoDeleted
                if self.will_handle::<DeletedChatPhoto>() =>
            {
                let context = DeletedChatPhoto::new(self.bot.clone(), data);
                self.handle(Arc::new(context));
            }
            message::Kind::ConnectedWebsite(website)
                if self.will_handle::<ConnectedWebsite>() =>
            {
                let context =
                    ConnectedWebsite::new(self.bot.clone(), data, website);
                self.handle(Arc::new(context));
            }
            message::Kind::Contact(contact)
                if self.will_handle::<Contact>() =>
            {
                let context = Contact::new(self.bot.clone(), data, contact);
                self.handle(Arc::new(context));
            }
            message::Kind::Dice(dice) if self.will_handle::<Dice>() => {
                let context = Dice::new(self.bot.clone(), data, dice);
                self.handle(Arc::new(context));
            }
            message::Kind::Document {
                document,
                caption,
                media_group_id,
            } if self.will_handle::<Document>() => {
                let context = Document::new(
                    self.bot.clone(),
                    data,
                    *document,
                    caption,
                    media_group_id,
                );
                self.handle(Arc::new(context));
            }
            message::Kind::Game(game) if self.will_handle::<Game>() => {
                let context = Game::new(self.bot.clone(), data, *game);
                self.handle(Arc::new(context));
            }
            message::Kind::GroupCreated
                if self.will_handle::<CreatedGroup>() =>
            {
                let context = CreatedGroup::new(self.bot.clone(), data);
                self.handle(Arc::new(context));
            }
            message::Kind::Invoice(invoice)
                if self.will_handle::<Invoice>() =>
            {
                let context = Invoice::new(self.bot.clone(), data, invoice);
                self.handle(Arc::new(context));
            }
            message::Kind::LeftChatMember(member)
                if self.will_handle::<LeftMember>() =>
            {
                let context = LeftMember::new(self.bot.clone(), data, member);
                self.handle(Arc::new(context));
            }
            message::Kind::Location(location)
                if self.will_handle::<Location>() =>
            {
                let context = Location::new(self.bot.clone(), data, location);
                self.handle(Arc::new(context));
            }
            message::Kind::MigrateFrom(old_id)
                if self.will_handle::<Migration>() =>
            {
                let context = Migration::new(self.bot.clone(), data, old_id);
                self.handle(Arc::new(context));
            }
            message::Kind::MigrateTo(..) => (), // ignored on purpose
            message::Kind::NewChatMembers(members)
                if self.will_handle::<NewMembers>() =>
            {
                let context = NewMembers::new(self.bot.clone(), data, members);
                self.handle(Arc::new(context));
            }
            message::Kind::NewChatPhoto(photo)
                if self.will_handle::<NewChatPhoto>() =>
            {
                let context = NewChatPhoto::new(self.bot.clone(), data, photo);
                self.handle(Arc::new(context));
            }
            message::Kind::NewChatTitle(title)
                if self.will_handle::<NewChatTitle>() =>
            {
                let context = NewChatTitle::new(self.bot.clone(), data, title);
                self.handle(Arc::new(context));
            }
            message::Kind::PassportData(passport_data)
                if self.will_handle::<Passport>() =>
            {
                let context =
                    Passport::new(self.bot.clone(), data, passport_data);
                self.handle(Arc::new(context));
            }
            message::Kind::Photo {
                photo,
                caption,
                media_group_id,
            } if self.will_handle::<Photo>() => {
                let context = Photo::new(
                    self.bot.clone(),
                    data,
                    photo,
                    caption,
                    media_group_id,
                );
                self.handle(Arc::new(context));
            }
            message::Kind::Pinned(message)
                if self.will_handle::<PinnedMessage>() =>
            {
                let context =
                    PinnedMessage::new(self.bot.clone(), data, *message);
                self.handle(Arc::new(context));
            }
            message::Kind::Poll(poll) if self.will_handle::<Poll>() => {
                let context = Poll::new(self.bot.clone(), data, poll);
                self.handle(Arc::new(context));
            }
            message::Kind::ProximityAlert(alert)
                if self.will_handle::<ProximityAlert>() =>
            {
                let context =
                    ProximityAlert::new(self.bot.clone(), data, alert);
                self.handle(Arc::new(context));
            }
            message::Kind::Sticker(sticker)
                if self.will_handle::<Sticker>() =>
            {
                let context = Sticker::new(self.bot.clone(), data, *sticker);
                self.handle(Arc::new(context));
            }
            message::Kind::SuccessfulPayment(payment)
                if self.will_handle::<Payment>() =>
            {
                let context = Payment::new(self.bot.clone(), data, *payment);
                self.handle(Arc::new(context));
            }
            message::Kind::Text(text) if is_command(&text) => {
                let (command, username) = parse_command(&text);

                if !self.is_for_this_bot(username) {
                    return;
                }

                if self.will_handle_command(&command) {
                    let text = trim_command(text);
                    let context = Command::new(
                        self.bot.clone(),
                        data,
                        text,
                        command.clone(),
                    );
                    self.handle_command(&command, &Arc::new(context));
                } else if self.will_handle::<Unhandled>() {
                    let kind = message::Kind::Text(text);
                    let message = Message::new(data, kind);
                    let update = update::Kind::Message(message);
                    self.handle_unhandled(update);
                }
            }
            message::Kind::Text(text) if self.will_handle::<Text>() => {
                let context = Text::new(self.bot.clone(), data, text);
                self.handle(Arc::new(context));
            }
            message::Kind::Venue(venue) if self.will_handle::<Venue>() => {
                let context = Venue::new(self.bot.clone(), data, venue);
                self.handle(Arc::new(context));
            }
            message::Kind::Video {
                video,
                caption,
                media_group_id,
            } if self.will_handle::<Video>() => {
                let context = Video::new(
                    self.bot.clone(),
                    data,
                    *video,
                    caption,
                    media_group_id,
                );
                self.handle(Arc::new(context));
            }
            message::Kind::VideoNote(video_note)
                if self.will_handle::<VideoNote>() =>
            {
                let context =
                    VideoNote::new(self.bot.clone(), data, video_note);
                self.handle(Arc::new(context));
            }
            message::Kind::Voice { voice, caption }
                if self.will_handle::<Voice>() =>
            {
                let context =
                    Voice::new(self.bot.clone(), data, voice, caption);
                self.handle(Arc::new(context));
            }
            message::Kind::VoiceChatParticipantsInvited(invited)
                if self.will_handle::<VoiceChatParticipantsInvited>() =>
            {
                let context = VoiceChatParticipantsInvited::new(
                    self.bot.clone(),
                    data,
                    invited,
                );
                self.handle(Arc::new(context));
            }
            message::Kind::VoiceChatStarted
                if self.will_handle::<VoiceChatStarted>() =>
            {
                let context = VoiceChatStarted::new(self.bot.clone(), data);
                self.handle(Arc::new(context));
            }
            message::Kind::SupergroupCreated
            | message::Kind::ChannelCreated => {
                warn!("Update not expected; skipping it")
            }
            kind if self.will_handle::<Unhandled>() => {
                let message = Message::new(data, kind);
                let update = update::Kind::Message(message);
                self.handle_unhandled(update);
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
            | message::Kind::ProximityAlert(..)
            | message::Kind::Sticker(..)
            | message::Kind::SuccessfulPayment(..)
            | message::Kind::Text(..)
            | message::Kind::Venue(..)
            | message::Kind::Video { .. }
            | message::Kind::VideoNote(..)
            | message::Kind::Voice { .. }
            | message::Kind::VoiceChatParticipantsInvited(..)
            | message::Kind::VoiceChatStarted
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
                if self.will_handle::<EditedAnimation>() =>
            {
                let context = EditedAnimation::new(
                    self.bot.clone(),
                    data,
                    edit_date,
                    *animation,
                    caption,
                );
                self.handle(Arc::new(context));
            }
            message::Kind::Audio {
                audio,
                caption,
                media_group_id,
            } if self.will_handle::<EditedAudio>() => {
                let context = EditedAudio::new(
                    self.bot.clone(),
                    data,
                    edit_date,
                    *audio,
                    caption,
                    media_group_id,
                );
                self.handle(Arc::new(context));
            }
            message::Kind::Document {
                document,
                caption,
                media_group_id,
            } if self.will_handle::<EditedDocument>() => {
                let context = EditedDocument::new(
                    self.bot.clone(),
                    data,
                    edit_date,
                    *document,
                    caption,
                    media_group_id,
                );
                self.handle(Arc::new(context));
            }
            message::Kind::Location(location)
                if self.will_handle::<EditedLocation>() =>
            {
                let context = EditedLocation::new(
                    self.bot.clone(),
                    data,
                    edit_date,
                    location,
                );
                self.handle(Arc::new(context));
            }
            message::Kind::Photo {
                photo,
                caption,
                media_group_id,
            } if self.will_handle::<EditedPhoto>() => {
                let context = EditedPhoto::new(
                    self.bot.clone(),
                    data,
                    edit_date,
                    photo,
                    caption,
                    media_group_id,
                );
                self.handle(Arc::new(context));
            }
            message::Kind::Text(text) if is_command(&text) => {
                let (command, username) = parse_command(&text);
                if !self.is_for_this_bot(username) {
                    return;
                }

                if self.will_handle_edited_command(&command) {
                    let text = trim_command(text);
                    let context = EditedCommand::new(
                        self.bot.clone(),
                        data,
                        edit_date,
                        text,
                        command.clone(),
                    );
                    self.handle_edited_command(&command, &Arc::new(context));
                } else if self.will_handle::<Unhandled>() {
                    let kind = message::Kind::Text(text);
                    let message = Message::new(data, kind);
                    let update = update::Kind::EditedMessage(message);
                    self.handle_unhandled(update);
                }
            }
            message::Kind::Text(text) if self.will_handle::<EditedText>() => {
                let context =
                    EditedText::new(self.bot.clone(), data, edit_date, text);
                self.handle(Arc::new(context));
            }
            message::Kind::Video {
                video,
                caption,
                media_group_id,
            } if self.will_handle::<EditedVideo>() => {
                let context = EditedVideo::new(
                    self.bot.clone(),
                    data,
                    edit_date,
                    *video,
                    caption,
                    media_group_id,
                );
                self.handle(Arc::new(context));
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
            | message::Kind::VoiceChatParticipantsInvited(..)
            | message::Kind::VoiceChatStarted
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
            | message::Kind::ProximityAlert(..)
            | message::Kind::SuccessfulPayment(..)
            | message::Kind::SupergroupCreated => warn!(
                "Unexpected message kind received as an edited message; \
                skipping it"
            ),

            kind if self.will_handle::<Unhandled>() => {
                let message = Message::new(data, kind);
                let update = update::Kind::EditedMessage(message);
                self.handle_unhandled(update)
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
}

fn is_command(text: &message::Text) -> bool {
    text.entities.get(0).map(|entity| {
        entity.kind == EntityKind::BotCommand && entity.offset == 0
    }) == Some(true)
}

fn parse_command(text: &message::Text) -> (String, Option<&str>) {
    let mut iter =
        // As this function is only run when a message starts with `/`,
        // the first value will always be yielded.
        text.value.split_whitespace().next().unwrap()[1..].split('@');

    // `split` always yields the first value.
    let command = iter.next().unwrap();
    let username = iter.next();

    (command.to_string(), username)
}

fn trim_command(text: message::Text) -> message::Text {
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

    message::Text { value, entities }
}

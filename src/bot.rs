use super::*;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use {
    methods::GetUpdates,
    types::{Message, MessageKind, UpdateKind},
};

#[macro_use]
mod handlers_macros;

mod mock_bot;
mod polling;
mod webhook;

pub use {mock_bot::*, polling::*, webhook::*};

type Handlers<T> = Vec<Mutex<Box<T>>>;

// Wish trait alises came out soon
type Handler<T> = dyn FnMut(&T) + Send + Sync;

type AnimationHandler = Handler<contexts::Animation>;
type AudioHandler = Handler<contexts::Audio>;
type ContactHandler = Handler<contexts::Contact>;
type CreatedGroupHandler = Handler<contexts::CreatedGroup>;
type DeletedChatPhotoHandler = Handler<contexts::DeletedChatPhoto>;
type DocumentHandler = Handler<contexts::Document>;
type EditedAnimationHandler = Handler<contexts::EditedAnimation>;
type EditedAudioHandler = Handler<contexts::EditedAudio>;
type EditedDocumentHandler = Handler<contexts::EditedDocument>;
type EditedLocationHandler = Handler<contexts::EditedLocation>;
type EditedPhotoHandler = Handler<contexts::EditedPhoto>;
type EditedTextHandler = Handler<contexts::EditedText>;
type EditedVideoHandler = Handler<contexts::EditedVideo>;
type GameHandler = Handler<contexts::Game>;
type LeftMemberHandler = Handler<contexts::LeftMember>;
type LocationHandler = Handler<contexts::Location>;
type MigrationHandler = Handler<contexts::Migration>;
type NewChatPhotoHandler = Handler<contexts::NewChatPhoto>;
type NewChatTitleHandler = Handler<contexts::NewChatTitle>;
type NewMembersHandler = Handler<contexts::NewMembers>;
type PhotoHandler = Handler<contexts::Photo>;
type PinnedMessageHandler = Handler<contexts::PinnedMessage>;
type PollHandler = Handler<contexts::Poll>;
type PollingErrorHandler = Handler<methods::DeliveryError>;
type StickerHandler = Handler<contexts::Sticker>;
type TextHandler = Handler<contexts::Text>;
type UnhandledHandler = Handler<contexts::Unhandled>;
type UpdatedPollHandler = Handler<contexts::UpdatedPoll>;
type UpdateHandler = Handler<contexts::Update>;
type VenueHandler = Handler<contexts::Venue>;
type VideoHandler = Handler<contexts::Video>;
type VideoNoteHandler = Handler<contexts::VideoNote>;
type VoiceHandler = Handler<contexts::Voice>;

/// Provides an event loop for handling Telegram updates, as well as methods
/// to call API methods.
///
/// The main purpose of a `Bot` is to configure update handlers and start
/// the event loop:
///
/// ```no_run
/// let mut bot = tbot::bot!("BOT_TOKEN");
///
/// bot.text(|_| println!("Got a text message"));
///
/// bot.polling().start();
/// ```
///
/// `tbot` has many update handlers, such as [`text`] you have seen
/// in the example. You can find all of them below on this page. Speaking
/// of the event loop, `tbot` supports [polling] and [webhook].
///
/// A `Bot` also implements the [`Methods`] trait which provides handy methods
/// on the struct to call API methods:
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
/// Note, however, that starting an event loop takes ownership of `Bot`, so, if
/// you need to call an API method after that, construct a [`MockBot`] with
/// [`Bot::mock`] that has no handling logic and thus can be cloned as much
/// as needed.
///
/// [polling]: #method.polling
/// [webhooks]: #method.webhook
/// [`text`]: #method.text
/// [`MockBot`]: ./struct.MockBot.html
/// [`Bot::mock`]: #method.mock
/// [`Methods`]: ./methods/trait.Methods.html
pub struct Bot {
    token: Arc<String>,
    #[cfg(feature = "proxy")]
    proxy: Option<proxy::Proxy>,
    command_handlers: HashMap<&'static str, Handlers<TextHandler>>,
    edited_command_handlers: HashMap<&'static str, Handlers<EditedTextHandler>>,
    after_update_handlers: Handlers<UpdateHandler>,
    animation_handlers: Handlers<AnimationHandler>,
    audio_handlers: Handlers<AudioHandler>,
    before_update_handlers: Handlers<UpdateHandler>,
    contact_handlers: Handlers<ContactHandler>,
    created_group_handlers: Handlers<CreatedGroupHandler>,
    deleted_chat_photo_handlers: Handlers<DeletedChatPhotoHandler>,
    document_handlers: Handlers<DocumentHandler>,
    edited_animation_handlers: Handlers<EditedAnimationHandler>,
    edited_audio_handlers: Handlers<EditedAudioHandler>,
    edited_document_handlers: Handlers<EditedDocumentHandler>,
    edited_location_handlers: Handlers<EditedLocationHandler>,
    edited_photo_handlers: Handlers<EditedPhotoHandler>,
    edited_text_handlers: Handlers<EditedTextHandler>,
    edited_video_handlers: Handlers<EditedVideoHandler>,
    game_handlers: Handlers<GameHandler>,
    left_member_handlers: Handlers<LeftMemberHandler>,
    location_handlers: Handlers<LocationHandler>,
    migration_handlers: Handlers<MigrationHandler>,
    new_chat_photo_handlers: Handlers<NewChatPhotoHandler>,
    new_chat_title_handlers: Handlers<NewChatTitleHandler>,
    new_members_handlers: Handlers<NewMembersHandler>,
    photo_handlers: Handlers<PhotoHandler>,
    pinned_message_handlers: Handlers<PinnedMessageHandler>,
    poll_handlers: Handlers<PollHandler>,
    polling_error_handlers: Handlers<PollingErrorHandler>,
    sticker_handlers: Handlers<StickerHandler>,
    text_handlers: Handlers<TextHandler>,
    unhandled_handlers: Handlers<UnhandledHandler>,
    updated_poll_handlers: Handlers<UpdatedPollHandler>,
    venue_handlers: Handlers<VenueHandler>,
    video_handlers: Handlers<VideoHandler>,
    video_note_handlers: Handlers<VideoNoteHandler>,
    voice_handlers: Handlers<VoiceHandler>,
    username: Option<&'static str>,
}

impl Bot {
    /// Constructs a new `Bot`.
    pub fn new(token: String) -> Self {
        Self {
            token: Arc::new(token),
            #[cfg(feature = "proxy")]
            proxy: None,
            command_handlers: HashMap::new(),
            edited_command_handlers: HashMap::new(),
            after_update_handlers: Vec::new(),
            animation_handlers: Vec::new(),
            audio_handlers: Vec::new(),
            before_update_handlers: Vec::new(),
            contact_handlers: Vec::new(),
            created_group_handlers: Vec::new(),
            deleted_chat_photo_handlers: Vec::new(),
            document_handlers: Vec::new(),
            edited_animation_handlers: Vec::new(),
            edited_audio_handlers: Vec::new(),
            edited_document_handlers: Vec::new(),
            edited_location_handlers: Vec::new(),
            edited_photo_handlers: Vec::new(),
            edited_text_handlers: Vec::new(),
            edited_video_handlers: Vec::new(),
            game_handlers: Vec::new(),
            left_member_handlers: Vec::new(),
            location_handlers: Vec::new(),
            migration_handlers: Vec::new(),
            new_chat_photo_handlers: Vec::new(),
            new_chat_title_handlers: Vec::new(),
            new_members_handlers: Vec::new(),
            photo_handlers: Vec::new(),
            pinned_message_handlers: Vec::new(),
            poll_handlers: Vec::new(),
            polling_error_handlers: Vec::new(),
            sticker_handlers: Vec::new(),
            text_handlers: Vec::new(),
            unhandled_handlers: Vec::new(),
            updated_poll_handlers: Vec::new(),
            venue_handlers: Vec::new(),
            video_handlers: Vec::new(),
            video_note_handlers: Vec::new(),
            voice_handlers: Vec::new(),
            username: None,
        }
    }

    /// Sets the bot's username.
    ///
    /// The username is used when checking if a command such as
    /// `/command@username` was directed to the bot.
    pub fn username(&mut self, username: &'static str) {
        self.username = Some(username);
    }

    /// Fetches the bot's username.
    ///
    /// # Panics
    ///
    /// This method panics if there was an error during calling the `getMe`
    /// method.
    pub fn fetch_username(&mut self) {
        let result = Arc::new(Mutex::new(None));
        let on_ok = Arc::clone(&result);
        let on_err = Arc::clone(&result);

        let get_me = self
            .get_me()
            .into_future()
            .map_err(move |error| {
                *on_err.lock().unwrap() = Some(Err(error));
            })
            .map(move |me| {
                *on_ok.lock().unwrap() = Some(Ok(me));
            });

        crate::run(get_me);

        let result = Arc::try_unwrap(result).unwrap().into_inner().unwrap();

        if let Some(result) = result {
            // will always run
            match result {
                Ok(me) => {
                    let username: String = me.username.expect(
                        "\n[tbot] Expected the bot to have a username\n",
                    );
                    let username = Box::leak(Box::new(username));

                    self.username(username);
                }
                Err(error) => panic!(
                    "\n[tbot] Error during fetching username: {:#?}\n",
                    error
                ),
            }
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
    /// ```
    /// use tbot::Bot;
    ///
    /// let mut bot = Bot::from_env("BOT_TOKEN");
    ///
    /// bot.text(|_| ());
    /// ```
    pub fn from_env(env_var: &'static str) -> Self {
        Self::new(std::env::var(env_var).unwrap_or_else(|_| {
            panic!("\n[tbot] Bot's token in {} was not specified\n", env_var)
        }))
    }

    /// Starts polling configuration.
    pub const fn polling<'a>(self) -> Polling<'a> {
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

    /// Sets a proxy through which requests to Telegram will be sent.
    #[cfg(feature = "proxy")]
    pub fn proxy(&mut self, proxy: proxy::Proxy) {
        self.proxy = Some(proxy);
    }

    /// Creates a new `MockBot` inheriting the token from this bot.
    pub fn mock(&self) -> MockBot {
        MockBot::new(
            Arc::clone(&self.token),
            #[cfg(feature = "proxy")]
            self.proxy.clone(),
        )
    }

    /// Adds a new handler for a command.
    pub fn command(
        &mut self,
        command: &'static str,
        handler: impl FnMut(&contexts::Text) + Send + Sync + 'static,
    ) {
        self.command_handlers
            .entry(command)
            .or_insert_with(Vec::new)
            .push(Mutex::new(Box::new(handler)));
    }

    fn will_handle_command(&self, command: &'static str) -> bool {
        self.command_handlers.contains_key(&command)
    }

    fn run_command_handlers(
        &self,
        command: &'static str,
        context: &contexts::Text,
    ) {
        if let Some(handlers) = self.command_handlers.get(&command) {
            for handler in handlers {
                (&mut *handler.lock().unwrap())(context);
            }
        }
    }

    /// Adds a new handler for the `/start` command.
    pub fn start(
        &mut self,
        handler: impl FnMut(&contexts::Text) + Send + Sync + 'static,
    ) {
        self.command("start", handler);
    }

    /// Adds a new handler for the `/settings` command.
    pub fn settings(
        &mut self,
        handler: impl FnMut(&contexts::Text) + Send + Sync + 'static,
    ) {
        self.command("settings", handler);
    }

    /// Adds a new handler for the `/help` command.
    pub fn help(
        &mut self,
        handler: impl FnMut(&contexts::Text) + Send + Sync + 'static,
    ) {
        self.command("help", handler);
    }

    /// Adds a new handler for an edited command.
    pub fn edited_command(
        &mut self,
        command: &'static str,
        handler: impl FnMut(&contexts::EditedText) + Send + Sync + 'static,
    ) {
        self.edited_command_handlers
            .entry(command)
            .or_insert_with(Vec::new)
            .push(Mutex::new(Box::new(handler)));
    }

    fn will_handle_edited_command(&self, command: &'static str) -> bool {
        self.edited_command_handlers.contains_key(&command)
    }

    fn run_edited_command_handlers(
        &self,
        command: &'static str,
        context: &contexts::EditedText,
    ) {
        if let Some(handlers) = self.edited_command_handlers.get(&command) {
            for handler in handlers {
                (&mut *handler.lock().unwrap())(context);
            }
        }
    }

    handler! {
        /// Adds a new handler which is run after handling an update.
        after_update_handlers,
        after_update,
        contexts::Update,
        run_after_update_handlers,
    }

    handler! {
        /// Adds a new handler for animations.
        animation_handlers,
        animation,
        contexts::Animation,
        run_animation_handlers,
        will_handle_animation,
    }

    handler! {
        /// Adds a new handler for audio.
        audio_handlers,
        audio,
        contexts::Audio,
        run_audio_handlers,
        will_handle_audio,
    }

    handler! {
        /// Adds a new handler which is run before handling an update.
        before_update_handlers,
        before_update,
        contexts::Update,
        run_before_update_handlers,
    }

    handler! {
        /// Adds a new handler for contacts.
        contact_handlers,
        contact,
        contexts::Contact,
        run_contact_handlers,
        will_handle_contact,
    }

    handler! {
        /// Adds a new handler for created groups.
        created_group_handlers,
        created_group,
        contexts::CreatedGroup,
        run_created_group_handlers,
        will_handle_created_group,
    }

    handler! {
        /// Adds a new handler for deleted chat photos.
        deleted_chat_photo_handlers,
        deleted_chat_photo,
        contexts::DeletedChatPhoto,
        run_deleted_chat_photo_handlers,
        will_handle_deleted_chat_photo,
    }

    handler! {
        /// Adds a new handler for documents.
        document_handlers,
        document,
        contexts::Document,
        run_document_handlers,
        will_handle_document,
    }

    handler! {
        /// Adds a new handler for edited animations.
        edited_animation_handlers,
        edited_animation,
        contexts::EditedAnimation,
        run_edited_animation_handlers,
        will_handle_edited_animation,
    }

    handler! {
        /// Adds a new handler for edited audio.
        edited_audio_handlers,
        edited_audio,
        contexts::EditedAudio,
        run_edited_audio_handlers,
        will_handle_edited_audio,
    }

    handler! {
        /// Adds a new handler for edited documents.
        edited_document_handlers,
        edited_document,
        contexts::EditedDocument,
        run_edited_document_handlers,
        will_handle_edited_document,
    }

    handler! {
        /// Adds a new handler for edited locations.
        edited_location_handlers,
        edited_location,
        contexts::EditedLocation,
        run_edited_location_handlers,
        will_handle_edited_location,
    }

    handler! {
        /// Adds a new handler for edited photos.
        edited_photo_handlers,
        edited_photo,
        contexts::EditedPhoto,
        run_edited_photo_handlers,
        will_handle_edited_photo,
    }

    handler! {
        /// Adds a new handler for edited text messages.
        edited_text_handlers,
        edited_text,
        contexts::EditedText,
        run_edited_text_handlers,
        will_handle_edited_text,
    }

    handler! {
        /// Adds a new handler for edited videos.
        edited_video_handlers,
        edited_video,
        contexts::EditedVideo,
        run_edited_video_handlers,
        will_handle_edited_video,
    }

    handler! {
        /// Adds a new handler for games.
        game_handlers,
        game,
        contexts::Game,
        run_game_handlers,
        will_handle_game,
    }

    handler! {
        /// Adds a new handler for left members.
        left_member_handlers,
        left_member,
        contexts::LeftMember,
        run_left_member_handlers,
        will_handle_left_member,
    }

    handler! {
        /// Adds a new handler for locations.
        location_handlers,
        location,
        contexts::Location,
        run_location_handlers,
        will_handle_location,
    }

    handler! {
        /// Adds a new handler for migrations.
        migration_handlers,
        migration,
        contexts::Migration,
        run_migration_handlers,
        will_handle_migration,
    }

    handler! {
        /// Adds a new handler for new chat photos.
        new_chat_photo_handlers,
        new_chat_photo,
        contexts::NewChatPhoto,
        run_new_chat_photo_handlers,
        will_handle_new_chat_photo,
    }

    handler! {
        /// Adds a new handler for new chat titles.
        new_chat_title_handlers,
        new_chat_title,
        contexts::NewChatTitle,
        run_new_chat_title_handlers,
        will_handle_new_chat_title,
    }

    handler! {
        /// Adds a new handler for new members.
        new_members_handlers,
        new_members,
        contexts::NewMembers,
        run_new_members_handlers,
        will_handle_new_members,
    }

    handler! {
        /// Adds a new handler for photos.
        photo_handlers,
        photo,
        contexts::Photo,
        run_photo_handlers,
        will_handle_photo,
    }

    handler! {
        /// Adds a new handler for pinned messages.
        pinned_message_handlers,
        pinned_message,
        contexts::PinnedMessage,
        run_pinned_message_handlers,
        will_handle_pinned_message,
    }

    handler! {
        /// Adds a new handler for poll messages.
        poll_handlers,
        poll,
        contexts::Poll,
        run_poll_handlers,
        will_handle_poll,
    }

    handler! {
        /// Adds a new handler for polling errors.
        polling_error_handlers,
        polling_error,
        methods::DeliveryError,
        run_polling_error_handlers,
    }

    handler! {
        /// Adds a new handler for stickers.
        sticker_handlers,
        sticker,
        contexts::Sticker,
        run_sticker_handlers,
        will_handle_sticker,
    }

    handler! {
        /// Adds a new handler for text messages.
        text_handlers,
        text,
        contexts::Text,
        run_text_handlers,
        will_handle_text,
    }

    /// Adds a new handler for unhandled updates.
    pub fn unhandled(
        &mut self,
        handler: impl FnMut(&contexts::Unhandled) + Send + Sync + 'static,
    ) {
        self.unhandled_handlers.push(Mutex::new(Box::new(handler)))
    }

    fn will_handle_unhandled(&self) -> bool {
        !self.unhandled_handlers.is_empty()
    }

    fn run_unhandled_handlers(
        &self,
        mock_bot: Arc<MockBot>,
        update: UpdateKind,
    ) {
        let context = contexts::Unhandled::new(mock_bot, update);

        for handler in &self.unhandled_handlers {
            (&mut *handler.lock().unwrap())(&context);
        }
    }

    handler! {
        /// Adds a new handler for new states of polls.
        updated_poll_handlers,
        updated_poll,
        contexts::UpdatedPoll,
        run_updated_poll_handlers,
        will_handle_updated_poll,
    }

    handler! {
        /// Adds a new handler for venues.
        venue_handlers,
        venue,
        contexts::Venue,
        run_venue_handlers,
        will_handle_venue,
    }

    handler! {
        /// Adds a new handler for videos.
        video_handlers,
        video,
        contexts::Video,
        run_video_handlers,
        will_handle_video,
    }

    handler! {
        /// Adds a new handler for video notes.
        video_note_handlers,
        video_note,
        contexts::VideoNote,
        run_video_note_handlers,
        will_handle_video_note,
    }

    handler! {
        /// Adds a new handler for voice messages.
        voice_handlers,
        voice,
        contexts::Voice,
        run_voice_handlers,
        will_handle_voice,
    }

    fn handle_update(&self, update: types::Update) {
        let mock_bot = Arc::new(self.mock());
        let update_context =
            contexts::Update::new(Arc::clone(&mock_bot), update.id);

        self.run_before_update_handlers(&update_context);

        match update.kind {
            UpdateKind::Message(message) | UpdateKind::ChannelPost(message) => {
                self.handle_message_update(mock_bot, message);
            }
            UpdateKind::EditedMessage(message)
            | UpdateKind::EditedChannelPost(message) => {
                self.handle_message_edit_update(mock_bot, message);
            }
            UpdateKind::Poll(poll) => {
                if self.will_handle_updated_poll() {
                    let context =
                        contexts::UpdatedPoll::new(Arc::clone(&mock_bot), poll);

                    self.run_updated_poll_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let update = UpdateKind::Poll(poll);

                    self.run_unhandled_handlers(mock_bot, update);
                }
            }
            update @ UpdateKind::Unknown => {
                self.run_unhandled_handlers(mock_bot, update);
            }
        }

        self.run_after_update_handlers(&update_context);
    }

    #[allow(clippy::cognitive_complexity)]
    fn handle_message_update(
        &self,
        mock_bot: Arc<MockBot>,
        message: types::Message,
    ) {
        let (data, kind) = message.split();

        match kind {
            MessageKind::Text(text) => {
                if is_command(&text) {
                    let (command, username) = parse_command(&text);

                    if !self.is_for_this_bot(username) {
                        return;
                    }

                    let command = Box::leak(Box::new(command.to_string()));

                    if self.will_handle_command(command) {
                        let text = trim_command(text);

                        let context = contexts::Text::new(mock_bot, data, text);

                        self.run_command_handlers(command, &context);
                    } else if self.will_handle_unhandled() {
                        let kind = MessageKind::Text(text);
                        let message = Message::new(data, kind);
                        let update = UpdateKind::Message(message);

                        self.run_unhandled_handlers(mock_bot, update);
                    }
                } else if self.will_handle_text() {
                    let context = contexts::Text::new(mock_bot, data, text);

                    self.run_text_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Text(text);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(mock_bot, update);
                }
            }
            MessageKind::Poll(poll) => {
                if self.will_handle_poll() {
                    let context = contexts::Poll::new(mock_bot, data, poll);

                    self.run_poll_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Poll(poll);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(mock_bot, update);
                }
            }
            MessageKind::Photo(photo, caption, media_group_id) => {
                if self.will_handle_photo() {
                    let context = contexts::Photo::new(
                        mock_bot,
                        data,
                        photo,
                        caption,
                        media_group_id,
                    );

                    self.run_photo_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind =
                        MessageKind::Photo(photo, caption, media_group_id);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(mock_bot, update);
                }
            }
            MessageKind::Pinned(message) => {
                if self.will_handle_pinned_message() {
                    let context =
                        contexts::PinnedMessage::new(mock_bot, data, *message);

                    self.run_pinned_message_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Pinned(message);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(mock_bot, update);
                }
            }
            MessageKind::Sticker(sticker) => {
                if self.will_handle_sticker() {
                    let context =
                        contexts::Sticker::new(mock_bot, data, sticker);

                    self.run_sticker_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Sticker(sticker);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(mock_bot, update);
                }
            }
            MessageKind::Venue(venue) => {
                if self.will_handle_venue() {
                    let context = contexts::Venue::new(mock_bot, data, venue);

                    self.run_venue_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Venue(venue);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(mock_bot, update);
                }
            }
            MessageKind::Video(video, caption, media_group_id) => {
                if self.will_handle_video() {
                    let context = contexts::Video::new(
                        mock_bot,
                        data,
                        video,
                        caption,
                        media_group_id,
                    );

                    self.run_video_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind =
                        MessageKind::Video(video, caption, media_group_id);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(mock_bot, update);
                }
            }
            MessageKind::VideoNote(video_note) => {
                if self.will_handle_video_note() {
                    let context =
                        contexts::VideoNote::new(mock_bot, data, video_note);

                    self.run_video_note_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::VideoNote(video_note);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(mock_bot, update);
                }
            }
            MessageKind::Voice(voice, caption) => {
                if self.will_handle_voice() {
                    let context =
                        contexts::Voice::new(mock_bot, data, voice, caption);

                    self.run_voice_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Voice(voice, caption);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(mock_bot, update);
                }
            }
            MessageKind::Audio(audio, caption) => {
                if self.will_handle_audio() {
                    let context =
                        contexts::Audio::new(mock_bot, data, audio, caption);

                    self.run_audio_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Audio(audio, caption);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(mock_bot, update);
                }
            }
            MessageKind::Animation(animation, caption) => {
                if self.will_handle_animation() {
                    let context = contexts::Animation::new(
                        mock_bot, data, animation, caption,
                    );

                    self.run_animation_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Animation(animation, caption);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(mock_bot, update);
                }
            }
            kind @ MessageKind::ChatPhotoDeleted => {
                if self.will_handle_deleted_chat_photo() {
                    let context =
                        contexts::DeletedChatPhoto::new(mock_bot, data);

                    self.run_deleted_chat_photo_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(mock_bot, update);
                }
            }
            MessageKind::Document(document, caption) => {
                if self.will_handle_document() {
                    let context = contexts::Document::new(
                        mock_bot, data, document, caption,
                    );

                    self.run_document_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Document(document, caption);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(mock_bot, update);
                }
            }
            MessageKind::Game(game) => {
                if self.will_handle_game() {
                    let context = contexts::Game::new(mock_bot, data, game);

                    self.run_game_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Game(game);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(mock_bot, update);
                }
            }
            MessageKind::LeftChatMember(member) => {
                if self.will_handle_left_member() {
                    let context =
                        contexts::LeftMember::new(mock_bot, data, member);

                    self.run_left_member_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::LeftChatMember(member);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(mock_bot, update);
                }
            }
            MessageKind::Location(location) => {
                if self.will_handle_location() {
                    let context =
                        contexts::Location::new(mock_bot, data, location);

                    self.run_location_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Location(location);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(mock_bot, update);
                }
            }
            MessageKind::MigrateTo(..) => (), // ignored on purpose
            MessageKind::MigrateFrom(old_id) => {
                if self.will_handle_migration() {
                    let context =
                        contexts::Migration::new(mock_bot, data, old_id);

                    self.run_migration_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::MigrateFrom(old_id);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(mock_bot, update);
                }
            }
            MessageKind::NewChatPhoto(photo) => {
                if self.will_handle_new_chat_photo() {
                    let context =
                        contexts::NewChatPhoto::new(mock_bot, data, photo);

                    self.run_new_chat_photo_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::NewChatPhoto(photo);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(mock_bot, update);
                }
            }
            MessageKind::NewChatTitle(title) => {
                if self.will_handle_new_chat_title() {
                    let context =
                        contexts::NewChatTitle::new(mock_bot, data, title);

                    self.run_new_chat_title_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::NewChatTitle(title);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(mock_bot, update);
                }
            }
            MessageKind::NewChatMembers(members) => {
                if self.will_handle_new_members() {
                    let context =
                        contexts::NewMembers::new(mock_bot, data, members);

                    self.run_new_members_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::NewChatMembers(members);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(mock_bot, update);
                }
            }
            MessageKind::Contact(contact) => {
                if self.will_handle_contact() {
                    let context =
                        contexts::Contact::new(mock_bot, data, contact);

                    self.run_contact_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Contact(contact);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(mock_bot, update);
                }
            }
            kind @ MessageKind::GroupCreated => {
                if self.will_handle_created_group() {
                    let context = contexts::CreatedGroup::new(mock_bot, data);

                    self.run_created_group_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(mock_bot, update);
                }
            }
            MessageKind::SupergroupCreated | MessageKind::ChannelCreated => {
                unreachable!(
                "\n[tbot] Expected a `{supergroup,channel}_created` update to \
                never exist\n",
            )
            }
            _ if self.will_handle_unhandled() => {
                let message = Message::new(data, kind);
                let update = UpdateKind::Message(message);
                self.run_unhandled_handlers(mock_bot, update);
            }
            _ => (),
        }
    }

    fn handle_message_edit_update(
        &self,
        mock_bot: Arc<MockBot>,
        message: types::Message,
    ) {
        let (data, kind) = message.split();
        let edit_date = data.edit_date.expect(
            "\n[tbot] Expected `edit_date` to exist on an edited message\n",
        );

        match kind {
            MessageKind::Animation(animation, caption) => {
                if self.will_handle_edited_animation() {
                    let context = contexts::EditedAnimation::new(
                        mock_bot, data, edit_date, animation, caption,
                    );

                    self.run_edited_animation_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Animation(animation, caption);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(mock_bot, update);
                }
            }
            MessageKind::Audio(audio, caption) => {
                if self.will_handle_edited_audio() {
                    let context = contexts::EditedAudio::new(
                        mock_bot, data, edit_date, audio, caption,
                    );

                    self.run_edited_audio_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Audio(audio, caption);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(mock_bot, update);
                }
            }
            MessageKind::Document(document, caption) => {
                if self.will_handle_edited_document() {
                    let context = contexts::EditedDocument::new(
                        mock_bot, data, edit_date, document, caption,
                    );

                    self.run_edited_document_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Document(document, caption);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(mock_bot, update);
                }
            }
            MessageKind::Location(location) => {
                if self.will_handle_edited_location() {
                    let context = contexts::EditedLocation::new(
                        mock_bot, data, edit_date, location,
                    );

                    self.run_edited_location_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Location(location);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(mock_bot, update);
                }
            }
            MessageKind::Photo(photo, caption, media_group_id) => {
                if self.will_handle_edited_photo() {
                    let context = contexts::EditedPhoto::new(
                        mock_bot,
                        data,
                        edit_date,
                        photo,
                        caption,
                        media_group_id,
                    );

                    self.run_edited_photo_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind =
                        MessageKind::Photo(photo, caption, media_group_id);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(mock_bot, update);
                }
            }
            MessageKind::Text(text) => {
                if is_command(&text) {
                    let (command, username) = parse_command(&text);

                    if !self.is_for_this_bot(username) {
                        return;
                    }

                    let command = Box::leak(Box::new(command.to_string()));

                    if self.will_handle_edited_command(command) {
                        let text = trim_command(text);

                        let context = contexts::EditedText::new(
                            mock_bot, data, edit_date, text,
                        );

                        self.run_edited_command_handlers(command, &context);
                    } else if self.will_handle_unhandled() {
                        let kind = MessageKind::Text(text);
                        let message = Message::new(data, kind);
                        let update = UpdateKind::EditedMessage(message);

                        self.run_unhandled_handlers(mock_bot, update);
                    }
                } else if self.will_handle_edited_text() {
                    let context = contexts::EditedText::new(
                        mock_bot, data, edit_date, text,
                    );

                    self.run_edited_text_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Text(text);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::EditedMessage(message);

                    self.run_unhandled_handlers(mock_bot, update);
                }
            }
            MessageKind::Video(video, caption, media_group_id) => {
                if self.will_handle_edited_video() {
                    let context = contexts::EditedVideo::new(
                        mock_bot,
                        data,
                        edit_date,
                        video,
                        caption,
                        media_group_id,
                    );

                    self.run_edited_video_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind =
                        MessageKind::Video(video, caption, media_group_id);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(mock_bot, update);
                }
            }
            MessageKind::Poll(_) => unreachable!(
                "\n[tbot] Unexpected poll as an edited message update\n"
            ),
            MessageKind::NewChatMembers(..)
            | MessageKind::LeftChatMember(..)
            | MessageKind::ChatPhotoDeleted
            | MessageKind::NewChatPhoto(..)
            | MessageKind::NewChatTitle(..)
            | MessageKind::GroupCreated
            | MessageKind::SupergroupCreated
            | MessageKind::ChannelCreated
            | MessageKind::Pinned(..)
            | MessageKind::MigrateTo(..)
            | MessageKind::MigrateFrom(..) => unreachable!(
                "\n[tbot]\nExpected service messages not to be edited\n"
            ),
            _ if self.will_handle_unhandled() => {
                let message = Message::new(data, kind);
                let update = UpdateKind::EditedMessage(message);
                self.run_unhandled_handlers(mock_bot, update)
            }
            _ => (),
        }
    }

    fn is_for_this_bot(&self, username: Option<&str>) -> bool {
        if let Some(username) = username {
            self.username.as_ref().map(|x| x == &username) == Some(true)
        } else {
            true
        }
    }
}

impl crate::Sealed for Bot {}

impl Methods<'_> for Bot {
    fn token(&self) -> &str {
        &self.token
    }

    #[cfg(feature = "proxy")]
    fn get_proxy(&self) -> Option<proxy::Proxy> {
        self.proxy.clone()
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
/// let mut bot = tbot::bot!("BOT_TOKEN");
///
/// bot.text(|_| ());
/// ```
#[macro_export]
macro_rules! bot {
    ($var:literal) => {{
        let token = env!($var).to_string();
        $crate::Bot::new(token)
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

fn is_command(text: &types::Text) -> bool {
    text.entities.get(0).map(|entity| {
        entity.kind == types::MessageEntityKind::BotCommand
            && entity.offset == 0
    }) == Some(true)
}

fn parse_command(text: &types::Text) -> (&str, Option<&str>) {
    let mut iter = text.text.split_whitespace().next().unwrap()[1..].split('@');

    let command = iter.next().unwrap();
    let username = iter.next();

    (command, username)
}

fn trim_command(text: types::Text) -> types::Text {
    let mut entities = text.entities.into_iter();
    let command_entity = entities.next().unwrap();
    let old_length = text.text.chars().count();

    let text: String = text
        .text
        .chars()
        .skip(command_entity.length)
        .skip_while(|x| x.is_whitespace())
        .collect();
    let new_length = text.chars().count();

    let entities = entities
        .map(|entity| types::MessageEntity {
            kind: entity.kind,
            length: entity.length,
            offset: entity.offset - (old_length - new_length),
        })
        .collect();

    types::Text {
        text,
        entities,
    }
}

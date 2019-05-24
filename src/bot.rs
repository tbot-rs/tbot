use super::*;
use std::{
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
type DocumentHandler = Handler<contexts::Document>;
type EditedTextHandler = Handler<contexts::EditedText>;
type GameHandler = Handler<contexts::Game>;
type LocationHandler = Handler<contexts::Location>;
type PhotoHandler = Handler<contexts::Photo>;
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

/// Represents a bot and provides convenient methods to work with the API.
pub struct Bot {
    token: Arc<String>,
    #[cfg(feature = "proxy")]
    proxy: Option<proxy::Proxy>,
    after_update_handlers: Handlers<UpdateHandler>,
    animation_handlers: Handlers<AnimationHandler>,
    audio_handlers: Handlers<AudioHandler>,
    before_update_handlers: Handlers<UpdateHandler>,
    contact_handlers: Handlers<ContactHandler>,
    document_handlers: Handlers<DocumentHandler>,
    edited_text_handlers: Handlers<EditedTextHandler>,
    game_handlers: Handlers<GameHandler>,
    location_handlers: Handlers<LocationHandler>,
    photo_handlers: Handlers<PhotoHandler>,
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
}

impl Bot {
    /// Creates a new `Bot`.
    pub fn new(token: String) -> Self {
        Self {
            token: Arc::new(token),
            #[cfg(feature = "proxy")]
            proxy: None,
            after_update_handlers: Vec::new(),
            animation_handlers: Vec::new(),
            audio_handlers: Vec::new(),
            before_update_handlers: Vec::new(),
            contact_handlers: Vec::new(),
            document_handlers: Vec::new(),
            edited_text_handlers: Vec::new(),
            game_handlers: Vec::new(),
            location_handlers: Vec::new(),
            photo_handlers: Vec::new(),
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
        }
    }

    /// Constructs a new `Bot`, extracting the token from the environment at
    /// _runtime_.
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

    /// Starts configuring polling.
    pub const fn polling<'a>(self) -> Polling<'a> {
        Polling::new(self)
    }

    /// Starts configuring webhook. See our [wiki] to learn how to use webhook
    /// with `tbot`.
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

    /// Creates a new [`MockBot`] based on this bot.
    ///
    /// [`MockBot`]: ./struct.MockBot.html
    pub fn mock(&self) -> MockBot {
        MockBot::new(
            Arc::clone(&self.token),
            #[cfg(feature = "proxy")]
            self.proxy.clone(),
        )
    }

    handler! {
        after_update_handlers,
        after_update,
        contexts::Update,
        run_after_update_handlers,
    }

    handler! {
        animation_handlers,
        animation,
        contexts::Animation,
        run_animation_handlers,
        will_handle_animation,
    }

    handler! {
        audio_handlers,
        audio,
        contexts::Audio,
        run_audio_handlers,
        will_handle_audio,
    }

    handler! {
        before_update_handlers,
        before_update,
        contexts::Update,
        run_before_update_handlers,
    }

    handler! {
        contact_handlers,
        contact,
        contexts::Contact,
        run_contact_handlers,
        will_handle_contact,
    }

    handler! {
        document_handlers,
        document,
        contexts::Document,
        run_document_handlers,
        will_handle_document,
    }

    handler! {
        edited_text_handlers,
        edited_text,
        contexts::EditedText,
        run_edited_text_handlers,
        will_handle_edited_text,
    }

    handler! {
        game_handlers,
        game,
        contexts::Game,
        run_game_handlers,
        will_handle_game,
    }
    handler! {
        location_handlers,
        location,
        contexts::Location,
        run_location_handlers,
        will_handle_location,
    }

    handler! {
        photo_handlers,
        photo,
        contexts::Photo,
        run_photo_handlers,
        will_handle_photo,
    }

    handler! {
        poll_handlers,
        poll,
        contexts::Poll,
        run_poll_handlers,
        will_handle_poll,
    }

    handler! {
        polling_error_handlers,
        polling_error,
        methods::DeliveryError,
        run_polling_error_handlers,
    }

    handler! {
        sticker_handlers,
        sticker,
        contexts::Sticker,
        run_sticker_handlers,
        will_handle_sticker,
    }

    handler! {
        text_handlers,
        text,
        contexts::Text,
        run_text_handlers,
        will_handle_text,
    }

    /// Adds a new handler for unhandled events.
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
        updated_poll_handlers,
        updated_poll,
        contexts::UpdatedPoll,
        run_updated_poll_handlers,
        will_handle_updated_poll,
    }

    handler! {
        venue_handlers,
        venue,
        contexts::Venue,
        run_venue_handlers,
        will_handle_venue,
    }

    handler! {
        video_handlers,
        video,
        contexts::Video,
        run_video_handlers,
        will_handle_video,
    }

    handler! {
        video_note_handlers,
        video_note,
        contexts::VideoNote,
        run_video_note_handlers,
        will_handle_video_note,
    }

    handler! {
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

    #[allow(clippy::cyclomatic_complexity)]
    fn handle_message_update(
        &self,
        mock_bot: Arc<MockBot>,
        message: types::Message,
    ) {
        let (data, kind) = message.split();

        match kind {
            MessageKind::Text(text) => {
                if !text.text.starts_with('/') {
                    if self.will_handle_text() {
                        let context = contexts::Text::new(mock_bot, data, text);

                        self.run_text_handlers(&context);
                    } else if self.will_handle_unhandled() {
                        let kind = MessageKind::Text(text);
                        let message = Message::new(data, kind);
                        let update = UpdateKind::Message(message);

                        self.run_unhandled_handlers(mock_bot, update);
                    }
                } // TODO: command handlers
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
            MessageKind::Text(text) => {
                if !text.text.starts_with('/') {
                    if self.will_handle_edited_text() {
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
            }
            MessageKind::Poll(_) => unreachable!(
                "\n[tbot] Unexpected poll as an edited message update\n"
            ),
            _ if self.will_handle_unhandled() => {
                let message = Message::new(data, kind);
                let update = UpdateKind::EditedMessage(message);
                self.run_unhandled_handlers(mock_bot, update)
            }
            _ => (),
        }
    }
}

impl Methods<'_> for Bot {
    fn token(&self) -> &str {
        &self.token
    }

    #[cfg(feature = "proxy")]
    fn get_proxy(&self) -> Option<proxy::Proxy> {
        self.proxy.clone()
    }
}

/// Constructs a new `Bot`, extracting the token from the environment at
/// _compile time_.
///
/// If you need to extract the token at _runtime_, use [`Bot::from_env`].
///
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

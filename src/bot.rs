use super::*;
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use {
    contexts::*,
    methods::GetUpdates,
    types::{Message, MessageKind, UpdateKind},
};

mod mock_bot;
mod polling;
mod webhook;

pub use {mock_bot::*, polling::*, webhook::*};

type Handlers<T> = Vec<Mutex<Box<T>>>;

// Wish trait alises came out soon
type PollingErrorHandler = dyn FnMut(&methods::DeliveryError) + Send + Sync;
type UpdateHandler = dyn FnMut(&UpdateContext) + Send + Sync;
type TextHandler = dyn FnMut(&TextContext) + Send + Sync;
type EditedTextHandler = dyn FnMut(&EditedTextContext) + Send + Sync;
type PollHandler = dyn FnMut(&PollContext) + Send + Sync;
type PhotoHandler = dyn FnMut(&PhotoContext) + Send + Sync;
type VideoHandler = dyn FnMut(&VideoContext) + Send + Sync;
type UpdatedPollHandler = dyn FnMut(&UpdatedPollContext) + Send + Sync;
type UnhandledHandler = dyn FnMut(&UnhandledContext) + Send + Sync;
type VoiceHandler = dyn FnMut(&VoiceContext) + Send + Sync;

/// Represents a bot and provides convenient methods to work with the API.
pub struct Bot {
    token: Arc<String>,
    polling_error_handlers: Handlers<PollingErrorHandler>,
    before_update_handlers: Handlers<UpdateHandler>,
    after_update_handlers: Handlers<UpdateHandler>,
    text_handlers: Handlers<TextHandler>,
    edited_text_handlers: Handlers<EditedTextHandler>,
    poll_handlers: Handlers<PollHandler>,
    photo_handlers: Handlers<PhotoHandler>,
    updated_poll_handlers: Handlers<UpdatedPollHandler>,
    unhandled_handlers: Handlers<UnhandledHandler>,
    video_handlers: Handlers<VideoHandler>,
    voice_handlers: Handlers<VoiceHandler>,
    #[cfg(feature = "proxy")]
    proxy: Option<proxy::Proxy>,
}

impl Bot {
    /// Creates a new `Bot`.
    pub fn new(token: String) -> Self {
        Self {
            token: Arc::new(token),
            polling_error_handlers: Vec::new(),
            before_update_handlers: Vec::new(),
            after_update_handlers: Vec::new(),
            text_handlers: Vec::new(),
            edited_text_handlers: Vec::new(),
            poll_handlers: Vec::new(),
            photo_handlers: Vec::new(),
            updated_poll_handlers: Vec::new(),
            unhandled_handlers: Vec::new(),
            video_handlers: Vec::new(),
            voice_handlers: Vec::new(),
            #[cfg(feature = "proxy")]
            proxy: None,
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

    /// Adds a new handler for errors that happened while polling.
    ///
    /// If no polling error handler is set and such an error occurs, `tbot` will
    /// panic printing the error.
    pub fn polling_error(
        &mut self,
        handler: impl FnMut(&methods::DeliveryError) + Send + Sync + 'static,
    ) {
        self.polling_error_handlers.push(Mutex::new(Box::new(handler)))
    }

    /// Adds a new handler for all updates run before the specialized updates.
    pub fn before_update(
        &mut self,
        handler: impl FnMut(&UpdateContext) + Send + Sync + 'static,
    ) {
        self.before_update_handlers.push(Mutex::new(Box::new(handler)))
    }

    /// Adds a new handler for all updates run after the specialized updates.
    pub fn after_update(
        &mut self,
        handler: impl FnMut(&UpdateContext) + Send + Sync + 'static,
    ) {
        self.after_update_handlers.push(Mutex::new(Box::new(handler)))
    }

    /// Adds a new handler for text messages.
    pub fn text(
        &mut self,
        handler: impl FnMut(&TextContext) + Send + Sync + 'static,
    ) {
        self.text_handlers.push(Mutex::new(Box::new(handler)))
    }

    /// Adds a new handler for edited text messages.
    pub fn edited_text(
        &mut self,
        handler: impl FnMut(&EditedTextContext) + Send + Sync + 'static,
    ) {
        self.edited_text_handlers.push(Mutex::new(Box::new(handler)))
    }

    /// Adds a new handler for poll messages.
    pub fn poll(
        &mut self,
        handler: impl FnMut(&PollContext) + Send + Sync + 'static,
    ) {
        self.poll_handlers.push(Mutex::new(Box::new(handler)))
    }

    /// Adds a new handler for photo messages.
    pub fn photo(
        &mut self,
        handler: impl FnMut(&PhotoContext) + Send + Sync + 'static,
    ) {
        self.photo_handlers.push(Mutex::new(Box::new(handler)))
    }

    /// Adds a new handler for new states of polls
    pub fn updated_poll(
        &mut self,
        handler: impl FnMut(&UpdatedPollContext) + Send + Sync + 'static,
    ) {
        self.updated_poll_handlers.push(Mutex::new(Box::new(handler)))
    }

    /// Adds a new handler for unhandled events.
    pub fn unhandled(
        &mut self,
        handler: impl FnMut(&UnhandledContext) + Send + Sync + 'static,
    ) {
        self.unhandled_handlers.push(Mutex::new(Box::new(handler)))
    }

    /// Adds a new handler for video messages.
    pub fn video(
        &mut self,
        handler: impl FnMut(&VideoContext) + Send + Sync + 'static,
    ) {
        self.video_handlers.push(Mutex::new(Box::new(handler)))
    }

    /// Adds a new handler for voice messages.
    pub fn voice(
        &mut self,
        handler: impl FnMut(&VoiceContext) + Send + Sync + 'static,
    ) {
        self.voice_handlers.push(Mutex::new(Box::new(handler)))
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

    fn handle_update(&self, update: types::Update) {
        let mock_bot = Arc::new(self.mock());
        let update_context =
            UpdateContext::new(Arc::clone(&mock_bot), update.id);

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
                        UpdatedPollContext::new(Arc::clone(&mock_bot), poll);

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
                        let context = TextContext::new(mock_bot, data, text);

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
                    let context = PollContext::new(mock_bot, data, poll);

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
                    let context = PhotoContext::new(
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
            MessageKind::Video(video, caption, media_group_id) => {
                if self.will_handle_video() {
                    let context = VideoContext::new(
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
            MessageKind::Voice(voice, caption) => {
                if self.will_handle_voice() {
                    let context =
                        VoiceContext::new(mock_bot, data, voice, caption);

                    self.run_voice_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Voice(voice, caption);
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

        match kind {
            MessageKind::Text(text) => {
                if !text.text.starts_with('/') {
                    if self.will_handle_edited_text() {
                        let context =
                            EditedTextContext::new(mock_bot, data, text);

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

    fn run_polling_error_handlers(&self, error: &methods::DeliveryError) {
        if self.polling_error_handlers.is_empty() {
            panic!("\n[tbot] Unhandled polling error: {:#?}\n", error);
        }

        for handler in &self.polling_error_handlers {
            (&mut *handler.lock().unwrap())(error);
        }
    }

    fn run_before_update_handlers(&self, context: &UpdateContext) {
        for handler in &self.before_update_handlers {
            (&mut *handler.lock().unwrap())(context);
        }
    }

    fn run_after_update_handlers(&self, context: &UpdateContext) {
        for handler in &self.after_update_handlers {
            (&mut *handler.lock().unwrap())(context);
        }
    }

    fn will_handle_text(&self) -> bool {
        !self.text_handlers.is_empty()
    }

    fn run_text_handlers(&self, context: &TextContext) {
        for handler in &self.text_handlers {
            (&mut *handler.lock().unwrap())(context);
        }
    }

    fn will_handle_edited_text(&self) -> bool {
        !self.edited_text_handlers.is_empty()
    }

    fn run_edited_text_handlers(&self, context: &EditedTextContext) {
        for handler in &self.edited_text_handlers {
            (&mut *handler.lock().unwrap())(context);
        }
    }

    fn will_handle_poll(&self) -> bool {
        !self.poll_handlers.is_empty()
    }

    fn run_poll_handlers(&self, context: &PollContext) {
        for handler in &self.poll_handlers {
            (&mut *handler.lock().unwrap())(context);
        }
    }

    fn will_handle_photo(&self) -> bool {
        !self.photo_handlers.is_empty()
    }

    fn run_photo_handlers(&self, context: &PhotoContext) {
        for handler in &self.photo_handlers {
            (&mut *handler.lock().unwrap())(context);
        }
    }

    fn will_handle_updated_poll(&self) -> bool {
        !self.updated_poll_handlers.is_empty()
    }

    fn run_updated_poll_handlers(&self, context: &UpdatedPollContext) {
        for handler in &self.updated_poll_handlers {
            (&mut *handler.lock().unwrap())(context);
        }
    }

    fn will_handle_unhandled(&self) -> bool {
        !self.unhandled_handlers.is_empty()
    }

    fn run_unhandled_handlers(
        &self,
        mock_bot: Arc<MockBot>,
        update: UpdateKind,
    ) {
        let context = UnhandledContext::new(mock_bot, update);

        for handler in &self.unhandled_handlers {
            (&mut *handler.lock().unwrap())(&context);
        }
    }

    fn will_handle_video(&self) -> bool {
        !self.video_handlers.is_empty()
    }

    fn run_video_handlers(&self, context: &VideoContext) {
        for handler in &self.video_handlers {
            (&mut *handler.lock().unwrap())(context);
        }
    }

    fn will_handle_voice(&self) -> bool {
        !self.voice_handlers.is_empty()
    }

    fn run_voice_handlers(&self, context: &VoiceContext) {
        for handler in &self.voice_handlers {
            (&mut *handler.lock().unwrap())(context);
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

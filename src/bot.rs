use super::*;
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use {
    contexts::*,
    methods::GetUpdates,
    types::{MessageKind, UpdateKind},
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
type UpdatedPollHandler = dyn FnMut(&UpdatedPollContext) + Send + Sync;

/// Represents a bot and provides convenient methods to work with the API.
pub struct Bot {
    token: Arc<String>,
    polling_error_handlers: Handlers<PollingErrorHandler>,
    before_update_handlers: Handlers<UpdateHandler>,
    after_update_handlers: Handlers<UpdateHandler>,
    text_handlers: Handlers<TextHandler>,
    edited_text_handlers: Handlers<EditedTextHandler>,
    poll_handlers: Handlers<PollHandler>,
    updated_poll_handlers: Handlers<UpdatedPollHandler>,
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
            updated_poll_handlers: Vec::new(),
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

    /// Adds a new handler for new states of polls
    pub fn updated_poll(
        &mut self,
        handler: impl FnMut(&UpdatedPollContext) + Send + Sync + 'static,
    ) {
        self.updated_poll_handlers.push(Mutex::new(Box::new(handler)))
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

        self.handle_before_update(&update_context);

        match update.kind {
            Some(UpdateKind::Message(message))
            | Some(UpdateKind::ChannelPost(message)) => {
                match message.kind {
                    MessageKind::Text(text) => {
                        if !text.text.starts_with('/')
                            && self.will_handle_text()
                        {
                            let context = TextContext::new(
                                Arc::clone(&mock_bot),
                                message.id,
                                message.from,
                                message.date,
                                message.chat,
                                message.forward,
                                message.reply_to.map(|message| *message),
                                text,
                            );

                            self.handle_text(&context);
                        }
                    }
                    MessageKind::Poll(poll) => {
                        if self.will_handle_poll() {
                            let context = PollContext::new(
                                Arc::clone(&mock_bot),
                                message.id,
                                message.from,
                                message.date,
                                message.chat,
                                message.forward,
                                message.reply_to.map(|message| *message),
                                poll,
                            );

                            self.handle_poll(&context);
                        }
                    }
                    _ => (), // TOOD
                }
            }
            Some(UpdateKind::EditedMessage(message))
            | Some(UpdateKind::EditedChannelPost(message)) => {
                let edit_date = message.edit_date.expect(
                    "\n[tbot] Edited message did not have the `edit_date` \
                     field\n",
                );

                match message.kind {
                    MessageKind::Text(text) => {
                        if !text.text.starts_with('/')
                            && self.will_handle_edited_text()
                        {
                            let context = EditedTextContext::new(
                                Arc::clone(&mock_bot),
                                message.id,
                                message.from,
                                message.date,
                                message.chat,
                                message.reply_to.map(|message| *message),
                                edit_date,
                                text,
                            );

                            self.handle_edited_text(&context);
                        }
                    }
                    MessageKind::Poll(_) => unreachable!(
                        "\n[tbot] Unexpected poll as an edited message update\n"
                    ),
                    _ => (), // TOOD
                }
            }
            Some(UpdateKind::Poll(poll)) => {
                if self.will_handle_updated_poll() {
                    let context =
                        UpdatedPollContext::new(Arc::clone(&mock_bot), poll);

                    self.handle_updated_poll(&context);
                }
            }
            _ => (), // TODO
        }

        self.handle_after_update(&update_context);
    }

    fn handle_polling_error(&self, error: &methods::DeliveryError) {
        if self.polling_error_handlers.is_empty() {
            panic!("\n[tbot] Unhandled polling error: {:#?}\n", error);
        }

        for handler in &self.polling_error_handlers {
            (&mut *handler.lock().unwrap())(error);
        }
    }

    fn handle_before_update(&self, context: &UpdateContext) {
        for handler in &self.before_update_handlers {
            (&mut *handler.lock().unwrap())(context);
        }
    }

    fn handle_after_update(&self, context: &UpdateContext) {
        for handler in &self.after_update_handlers {
            (&mut *handler.lock().unwrap())(context);
        }
    }

    fn will_handle_text(&self) -> bool {
        !self.text_handlers.is_empty()
    }

    fn handle_text(&self, context: &TextContext) {
        for handler in &self.text_handlers {
            (&mut *handler.lock().unwrap())(context);
        }
    }

    fn will_handle_edited_text(&self) -> bool {
        !self.edited_text_handlers.is_empty()
    }

    fn handle_edited_text(&self, context: &EditedTextContext) {
        for handler in &self.edited_text_handlers {
            (&mut *handler.lock().unwrap())(context);
        }
    }

    fn will_handle_poll(&self) -> bool {
        !self.poll_handlers.is_empty()
    }

    fn handle_poll(&self, context: &PollContext) {
        for handler in &self.poll_handlers {
            (&mut *handler.lock().unwrap())(context);
        }
    }

    fn will_handle_updated_poll(&self) -> bool {
        !self.updated_poll_handlers.is_empty()
    }

    fn handle_updated_poll(&self, context: &UpdatedPollContext) {
        for handler in &self.updated_poll_handlers {
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

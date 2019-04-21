use super::*;
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use {contexts::*, methods::GetUpdates, types::UpdateKind};

mod mock_bot;
mod polling;
mod webhook;

pub use {mock_bot::*, polling::*, webhook::*};

type Handlers<T> = Vec<Mutex<Box<T>>>;

// Wish trait alises came out soon
type PollingErrorHandler = dyn FnMut(&methods::DeliveryError) + Send + Sync;
type BeforeUpdateHandler = dyn FnMut(&types::Update) + Send + Sync;
type MessageHandler = dyn FnMut(&MessageContext) + Send + Sync;

/// Represents a bot and provides convenient methods to work with the API.
pub struct Bot {
    token: Arc<String>,
    polling_error_handlers: Handlers<PollingErrorHandler>,
    before_update_handlers: Handlers<BeforeUpdateHandler>,
    message_handlers: Handlers<MessageHandler>,
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
            message_handlers: Vec::new(),
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
    /// bot.on_message(|_| ());
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
        handler: impl FnMut(&types::Update) + Send + Sync + 'static,
    ) {
        self.before_update_handlers.push(Mutex::new(Box::new(handler)))
    }

    /// Adds a new text message handler.
    pub fn on_message(
        &mut self,
        handler: impl FnMut(&MessageContext) + Send + Sync + 'static,
    ) {
        self.message_handlers.push(Mutex::new(Box::new(handler)))
    }

    /// Starts configuring polling.
    pub fn polling<'a>(self) -> Polling<'a> {
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
        #[cfg(feature = "proxy")]
        {
            MockBot::new(self.token.clone(), self.proxy.clone())
        }

        #[cfg(not(feature = "proxy"))]
        MockBot::new(self.token.clone())
    }

    fn handle_update(&self, update: types::Update) {
        self.handle_before_update(&update);

        let mock_bot = Arc::new(self.mock());

        match update.kind {
            Some(UpdateKind::Message(mut message)) => {
                match MessageContext::try_new(mock_bot.clone(), message) {
                    Ok(context) => {
                        self.handle_message(&context);
                        return;
                    }
                    Err(original) => message = original,
                }
            }
            _ => (), // TODO
        }
    }

    fn handle_polling_error(&self, error: &methods::DeliveryError) {
        if self.polling_error_handlers.is_empty() {
            panic!("\n[tbot] Unhandled polling error: {:#?}\n", error);
        }

        for handler in &self.polling_error_handlers {
            (&mut *handler.lock().unwrap())(&error);
        }
    }

    fn handle_before_update(&self, update: &types::Update) {
        for handler in &self.before_update_handlers {
            (&mut *handler.lock().unwrap())(&update);
        }
    }

    fn handle_message(&self, context: &MessageContext) {
        for handler in &self.message_handlers {
            (&mut *handler.lock().unwrap())(&context);
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
/// bot.on_message(|_| ());
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

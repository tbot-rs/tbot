use self::methods::GetUpdates;
use super::*;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

mod polling;
pub use self::polling::*;

// Wish trait alises came out soon
type PollingErrorHandler = dyn FnMut(&methods::DeliveryError) + Send + Sync;
type BeforeUpdateHandler = dyn FnMut(&types::Update) + Send + Sync;

/// Represents a bot and provides convenient methods to work with the API.
pub struct Bot {
    token: String,
    polling_error_handlers: Vec<Mutex<Box<PollingErrorHandler>>>,
    before_update_handlers: Vec<Mutex<Box<BeforeUpdateHandler>>>,
}

impl Bot {
    /// Creates a new `Bot`.
    pub fn new(token: String) -> Self {
        Self {
            token,
            polling_error_handlers: Vec::new(),
            before_update_handlers: Vec::new(),
        }
    }

    /// Constructs a new `Bot`, getting the token from the environment.
    pub fn from_env(env_var: &'static str) -> Self {
        Self::new(std::env::var(env_var).unwrap_or_else(|_| {
            panic!("The bot's token in {} was not specified", env_var)
        }))
    }

    /// Adds a new handler for errors that happened while sending poll
    /// requests.
    pub fn on_polling_error<T>(&mut self, handler: T)
    where
        T: FnMut(&methods::DeliveryError) + Send + Sync + 'static,
    {
        self.polling_error_handlers.push(Mutex::new(Box::new(handler)))
    }

    /// Adds a new handler for all updates run before the specialized updates.
    pub fn before_update<T>(&mut self, handler: T)
    where
        T: FnMut(&types::Update) + Send + Sync + 'static,
    {
        self.before_update_handlers.push(Mutex::new(Box::new(handler)))
    }

    /// Starts configuring polling.
    pub fn polling<'a>(self) -> Polling<'a> {
        Polling::new(self)
    }

    /// Sets a proxy through which requests to Telegram will be sent.
    #[cfg(feature = "proxy")]
    pub fn set_proxy(&mut self, proxy: hyper_proxy::Proxy) {
        unimplemented!();
    }

    fn handle_update(&self, update: &types::Update) {
        self.handle_before_update(update);
    }

    fn handle_polling_error(&self, error: &methods::DeliveryError) {
        for handler in &self.polling_error_handlers {
            (&mut *handler.lock().unwrap())(&error);
        }
    }

    fn handle_before_update(&self, update: &types::Update) {
        for handler in &self.before_update_handlers {
            (&mut *handler.lock().unwrap())(&update);
        }
    }
}

impl Methods for Bot {
    fn token(&self) -> &str {
        &self.token
    }
}

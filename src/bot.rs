use self::methods::GetUpdates;
use super::*;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

mod polling;
pub use self::polling::*;

type PollingErrorHandler = dyn FnMut(&methods::DeliveryError) + Send + Sync;

/// Represents a bot and provides convenient methods to work with the API.
pub struct Bot {
    token: String,
    polling_error_handlers: Vec<Mutex<Box<PollingErrorHandler>>>,
}

impl Bot {
    /// Creates a new `Bot`.
    pub fn new(token: String) -> Self {
        Self {
            token,
            polling_error_handlers: Vec::new(),
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

    /// Starts configuring polling.
    pub fn polling<'a>(self) -> Polling<'a> {
        Polling::new(self)
    }

    /// Sets a proxy through which requests to Telegram will be sent.
    #[cfg(feature = "proxy")]
    pub fn set_proxy(&mut self, proxy: hyper_proxy::Proxy) {
        unimplemented!();
    }

    /// Constructs a new [`GetMe`] inferring `token`.
    ///
    /// [`GetMe`]: ./methods/struct.GetMe.html
    #[must_use]
    pub fn get_me(&self) -> methods::GetMe {
        methods::GetMe::new(&self.token)
    }

    /// Constructs a new [`SendMessage`] inferring `token`.
    ///
    /// [`SendMessage`]: ./methods/struct.SendMessage.html
    #[must_use]
    pub fn send_message<'a, 'b: 'a>(
        &'b self,
        chat_id: impl Into<types::ChatId<'a>>,
        text: &'a str,
    ) -> methods::SendMessage<'a> {
        methods::SendMessage::new(&self.token, chat_id, text)
    }

    /// Constructs a new [`ForwardMessage`] inferring `token`.
    ///
    /// [`ForwardMessage`]: ./methods/struct.ForwardMessage.html
    #[must_use]
    pub fn forward_message<'a, 'b: 'a>(
        &'b self,
        chat_id: impl Into<types::ChatId<'a>>,
        from_chat_id: &'a types::ChatId,
        message_id: u64,
    ) -> methods::ForwardMessage<'a> {
        methods::ForwardMessage::new(
            &self.token,
            chat_id,
            from_chat_id,
            message_id,
        )
    }

    /// Constructs a new [`SendLocation`] inferring `token`.
    ///
    /// [`SendLocation`]: ./methods/struct.SendLocation.html
    #[must_use]
    pub fn send_location<'a, 'b: 'a>(
        &'b self,
        chat_id: impl Into<types::ChatId<'a>>,
        position: (f64, f64),
    ) -> methods::SendLocation<'a> {
        methods::SendLocation::new(&self.token, chat_id, position)
    }

    /// Constructs a new [`EditInlineLocation`] inferring `token`.
    ///
    /// [`EditInlineLocation`]: ./methods/struct.EditInlineLocation.html
    #[must_use]
    pub fn edit_inline_location<'a, 'b: 'a>(
        &'b self,
        inline_message_id: u64,
        position: (f64, f64),
    ) -> methods::EditInlineLocation<'a> {
        methods::EditInlineLocation::new(
            &self.token,
            inline_message_id,
            position,
        )
    }

    /// Constructs a new [`EditMessageLocation`] inferring `token`.
    ///
    /// [`EditMessageLocation`]: ./methods/struct.EditMessageLocation.html
    #[must_use]
    pub fn edit_message_location<'a, 'b: 'a>(
        &'b self,
        chat_id: impl Into<types::ChatId<'b>>,
        message_id: u64,
        position: (f64, f64),
    ) -> methods::EditMessageLocation<'a> {
        methods::EditMessageLocation::new(
            &self.token,
            chat_id,
            message_id,
            position,
        )
    }

    /// Constructs a new [`StopInlineLocation`] inferring `token`.
    ///
    /// [`StopInlineLocation`]: ./methods/struct.StopInlineLocation.html
    #[must_use]
    pub fn stop_inline_location<'a, 'b: 'a>(
        &'b self,
        inline_message_id: u64,
    ) -> methods::StopInlineLocation<'a> {
        methods::StopInlineLocation::new(&self.token, inline_message_id)
    }

    /// Constructs a new [`StopMessageLocation`] inferring `token`.
    ///
    /// [`StopMessageLocation`]: ./methods/struct.StopMessageLocation.html
    #[must_use]
    pub fn stop_message_location<'a, 'b: 'a>(
        &'b self,
        chat_id: impl Into<types::ChatId<'b>>,
        message_id: u64,
    ) -> methods::StopMessageLocation<'a> {
        methods::StopMessageLocation::new(&self.token, chat_id, message_id)
    }

    /// Constructs a new [`SendVenue`] inferring `token`.
    ///
    /// [`SendVenue`]: ./methods/struct.SendVenue.html
    #[must_use]
    pub fn send_venue<'a, 'b: 'a>(
        &'b self,
        chat_id: impl Into<types::ChatId<'b>>,
        position: (f64, f64),
        title: &'b str,
        address: &'b str,
    ) -> methods::SendVenue<'a> {
        methods::SendVenue::new(&self.token, chat_id, position, title, address)
    }

    /// Constructs a new [`SendContact`] inferring `token`.
    ///
    /// [`SendContact`]: ./methods/struct.SendContact.html
    #[must_use]
    pub fn send_contact<'a, 'b: 'a>(
        &'b self,
        chat_id: impl Into<types::ChatId<'b>>,
        phone_number: &'b str,
        first_name: &'b str,
    ) -> methods::SendContact<'a> {
        methods::SendContact::new(
            &self.token,
            chat_id,
            phone_number,
            first_name,
        )
    }

    /// Constructs a new [`SendChatAction`] inferring `token`.
    ///
    /// [`SendChatAction`]: ./methods/struct.SendChatAction.html
    #[must_use]
    pub fn send_chat_action<'a, 'b: 'a>(
        &'b self,
        chat_id: impl Into<types::ChatId<'b>>,
        action: types::ChatAction,
    ) -> methods::SendChatAction<'a> {
        methods::SendChatAction::new(&self.token, chat_id, action)
    }

    fn handle_polling_error(&self, error: &methods::DeliveryError) {
        for handler in &self.polling_error_handlers {
            (&mut *handler.lock().unwrap())(&error);
        }
    }
}

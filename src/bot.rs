use super::*;

/// Represents a bot and provides convenient methods to work with the API.
pub struct Bot<'a> {
    token: &'a str,
}

impl<'bot> Bot<'bot> {
    /// Creates a new `Bot`.
    pub fn new<'a: 'bot>(token: &'a str) -> Bot<'bot> {
        Bot {
            token,
        }
    }

    /// Starts webhook.
    ///
    /// [setWebook]: https://core.telegram.org/bots/api#setwebhook
    pub fn start_webhook(
        &mut self,
        url: &str,
        certificate: &str,
        max_connections: u8,
    ) -> ! {
        unimplemented!();
    }

    /// Sets a proxy through which requests to Telegram will be sent.
    #[cfg(feature = "proxy")]
    pub fn set_proxy(&mut self, proxy: hyper_proxy::Proxy) {
        unimplemented!();
    }

    /// Creates [`GetMe`] inferring `token`.
    ///
    /// [`GetMe`]: ./methods/struct.GetMe.html
    #[must_use]
    pub fn get_me(&self) -> methods::GetMe {
        methods::GetMe::new(self.token)
    }

    /// Creates [`SendMessage`] inferring `token`.
    ///
    /// [`SendMessage`]: ./methods/struct.SendMessage.html
    #[must_use]
    pub fn send_message<'a, 'b: 'a>(
        &'b self,
        chat_id: impl Into<types::ChatId<'a>>,
        text: &'a str,
    ) -> methods::SendMessage<'a> {
        methods::SendMessage::new(self.token, chat_id, text)
    }

    /// Creates [`ForwardMessage`] inferring `token`.
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
            self.token,
            chat_id,
            from_chat_id,
            message_id,
        )
    }

    /// Constructs [`SendLocation`] inferring `token`.
    ///
    /// [`SendLocation`]: ./methods/struct.SendLocation.html
    #[must_use]
    pub fn send_location<'a, 'b: 'a>(
        &'b self,
        chat_id: impl Into<types::ChatId<'a>>,
        latitude: f64,
        longitude: f64,
    ) -> methods::SendLocation<'a> {
        methods::SendLocation::new(
            self.token,
            chat_id,
            latitude,
            longitude,
        )
    }
}

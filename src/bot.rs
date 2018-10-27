use super::*;

/// Represents a bot and provides convenient methods to work with the API.
pub struct Bot<'a> {
    token: &'a str,
}

impl<'a> Bot<'a> {
    /// Creates a new `Bot`.
    pub fn new<'b: 'a>(token: &'b str) -> Bot<'a> {
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
    pub fn send_message<'b, 'c: 'b>(
        &'c self,
        chat_id: &'b types::ChatId,
        text: &'b str,
    ) -> methods::SendMessage<'b> {
        methods::SendMessage::new(self.token, chat_id, text)
    }
}

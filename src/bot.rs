use super::*;

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
    /// # Panics
    ///
    /// Panics if `max_connections` is out of range `[1, 100]` as per the
    /// [API docs][setWebhook].
    ///
    /// [setWebook]: https://core.telegram.org/bots/api#setwebhook
    pub fn start_webhook(
        &mut self,
        url: &str,
        certificate: &str,
        max_connections: u8,
    ) -> ! {
        assert!(1 <= max_connections && max_connections <= 100);
        unimplemented!();
    }

    /// Sets a proxy throug which requests to Telegram's servers will be sent.
    #[cfg(feature = "proxy")]
    pub fn set_proxy(&mut self, proxy: hyper_proxy::Proxy) {
        unimplemented!();
    }

    pub fn get_me(&self) -> methods::GetMe {
        methods::GetMe::new(self.token)
    }
}

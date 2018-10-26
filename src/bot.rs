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
    /// [setWebook]: https://core.telegram.org/bots/api#setwebhook
    pub fn start_webhook(
        &mut self,
        url: &str,
        certificate: &str,
        max_connections: u8,
    ) -> ! {
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

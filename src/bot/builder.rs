use super::{Bot, InnerBot};
use crate::{connectors::Client, proxy::Proxy, token::Token};
use std::sync::Arc;

/// A builder for a [`Bot`] with advanced configuration.
///
/// [`Bot`]: ./struct.Bot.html
#[derive(Debug)]
#[must_use]
pub struct Builder(InnerBot);

impl Builder {
    /// Starts constructing a `Bot` with the provided token.
    pub fn with_string_token(token: String) -> Self {
        Self(InnerBot::new(Token(token), Client::https()))
    }

    /// Starts constructing a `Bot`, extracting the token from the provided
    /// environment variable.
    pub fn with_env_token(env_var: &'static str) -> Self {
        let token = std::env::var(env_var).unwrap_or_else(|_| {
            panic!("[tbot] Bot's token in {} was not specified", env_var)
        });

        Self::with_string_token(token)
    }

    /// Configures a proxy through which all the request will go.
    pub fn proxy(mut self, proxy: impl Into<Proxy>) -> Self {
        let proxy: Proxy = proxy.into();
        self.0.set_client(proxy.into());
        self
    }

    /// Finishes constructing the [`Bot`].
    ///
    /// [`Bot`]: ./struct.Bot.html
    pub fn build(self) -> Bot {
        Bot {
            inner: Arc::new(self.0),
        }
    }
}

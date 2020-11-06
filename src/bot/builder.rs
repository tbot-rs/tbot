use super::{Bot, InnerBot};
use crate::{
    connectors::Client, errors, methods::LogOut, proxy::Proxy, token::Token,
};
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

    /// Logs out from the cloud Bot API server.
    ///
    /// Note that after calling this method you must change the URI where `tbot`
    /// makes requests to your local Bot API server using [`server_uri`]. Once
    /// you log out, you cannot log back in the cloud server for 10 minutes.
    ///
    /// [`server_uri`]: #method.server_uri
    ///
    /// In case of an error, a tuple of `(`[`errors::MethodCall`]`, Self)` is
    /// returned in case you expect an error and can recover from it.
    ///
    /// [`errors::MethodCall`]: ./errors/enum.MethodCall.html
    pub async fn log_out(self) -> Result<Self, (errors::MethodCall, Self)> {
        match LogOut::new(&self.0).call().await {
            Ok(()) => Ok(self),
            Err(error) => Err((error, self)),
        }
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

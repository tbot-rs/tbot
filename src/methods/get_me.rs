use super::send_method;
use crate::{connectors::Connector, errors, internal::Client, types, Token};

/// Gets information about the bot.
///
/// Represents the [`getMe`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getme
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetMe<'a, C> {
    client: &'a Client<C>,
    token: Token,
}

impl<'a, C> GetMe<'a, C> {
    pub(crate) const fn new(client: &'a Client<C>, token: Token) -> Self {
        Self { client, token }
    }
}

impl<C: Connector> GetMe<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<types::User, errors::MethodCall> {
        send_method(self.client, &self.token, "getMe", None, Vec::new()).await
    }
}

use super::send_method;
use crate::{connectors::Client, errors, token, types};

/// Gets information about the bot.
///
/// Represents the [`getMe`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getme
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetMe<'a> {
    client: &'a Client,
    token: token::Ref<'a>,
}

impl<'a> GetMe<'a> {
    pub(crate) const fn new(client: &'a Client, token: token::Ref<'a>) -> Self {
        Self { client, token }
    }
}

impl GetMe<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<types::user::Me, errors::MethodCall> {
        send_method(self.client, self.token, "getMe", None, Vec::new()).await
    }
}

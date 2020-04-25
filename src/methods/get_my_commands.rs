use super::call_method;
use crate::{connectors::Client, errors, token, types::BotCommand};

/// Gets the list of the bot's commands.
///
/// Represents the [`getMyCommands`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getmycommands
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetMyCommands<'a> {
    client: &'a Client,
    token: token::Ref<'a>,
}

impl<'a> GetMyCommands<'a> {
    pub(crate) const fn new(client: &'a Client, token: token::Ref<'a>) -> Self {
        Self { client, token }
    }
}

impl GetMyCommands<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Vec<BotCommand>, errors::MethodCall> {
        call_method(self.client, self.token, "getMyCommands", None, Vec::new())
            .await
    }
}

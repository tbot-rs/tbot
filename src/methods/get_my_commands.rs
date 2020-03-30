use super::send_method;
use crate::{
    connectors::Connector, errors, internal::Client, token, types::BotCommand,
};

/// Gets the list of the bot's commands.
///
/// Represents the [`getMyCommands`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getmycommands
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetMyCommands<'a, C> {
    client: &'a Client<C>,
    token: token::Ref<'a>,
}

impl<'a, C> GetMyCommands<'a, C> {
    pub(crate) const fn new(
        client: &'a Client<C>,
        token: token::Ref<'a>,
    ) -> Self {
        Self { client, token }
    }
}

impl<C: Connector> GetMyCommands<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<Vec<BotCommand>, errors::MethodCall> {
        send_method(self.client, self.token, "getMyCommands", None, Vec::new())
            .await
    }
}

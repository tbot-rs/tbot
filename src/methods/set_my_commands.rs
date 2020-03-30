use super::send_method;
use crate::{
    connectors::Connector, errors, internal::Client, token,
    types::parameters::BotCommand,
};
use serde::Serialize;

/// Sets the list of the bot's commands.
///
/// Represents the [`setMyCommands`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setmycommands
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetMyCommands<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: token::Ref<'a>,
    commands: &'a [BotCommand<'a>],
}

impl<'a, C> SetMyCommands<'a, C> {
    pub(crate) const fn new(
        client: &'a Client<C>,
        token: token::Ref<'a>,
        commands: &'a [BotCommand<'a>],
    ) -> Self {
        Self {
            client,
            token,
            commands,
        }
    }
}

impl<C: Connector> SetMyCommands<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        send_method::<bool, _>(
            self.client,
            self.token,
            "setMyCommands",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}

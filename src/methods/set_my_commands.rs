use super::call_method;
use crate::{connectors::Client, errors, token, types::parameters::BotCommand};
use serde::Serialize;
use std::borrow::Cow;

/// Sets the list of the bot's commands.
///
/// Represents the [`setMyCommands`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setmycommands
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetMyCommands<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    commands: Cow<'a, [BotCommand<'a>]>,
}

impl<'a> SetMyCommands<'a> {
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        commands: impl Into<Cow<'a, [BotCommand<'a>]>>,
    ) -> Self {
        Self {
            client,
            token,
            commands: commands.into(),
        }
    }
}

impl SetMyCommands<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
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

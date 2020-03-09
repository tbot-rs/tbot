use super::{EventLoop, Polling, Webhook};
use crate::{connectors::Connector, contexts, errors};
use std::{future::Future, sync::Arc};

/// A stateful event loop.
pub struct Stateful<C, S> {
    inner: EventLoop<C>,
    state: Arc<S>,
}

impl<C, S> Stateful<C, S> {
    pub(crate) fn new(inner: EventLoop<C>, state: S) -> Self {
        Self {
            inner,
            state: Arc::new(state),
        }
    }

    /// Sets the bot's username.
    ///
    /// The username is used when checking if a command such as
    /// `/command@username` was directed to the bot.
    pub fn username(&mut self, username: String) {
        self.inner.username(username);
    }

    /// Starts polling configuration.
    pub fn polling(self) -> Polling<C> {
        self.inner.polling()
    }

    /// Starts webhook configuration.
    ///
    /// See our [wiki] to learn how to use webhook with `tbot`.
    ///
    /// [wiki]: https://gitlab.com/SnejUgal/tbot/wikis/How-to/How-to-use-webhooks
    pub fn webhook(self, url: &str, port: u16) -> Webhook<'_, C> {
        self.inner.webhook(url, port)
    }
}

impl<C, S: Send + Sync + 'static> Stateful<C, S> {
    /// Adds a new handler for a command.
    pub fn command<H, F>(&mut self, command: &'static str, handler: H)
    where
        H: (Fn(Arc<contexts::Command<contexts::Text<C>>>, Arc<S>) -> F)
            + Send
            + Sync
            + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        let state = Arc::clone(&self.state);
        self.inner.command(command, move |context| {
            handler(context, Arc::clone(&state))
        });
    }

    /// Adds a new handler for the `/start` command.
    pub fn start<H, F>(&mut self, handler: H)
    where
        H: (Fn(Arc<contexts::Command<contexts::Text<C>>>, Arc<S>) -> F)
            + Send
            + Sync
            + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.command("start", handler);
    }
}

impl<C: Connector, S> Stateful<C, S> {
    /// Fetches the bot's username.
    pub async fn fetch_username(&mut self) -> Result<(), errors::MethodCall> {
        self.inner.fetch_username().await
    }
}

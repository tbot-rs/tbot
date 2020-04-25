use crate::{
    contexts::fields::Message,
    errors,
    internal::Sealed,
    types::{
        chat,
        parameters::{ChatId, ImplicitChatId},
    },
    Bot,
};
use futures::future::BoxFuture;
use std::{convert::Infallible, time::Duration};
use tokio::time::delay_for;

const INTERVAL: Duration = Duration::from_secs(5);

async fn send_chat_action_in_loop(
    bot: Bot,
    chat_id: ChatId<'_>,
    action: chat::Action,
) -> Result<Infallible, errors::MethodCall> {
    loop {
        let delay = delay_for(INTERVAL);
        bot.send_chat_action(chat_id, action).call().await?;
        delay.await;
    }
}

/// An utility trait for [`Bot`] with a method to send a chat action in a loop.
///
/// [`Bot`]: ../struct.Bot.html
#[allow(clippy::module_name_repetitions)]
pub trait ChatActionLoopBotExt: Sealed {
    /// Sends a chat action in an infinite loop, returning only if failed
    /// to send the action.
    ///
    /// This utility is suppoed to be [`select!`]ed with another future.
    /// As soon as the other future completes, this future is dropped
    /// and the chat action is no longer sent.
    ///
    /// [`select!`]: https://docs.rs/tokio/0.2.*/tokio/macro.select.html
    fn send_chat_action_in_loop<'a>(
        &self,
        chat_id: impl ImplicitChatId<'a>,
        action: chat::Action,
    ) -> BoxFuture<'a, Result<Infallible, errors::MethodCall>>;
}

impl ChatActionLoopBotExt for Bot {
    fn send_chat_action_in_loop<'a>(
        &self,
        chat_id: impl ImplicitChatId<'a>,
        action: chat::Action,
    ) -> BoxFuture<'a, Result<Infallible, errors::MethodCall>> {
        let chat_id: ChatId = chat_id.into();
        Box::pin(send_chat_action_in_loop(self.clone(), chat_id, action))
    }
}

/// An utility trait for contexts with a method to send a chat action in a loop.
pub trait ChatActionLoop: Message {
    /// Sends a chat action in an infinite loop, returning only if failed
    /// to send the action.
    ///
    /// This utility is suppoed to be [`select!`]ed with another future.
    /// As soon as the other future completes, this future is dropped
    /// and the chat action is no longer sent.
    ///
    /// [`select!`]: https://docs.rs/tokio/0.2.*/tokio/macro.select.html
    fn send_chat_action_in_loop(
        &self,
        action: chat::Action,
    ) -> BoxFuture<Result<Infallible, errors::MethodCall>> {
        Box::pin(self.bot().send_chat_action_in_loop(self.chat().id, action))
    }
}

impl<T: Message> ChatActionLoop for T {}

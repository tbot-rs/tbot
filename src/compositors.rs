//! Useful compositors for update handlers.
//!
//! Let's start with a simple example. You're writing a bot, and its main
//! functionality is searching through something. In private chats, it's enough
//! to send a text message to start searching; but in groups, it's better to
//! search only when someone sent the `/search` command. Since you already
//! expected to have the same logic in different handlers, you wrote a generic
//! handler, `search`, and then registered it like this:
//!
//! ```no_run
//! use std::sync::Arc;
//! use tbot::{Bot, contexts::fields::Text, prelude::*};
//! async fn search(context: Arc<impl Text>) {
//!     // ..
//! }
//!
//! let mut bot = Bot::from_env("BOT_TOKEN").event_loop();
//!
//! bot.text(|context| async move {
//!     if context.chat.kind.is_private() {
//!         search(context).await;
//!     }
//! });
//!
//! bot.command("search", search);
//! ```
//!
//! The `text` handler seems a bit too long just to filter out messages from
//! channels and groups. That's where we can make use of our first compositor,
//! [`filter`]! It takes a predicate and a handler and returns its own handler.
//! If the predicate returns `true`, [`filter`]'s handler executes yours.
//! Here's an example:
//!
//! ```no_run
//! # use std::sync::Arc;
//! # use tbot::{Bot, contexts::fields, prelude::*};
//! # async fn search(context: Arc<impl fields::Text>) {}
//! # let mut bot = Bot::from_env("BOT_TOKEN").event_loop();
//! use tbot::{contexts::Text, compositors::filter};
//!
//! bot.text(filter(
//!     |context: Arc<Text>| async move { context.chat.kind.is_private() },
//!     search,
//! ));
//! ```
//!
//! That looks better! But `tbot` already provides the predicate we just wrote
//! ourselves, so let's use that:
//!
//! ```no_run
//! # use std::sync::Arc;
//! # use tbot::{Bot, contexts::fields::Text, prelude::*};
//! # async fn search(context: Arc<impl Text>) {}
//! # let mut bot = Bot::from_env("BOT_TOKEN").event_loop();
//! use tbot::{compositors::filter, predicates::chat::is_private};
//!
//! bot.text(filter(is_private, search));
//! ```
//!
//! Great! Now you see how compositors make registering handlers easier.
//! [`filter`] isn't the only compositor, you'll find all of them in this
//! module.
//!
//! Remember one thing though. Compositors add layers between the handler
//! and handler registers. If you're going to use a closure for your handler,
//! type inference will fail and you'll have to explicitly define `context`'s
//! type, as seen in the example with an inlined predicate above. That only
//! affects closures: using plain functions is fine even with generics, as seen
//! in the last example.

use crate::contexts::fields::Context;
use futures::future::BoxFuture;
use std::{future::Future, sync::Arc};

pub mod state;

/// Filters updates: executes `handler` only if `predicate` returns `true`.
///
/// # Example
///
/// ```no_run
/// use tbot::{Bot, contexts::Text, compositors::filter, predicates::chat::is_group};
/// use std::sync::Arc;
///
/// let mut bot = Bot::from_env("BOT_TOKEN").event_loop();
/// bot.text(filter(is_group, |context: Arc<Text>| async move {
///     // Do something assuming we're in a group
/// }));
/// ```
pub fn filter<C, P, PF, H, HF>(
    predicate: P,
    handler: H,
) -> impl Fn(Arc<C>) -> BoxFuture<'static, ()>
where
    C: Context,
    P: Fn(Arc<C>) -> PF + Send + Sync + 'static,
    PF: Future<Output = bool> + Send + 'static,
    H: Fn(Arc<C>) -> HF + Send + Sync + 'static,
    HF: Future<Output = ()> + Send + 'static,
{
    let shared = Arc::new((predicate, handler));

    move |context| {
        let shared = Arc::clone(&shared);

        Box::pin(async move {
            let (predicate, handler) = &*shared;

            if predicate(Arc::clone(&context)).await {
                handler(context).await;
            }
        })
    }
}

/// Maps updates: calls `mapper`, and then passes its return value to `handler`.
///
/// # Example
///
/// ```no_run
/// use tbot::{Bot, contexts::Text, compositors::map};
/// use std::sync::Arc;
///
/// let mut bot = Bot::from_env("BOT_TOKEN").event_loop();
/// bot.text(map(
///     |context: Arc<Text>| async move { context.text.clone() },
///     |text| async move {
///         dbg!(text);
///     },
/// ));
/// ```
pub fn map<C, T, M, MF, H, HF>(
    mapper: M,
    handler: H,
) -> impl Fn(Arc<C>) -> BoxFuture<'static, ()>
where
    C: Context,
    T: Send + 'static,
    M: Fn(Arc<C>) -> MF + Send + Sync + 'static,
    MF: Future<Output = T> + Send + 'static,
    H: Fn(T) -> HF + Send + Sync + 'static,
    HF: Future<Output = ()> + Send + 'static,
{
    let shared = Arc::new((mapper, handler));

    move |context| {
        let shared = Arc::clone(&shared);

        Box::pin(async move {
            let (mapper, handler) = &*shared;
            handler(mapper(context).await).await
        })
    }
}

/// Filters and maps updates: calls `predicate`, and if it returned `Some`,
/// calls `handler` with that value.
///
/// # Example
///
/// ```no_run
/// use tbot::{Bot, contexts::Text, compositors::filter_map};
/// use std::sync::Arc;
///
/// let mut bot = Bot::from_env("BOT_TOKEN").event_loop();
/// bot.text(filter_map(
///     |context: Arc<Text>| async move { context.reply_to.clone() },
///     |reply_to| async move {
///         println!("Recevied a message in reply to {}", reply_to.id);
///     },
/// ));
/// ```
pub fn filter_map<C, T, M, MF, H, HF>(
    mapper: M,
    handler: H,
) -> impl Fn(Arc<C>) -> BoxFuture<'static, ()>
where
    C: Context,
    T: Send + 'static,
    M: Fn(Arc<C>) -> MF + Send + Sync + 'static,
    MF: Future<Output = Option<T>> + Send + 'static,
    H: Fn(T) -> HF + Send + Sync + 'static,
    HF: Future<Output = ()> + Send + 'static,
{
    let shared = Arc::new((mapper, handler));

    move |context| {
        let shared = Arc::clone(&shared);

        Box::pin(async move {
            let (mapper, handler) = &*shared;
            if let Some(mapped) = mapper(context).await {
                handler(mapped).await
            }
        })
    }
}

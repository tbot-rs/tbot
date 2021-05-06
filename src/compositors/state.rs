//! All the compositors, but adapted for [`StatefulEventLoop`].
//!
//! [`StatefulEventLoop`]: crate::state::StatefulEventLoop

use crate::contexts::fields::Context;
use futures::future::BoxFuture;
use std::{future::Future, sync::Arc};

/// Filters updates: executes `handler` only if `predicate` returns `true`.
///
/// # Example
///
/// ```no_run
/// use tbot::{
///     Bot, contexts::Text, compositors::state::filter,
///     predicates::{without_state, chat::is_group},
/// };
/// use std::sync::{Arc, atomic::AtomicU32};
///
/// let mut bot = Bot::from_env("BOT_TOKEN").stateful_event_loop(AtomicU32::new(0));
/// bot.text(filter(
///     without_state(is_group),
///     |context: Arc<Text>, state: Arc<AtomicU32>| async move {
///         // Do something assuming we're in a group
///     },
/// ));
/// ```
pub fn filter<C, S, P, PF, H, HF>(
    predicate: P,
    handler: H,
) -> impl Fn(Arc<C>, Arc<S>) -> BoxFuture<'static, ()>
where
    C: Context,
    S: Send + Sync + 'static,
    P: Fn(Arc<C>, Arc<S>) -> PF + Send + Sync + 'static,
    PF: Future<Output = bool> + Send + 'static,
    H: Fn(Arc<C>, Arc<S>) -> HF + Send + Sync + 'static,
    HF: Future<Output = ()> + Send + 'static,
{
    let shared = Arc::new((predicate, handler));

    move |context, state| {
        let shared = Arc::clone(&shared);

        Box::pin(async move {
            let (predicate, handler) = &*shared;

            if predicate(Arc::clone(&context), Arc::clone(&state)).await {
                handler(context, state).await;
            }
        })
    }
}

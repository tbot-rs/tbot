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

/// Maps updates: calls `mapper`, and then passes its return value to `handler`.
///
/// Note that `handler` does **not** receive the state. If you need it,
/// return it from `mapper` in a tuple.
///
/// # Example
///
/// ```no_run
/// use tbot::{Bot, contexts::Text, compositors::state::map};
/// use std::sync::{Arc, atomic::AtomicU32};
///
/// let mut bot = Bot::from_env("BOT_TOKEN").stateful_event_loop(AtomicU32::new(0));
/// bot.text(map(
///     |context: Arc<Text>, state: Arc<AtomicU32>| async move {
///         (context.text.clone(), state)
///     },
///     |(text, _state)| async move {
///         dbg!(text);
///     },
/// ));
/// ```
pub fn map<C, S, T, M, MF, H, HF>(
    mapper: M,
    handler: H,
) -> impl Fn(Arc<C>, Arc<S>) -> BoxFuture<'static, ()>
where
    C: Context,
    S: Send + Sync + 'static,
    T: Send + 'static,
    M: Fn(Arc<C>, Arc<S>) -> MF + Send + Sync + 'static,
    MF: Future<Output = T> + Send + 'static,
    H: Fn(T) -> HF + Send + Sync + 'static,
    HF: Future<Output = ()> + Send + 'static,
{
    let shared = Arc::new((mapper, handler));

    move |context, state| {
        let shared = Arc::clone(&shared);

        Box::pin(async move {
            let (mapper, handler) = &*shared;
            handler(mapper(context, state).await).await
        })
    }
}

/// Filters and maps updates: calls `predicate`, and if it returned `Some`,
/// calls `handler` with that value.
///
/// Note that `handler` does **not** receive the state. If you need it,
/// return it from `mapper` in a tuple.
///
/// # Example
///
/// ```no_run
/// use tbot::{Bot, contexts::Text, compositors::state::filter_map};
/// use std::sync::{Arc, atomic::{AtomicU32, Ordering}};
///
/// let mut bot = Bot::from_env("BOT_TOKEN").stateful_event_loop(AtomicU32::new(0));
/// bot.text(filter_map(
///     |context: Arc<Text>, state: Arc<AtomicU32>| async move {
///         if state.fetch_add(1, Ordering::Relaxed) < 10 {
///             Some(context.text.clone())
///         } else {
///             None
///         }
///     },
///     |text| async move {
///         dbg!(text);
///     },
/// ));
/// ```
pub fn filter_map<C, S, T, M, MF, H, HF>(
    mapper: M,
    handler: H,
) -> impl Fn(Arc<C>, Arc<S>) -> BoxFuture<'static, ()>
where
    C: Context,
    S: Send + Sync + 'static,
    T: Send + 'static,
    M: Fn(Arc<C>, Arc<S>) -> MF + Send + Sync + 'static,
    MF: Future<Output = Option<T>> + Send + 'static,
    H: Fn(T) -> HF + Send + Sync + 'static,
    HF: Future<Output = ()> + Send + 'static,
{
    let shared = Arc::new((mapper, handler));

    move |context, state| {
        let shared = Arc::clone(&shared);

        Box::pin(async move {
            let (mapper, handler) = &*shared;
            if let Some(mapped) = mapper(context, state).await {
                handler(mapped).await
            }
        })
    }
}

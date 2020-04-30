//! Useful predicates and utilities for them.

pub mod chat;
pub mod media;
pub mod message;
mod traits;

use futures::{future::BoxFuture, Future};
use std::sync::Arc;
pub use traits::{
    PredicateBooleanOperations, StatefulPredicateBooleanOperations,
};

/// Allows running stateless predicates in the stateful event loop.
pub fn without_state<'a, C, P, S, F>(
    predicate: P,
) -> impl Fn(Arc<C>, Arc<S>) -> BoxFuture<'a, bool> + Send + Sync + 'a
where
    P: PredicateBooleanOperations<C, F>,
    F: Future<Output = bool> + Send,
    C: Send + Sync + 'static,
    S: Send + Sync + 'static,
{
    let predicate = Arc::new(predicate);

    move |ctx, _state| {
        let predicate = Arc::clone(&predicate);
        Box::pin(async move { predicate(ctx).await })
    }
}

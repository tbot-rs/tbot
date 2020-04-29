use futures::{future::BoxFuture, Future};
use std::sync::Arc;

/// Boolean operations on async functions' results.
/// Simplifies combinating predicates.
pub trait PredicateBooleanOperations<C, F>: Fn(Arc<C>) -> F
where
    F: Future<Output = bool> + Send,
    C: Send + Sync + 'static,
    Self: Sized + Send + Sync + 'static,
{
    /// `self(..).await && other(..).await`
    fn and<'a, P: 'static, PF>(
        self,
        other: P,
    ) -> Box<dyn Fn(Arc<C>) -> BoxFuture<'a, bool> + Send + Sync + 'a>
    where
        P: PredicateBooleanOperations<C, PF> + 'a,
        PF: Future<Output = bool> + Send,
    {
        let other = Arc::new(other);
        let this = Arc::new(self);

        Box::new(move |ctx| {
            let other = Arc::clone(&other);
            let this = Arc::clone(&this);
            Box::pin(async move { this(ctx.clone()).await && other(ctx).await })
        })
    }

    /// `self(..).await || other(..).await`
    fn or<'a, P: 'static, PF>(
        self,
        other: P,
    ) -> Box<dyn Fn(Arc<C>) -> BoxFuture<'a, bool> + Send + Sync + 'a>
    where
        P: PredicateBooleanOperations<C, PF> + 'a,
        PF: Future<Output = bool> + Send,
    {
        let other = Arc::new(other);
        let this = Arc::new(self);

        Box::new(move |ctx| {
            let other = Arc::clone(&other);
            let this = Arc::clone(&this);
            Box::pin(async move { this(ctx.clone()).await || other(ctx).await })
        })
    }

    /// `!self(..).await`
    fn not<'a>(
        self,
    ) -> Box<dyn Fn(Arc<C>) -> BoxFuture<'a, bool> + Send + Sync + 'a> {
        let this = Arc::new(self);

        Box::new(move |ctx| {
            let this = Arc::clone(&this);
            Box::pin(async move { !this(ctx.clone()).await })
        })
    }
}

impl<C, F, T> PredicateBooleanOperations<C, F> for T
where
    T: (Fn(Arc<C>) -> F) + Sized + Send + Sync + 'static,
    F: Future<Output = bool> + Send,
    C: Send + Sync + 'static,
{
}

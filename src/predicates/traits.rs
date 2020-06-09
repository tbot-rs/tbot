use futures::{future::BoxFuture, Future};
use std::sync::Arc;

type BoxedPredicate<C> =
    Box<dyn Fn(Arc<C>) -> BoxFuture<'static, bool> + Send + Sync + 'static>;
type BoxedStatefulPredicate<C, S> = Box<
    dyn Fn(Arc<C>, Arc<S>) -> BoxFuture<'static, bool> + Send + Sync + 'static,
>;

/// Boolean operations on predicates.
pub trait PredicateBooleanOperations<C, F>: Fn(Arc<C>) -> F
where
    F: Future<Output = bool> + Send,
    C: Send + Sync + 'static,
    Self: Sized + Send + Sync + 'static,
{
    /// `self(..).await && other(..).await`
    fn and<P, PF>(self, other: P) -> BoxedPredicate<C>
    where
        P: PredicateBooleanOperations<C, PF> + 'static,
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
    fn or<P, PF>(self, other: P) -> BoxedPredicate<C>
    where
        P: PredicateBooleanOperations<C, PF> + 'static,
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

    /// `self(..).await != other(..).await`
    #[allow(clippy::eval_order_dependence)] // everything is clear here
    fn xor<P, PF>(self, other: P) -> BoxedPredicate<C>
    where
        P: PredicateBooleanOperations<C, PF> + 'static,
        PF: Future<Output = bool> + Send,
    {
        let other = Arc::new(other);
        let this = Arc::new(self);

        Box::new(move |ctx| {
            let other = Arc::clone(&other);
            let this = Arc::clone(&this);
            Box::pin(async move { this(ctx.clone()).await != other(ctx).await })
        })
    }

    /// `!self(..).await`
    fn not(self) -> BoxedPredicate<C> {
        let this = Arc::new(self);

        Box::new(move |ctx| {
            let this = Arc::clone(&this);
            Box::pin(async move { !this(ctx).await })
        })
    }
}

/// Boolean operations on stateful predicates.
pub trait StatefulPredicateBooleanOperations<C, S, F>:
    Fn(Arc<C>, Arc<S>) -> F
where
    F: Future<Output = bool> + Send,
    C: Send + Sync + 'static,
    S: Send + Sync + 'static,
    Self: Sized + Send + Sync + 'static,
{
    /// `self(..).await && other(..).await`
    fn and<P, PF>(self, other: P) -> BoxedStatefulPredicate<C, S>
    where
        P: StatefulPredicateBooleanOperations<C, S, PF> + 'static,
        PF: Future<Output = bool> + Send,
    {
        let other = Arc::new(other);
        let this = Arc::new(self);

        Box::new(move |ctx, state| {
            let other = Arc::clone(&other);
            let this = Arc::clone(&this);
            Box::pin(async move {
                this(ctx.clone(), state.clone()).await
                    && other(ctx, state).await
            })
        })
    }

    /// `self(..).await || other(..).await`
    fn or<P, PF>(self, other: P) -> BoxedStatefulPredicate<C, S>
    where
        P: StatefulPredicateBooleanOperations<C, S, PF> + 'static,
        PF: Future<Output = bool> + Send,
    {
        let other = Arc::new(other);
        let this = Arc::new(self);

        Box::new(move |ctx, state| {
            let other = Arc::clone(&other);
            let this = Arc::clone(&this);
            Box::pin(async move {
                this(ctx.clone(), state.clone()).await
                    || other(ctx, state).await
            })
        })
    }

    /// `self(..).await != other(..).await`
    #[allow(clippy::eval_order_dependence)] // everything is clear here
    fn xor<P, PF>(self, other: P) -> BoxedStatefulPredicate<C, S>
    where
        P: StatefulPredicateBooleanOperations<C, S, PF> + 'static,
        PF: Future<Output = bool> + Send,
    {
        let other = Arc::new(other);
        let this = Arc::new(self);

        Box::new(move |ctx, state| {
            let other = Arc::clone(&other);
            let this = Arc::clone(&this);
            Box::pin(async move {
                this(ctx.clone(), state.clone()).await
                    != other(ctx, state).await
            })
        })
    }

    /// `!self(..).await`
    fn not(self) -> BoxedStatefulPredicate<C, S> {
        let this = Arc::new(self);

        Box::new(move |ctx, state| {
            let this = Arc::clone(&this);
            Box::pin(async move { !this(ctx, state).await })
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

impl<C, S, F, T> StatefulPredicateBooleanOperations<C, S, F> for T
where
    T: (Fn(Arc<C>, Arc<S>) -> F) + Sized + Send + Sync + 'static,
    F: Future<Output = bool> + Send,
    C: Send + Sync + 'static,
    S: Send + Sync + 'static,
{
}

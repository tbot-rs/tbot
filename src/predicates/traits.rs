use futures::{future::BoxFuture, Future};
use std::sync::Arc;

type BoxedPredicate<'a, C> =
    Box<dyn Fn(Arc<C>) -> BoxFuture<'a, bool> + Send + Sync + 'a>;
type BoxedStatefulPredicate<'a, C, S> =
    Box<dyn Fn(Arc<C>, Arc<S>) -> BoxFuture<'a, bool> + Send + Sync + 'a>;

/// Boolean operations on predicates.
pub trait PredicateBooleanOperations<C, F>: Fn(Arc<C>) -> F
where
    F: Future<Output = bool> + Send,
    C: Send + Sync + 'static,
    Self: Sized + Send + Sync + 'static,
{
    /// `self(..).await && other(..).await`
    fn and<'a, P: 'static, PF>(self, other: P) -> BoxedPredicate<'a, C>
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
    fn or<'a, P: 'static, PF>(self, other: P) -> BoxedPredicate<'a, C>
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

    /// `self(..).await != other(..).await`
    fn xor<'a, P: 'static, PF>(self, other: P) -> BoxedPredicate<'a, C>
    where
        P: PredicateBooleanOperations<C, PF> + 'a,
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
    fn not<'a>(self) -> BoxedPredicate<'a, C> {
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
    fn and<'a, P: 'static, PF>(
        self,
        other: P,
    ) -> BoxedStatefulPredicate<'a, C, S>
    where
        P: StatefulPredicateBooleanOperations<C, S, PF> + 'a,
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
    fn or<'a, P: 'static, PF>(
        self,
        other: P,
    ) -> BoxedStatefulPredicate<'a, C, S>
    where
        P: StatefulPredicateBooleanOperations<C, S, PF> + 'a,
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
    fn xor<'a, P: 'static, PF>(
        self,
        other: P,
    ) -> BoxedStatefulPredicate<'a, C, S>
    where
        P: StatefulPredicateBooleanOperations<C, S, PF> + 'a,
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
    fn not<'a>(self) -> BoxedStatefulPredicate<'a, C, S> {
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

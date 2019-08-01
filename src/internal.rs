use futures::Future;

pub trait Sealed {} // used for sealing traits

pub type Client<C> = hyper::Client<C, hyper::Body>;
pub type BoxFuture<I, E> = Box<dyn Future<Item = I, Error = E> + Send>;

pub trait AsInnerRef<'a, T> {
    fn as_inner_ref(&'a self) -> Option<&'a T>;
}

pub trait Sealed {} // used for sealing traits

pub type Client<C> = hyper::Client<C, hyper::Body>;

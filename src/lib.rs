#![deny(future_incompatible)]
#![deny(nonstandard_style)]

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
extern crate serde_json;
extern crate tokio;
#[macro_use]
extern crate serde_derive;

#[cfg(feature = "proxy")]
extern crate hyper_proxy;

mod bot;
mod updates;

pub mod methods;
pub mod types;

pub use self::bot::*;
pub use self::tokio::{run, spawn};
pub use self::updates::*;

pub mod prelude {
    //! Re-exports some traits the compile may demand when working with `tbot`.
    pub use futures::Future;
}

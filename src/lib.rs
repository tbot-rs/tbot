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
pub use self::updates::*;

/// Like re-exported `tokio::run`, but doesn't require `future::Item` to be
/// `()`.
///
/// Most use-caces of `tbot` do not need to use the future's `Item` value,
/// leading to many `.map(|_| ())` in the code. This code will implicitly map
/// `Item` to `()`. Note that it does **not** map `Error` to `()`, because
/// error handling must be done on your own.
pub fn run<F>(future: F)
where
    F: futures::Future<Error = ()> + Send + 'static,
{
    tokio::run(future.map(|_| ()));
}

/// Like re-exported `tokio::spawn`, but doesn't require `future::Item` to be
/// `()`.
///
/// Most use-caces of `tbot` do not need to use the future's `Item` value,
/// leading to many `.map(|_| ())` in the code. This code will implicitly map
/// `Item` to `()`. Note that it does **not** map `Error` to `()`, because
/// error handling must be done on your own.
pub fn spawn<F>(future: F) -> tokio::executor::Spawn
where
    F: futures::Future<Error = ()> + Send + 'static,
{
    tokio::spawn(future.map(|_| ()))
}

pub mod prelude {
    //! Re-exports some traits the compile may demand when working with `tbot`.
    pub use futures::Future;
}

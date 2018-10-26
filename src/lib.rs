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
mod ser_mutlipart;
mod updates;

pub mod methods;
pub mod types;

pub use bot::*;
pub use tokio::{run, spawn};
pub use updates::*;

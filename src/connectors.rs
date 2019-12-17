//! A few common connectors for making requests.

use crate::internal;
use hyper::{
    client::{connect::Connect, HttpConnector},
    Body, Client,
};
use hyper_tls::HttpsConnector;

/// The default HTTPS connector.
pub type Https = HttpsConnector<HttpConnector>;

/// Constructs a HTTPS connector.
pub fn https() -> Https {
    HttpsConnector::new()
}

pub(crate) fn create_client<C: Connector>(connector: C) -> internal::Client<C> {
    Client::builder()
        .keep_alive(false)
        .build::<C, Body>(connector)
}

pub(crate) fn default() -> internal::Client<Https> {
    create_client(https())
}

/// An alias for a connector usable by `hyper`.
pub trait Connector: Connect + Clone + Send + Sync + 'static {}
impl<T: Connect + Clone + Send + Sync + 'static> Connector for T {}

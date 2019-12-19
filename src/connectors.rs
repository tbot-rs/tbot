//! A few common connectors for making requests.

use crate::internal;
use hyper::{
    client::{connect::Connect, HttpConnector},
    Body, Client,
};
use hyper_tls::HttpsConnector;

#[cfg(feature = "proxy")]
pub use hyper_proxy as proxy;
#[cfg(feature = "proxy")]
use proxy::ProxyConnector;

/// The default HTTPS connector.
pub type Https = HttpsConnector<HttpConnector>;

#[cfg(feature = "proxy")]
/// The default proxy connector.
pub type Proxy = ProxyConnector<Https>;

/// Constructs a HTTPS connector.
pub fn https() -> Https {
    HttpsConnector::new()
}

#[cfg(feature = "proxy")]
/// Constructs a proxy connector.
pub fn proxy(proxy: proxy::Proxy) -> Proxy {
    ProxyConnector::from_proxy(https(), proxy).unwrap_or_else(|error| {
        panic!("[tbot] Failed to construct a proxy connector: {:#?}", error)
    })
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

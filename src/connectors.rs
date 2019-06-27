//! A few common connectors for making requests.

#[cfg(feature = "proxy")]
pub use hyper_proxy as proxy;

use crate::internal;
use hyper::{
    client::{connect::Connect, HttpConnector},
    Body, Client,
};
use hyper_tls::HttpsConnector;

#[cfg(feature = "proxy")]
use proxy::ProxyConnector;

/// The default HTTPS connector.
pub type Https = HttpsConnector<HttpConnector>;

#[cfg(feature = "proxy")]
/// The default proxy connector.
pub type Proxy = ProxyConnector<Https>;

/// Constructs a HTTPS connector.
pub fn https() -> Https {
    HttpsConnector::new(num_cpus::get()).unwrap()
}

#[cfg(feature = "proxy")]
/// Constructs a proxy connector.
pub fn proxy(proxy: proxy::Proxy) -> Proxy {
    ProxyConnector::from_proxy(https(), proxy).unwrap()
}

pub(crate) fn create_client<C>(connector: C) -> internal::Client<C>
where
    C: Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    Client::builder().keep_alive(false).build::<C, Body>(connector)
}

pub(crate) fn default() -> internal::Client<Https> {
    create_client(https())
}

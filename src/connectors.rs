//! A few common connectors for making requests.

use hyper::{
    client::{HttpConnector, ResponseFuture},
    Body, Client as HyperClient, Request, Uri,
};

#[cfg(feature = "rustls")]
use hyper_rustls::HttpsConnector;
#[cfg(feature = "tls")]
use hyper_tls::HttpsConnector;

pub use hyper_proxy as proxy;
use proxy::ProxyConnector;

/// The default HTTPS connector.
pub type Https = HttpsConnector<HttpConnector>;

/// The default proxy connector.
pub type Proxy = ProxyConnector<Https>;

#[derive(Debug)]
pub(crate) enum Client {
    Https(HyperClient<Https>),
    Proxy(HyperClient<Proxy>),
}

impl Client {
    pub(crate) fn proxy(proxy: proxy::Proxy) -> Self {
        let connector =
            ProxyConnector::from_proxy(HttpsConnector::new(), proxy)
                .unwrap_or_else(|error| {
                    panic!(
                        "[tbot] Failed to construct a proxy connector: {:#?}",
                        error
                    )
                });

        Self::Proxy(
            HyperClient::builder()
                .pool_max_idle_per_host(0)
                .build::<Proxy, Body>(connector),
        )
    }

    #[must_use]
    pub(crate) fn https() -> Self {
        let connector = HttpsConnector::new();

        Self::Https(
            HyperClient::builder()
                .pool_max_idle_per_host(0)
                .build::<Https, Body>(connector),
        )
    }

    pub(crate) fn get(&self, uri: Uri) -> ResponseFuture {
        match self {
            Self::Https(https) => https.get(uri),
            Self::Proxy(proxy) => proxy.get(uri),
        }
    }

    pub(crate) fn request(&self, req: Request<Body>) -> ResponseFuture {
        match self {
            Self::Https(https) => https.request(req),
            Self::Proxy(proxy) => proxy.request(req),
        }
    }
}

pub(crate) fn default() -> Client {
    Client::https()
}

//! A few common connectors for making requests.

use hyper::{
    self,
    client::{HttpConnector, ResponseFuture},
    Body, Request, Uri,
};

#[cfg(feature = "rustls")]
use hyper_rustls::HttpsConnector;
#[cfg(feature = "tls")]
use hyper_tls::HttpsConnector;

use hyper_proxy as proxy;
use proxy::ProxyConnector;

use hyper_socks2 as socks_proxy;
use socks_proxy::{Auth, SocksConnector};

/// The default HTTPS connector.
pub type Https = HttpsConnector<HttpConnector>;

/// The default HTTPS proxy connector.
pub type HttpsProxy = ProxyConnector<Https>;
/// The default SOCKS proxy connector.
pub type SocksProxy = HttpsConnector<SocksConnector<Https>>;

#[derive(Debug)]
pub(crate) enum Client {
    Https(hyper::Client<Https>),
    HttpsProxy(hyper::Client<HttpsProxy>),
    SocksProxy(hyper::Client<SocksProxy>),
}

impl Client {
    pub(crate) fn https_proxy(proxy: proxy::Proxy) -> Self {
        let connector =
            ProxyConnector::from_proxy(HttpsConnector::new(), proxy)
                .unwrap_or_else(|error| {
                    panic!(
                        "[tbot] Failed to construct a proxy connector: {:#?}",
                        error
                    )
                });

        Self::HttpsProxy(
            hyper::Client::builder()
                .pool_max_idle_per_host(0)
                .build::<HttpsProxy, Body>(connector),
        )
    }

    pub(crate) fn socks_proxy(proxy_addr: Uri, auth: Option<Auth>) -> Self {
        let connector = SocksConnector {
            proxy_addr,
            auth,
            connector: HttpsConnector::new(),
        };

        let connector = connector.with_tls().unwrap_or_else(|error| {
            panic!(
                "[tbot] Failed to construct a SOCKS proxy connector: {:#?}",
                error
            )
        });

        Self::SocksProxy(
            hyper::Client::builder()
                .pool_max_idle_per_host(0)
                .build::<SocksProxy, Body>(connector),
        )
    }

    #[must_use]
    pub(crate) fn https() -> Self {
        let connector = HttpsConnector::new();

        Self::Https(
            hyper::Client::builder()
                .pool_max_idle_per_host(0)
                .build::<Https, Body>(connector),
        )
    }

    pub(crate) fn get(&self, uri: Uri) -> ResponseFuture {
        match self {
            Self::Https(https) => https.get(uri),
            Self::HttpsProxy(proxy) => proxy.get(uri),
            Self::SocksProxy(socks_proxy) => socks_proxy.get(uri),
        }
    }

    pub(crate) fn request(&self, req: Request<Body>) -> ResponseFuture {
        match self {
            Self::Https(https) => https.request(req),
            Self::HttpsProxy(proxy) => proxy.request(req),
            Self::SocksProxy(socks_proxy) => socks_proxy.request(req),
        }
    }
}

//! A module for working with proxy.

use crate::connectors::Client;
use hyper::Uri;
pub use hyper_proxy as https;
pub use hyper_socks2 as socks;
use socks::Auth;

/// An enum of possible proxies.
#[derive(Debug, Clone)]
pub enum Proxy {
    /// A HTTPS proxy.
    Https(https::Proxy),
    /// A SOCKS proxy.
    Socks {
        /// The proxy's address.
        uri: Uri,
        /// The proxy's authentication data.
        auth: Option<Auth>,
    },
}

impl Proxy {
    /// Configures an HTTPS proxy.
    pub const fn https(proxy: https::Proxy) -> Self {
        Self::Https(proxy)
    }

    /// Configures a SOCKS proxy.
    pub const fn socks(uri: Uri, auth: Option<Auth>) -> Self {
        Self::Socks { uri, auth }
    }
}

impl From<https::Proxy> for Proxy {
    fn from(proxy: https::Proxy) -> Self {
        Self::https(proxy)
    }
}

impl From<Proxy> for Client {
    fn from(proxy: Proxy) -> Self {
        match proxy {
            Proxy::Https(https) => Self::https_proxy(https),
            Proxy::Socks { uri, auth } => Self::socks_proxy(uri, auth),
        }
    }
}

use super::*;

/// A struct that implementes [`Methods`] and inherits the token from a [`Bot`].
///
/// In some cases, you may want to clone a [`Bot`] to call a method. However,
/// you can't do it because a [`Bot`] also contains data for updata handling.
/// Instead, you can call [`Bot::mock`] to construct this struct that implements
/// the [`Methods`] trait but has no handling logic. For example, `tbot` uses
/// `MockBot` in [contexts] to provide the ability to call any API method
/// in a handler.
///
/// [`Methods`]: ./methods/trait.Method.html
/// [`Bot`]: ./struct.Bot.html
/// [`Bot::mock`]: ./struct.Bot.html#method.mock
/// [contexts]: ./contexts/
#[derive(Clone)]
pub struct MockBot {
    token: Arc<String>,
    #[cfg(feature = "proxy")]
    proxy: Option<proxy::Proxy>,
}

impl MockBot {
    #[cfg(feature = "proxy")]
    pub(crate) const fn new(
        token: Arc<String>,
        proxy: Option<proxy::Proxy>,
    ) -> Self {
        Self {
            token,
            proxy,
        }
    }

    #[cfg(not(feature = "proxy"))]
    pub(crate) const fn new(token: Arc<String>) -> Self {
        Self {
            token,
        }
    }
}

impl Methods<'_> for MockBot {
    fn token(&self) -> &str {
        &self.token
    }

    #[cfg(feature = "proxy")]
    fn get_proxy(&self) -> Option<proxy::Proxy> {
        self.proxy.clone()
    }
}

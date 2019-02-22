use super::*;

/// Struct that isn't actually a [`Bot`], but has all the methods for calling
/// the API inferring token.
///
/// It is used in contexts to imitate that `context.bot` is a [`Bot`], but it
/// actually isn't. Using the real `Bot` is complicated and could actually lead
/// to confusing error messages by the compiler on the user's side.
///
/// You can also construct it with [`Bot::mock`] whenever you feel like cloning
/// a [`Bot`] to call API methods outside `tbot`.
///
/// [`Bot`]: ./struct.Bot.html
/// [`Bot::mock`]: ./struct.Bot.html#method.mock
#[derive(Clone)]
pub struct MockBot {
    token: Arc<String>,
    #[cfg(feature = "proxy")]
    proxy: Option<proxy::Proxy>,
}

impl MockBot {
    #[cfg(feature = "proxy")]
    pub(crate) fn new(token: Arc<String>, proxy: Option<proxy::Proxy>) -> Self {
        Self {
            token,
            proxy,
        }
    }

    #[cfg(not(feature = "proxy"))]
    pub(crate) fn new(token: Arc<String>) -> Self {
        Self {
            token,
        }
    }
}

impl Methods for MockBot {
    fn token(&self) -> &str {
        &self.token
    }

    #[cfg(feature = "proxy")]
    fn get_proxy(&self) -> Option<proxy::Proxy> {
        self.proxy.clone()
    }
}

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
}

impl MockBot {
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
}

use serde::Serialize;
use std::borrow::Cow;

/// Represents a [`LoginUrl`].
///
/// [`LoginUrl`]: https://core.telegram.org/bots/api#loginurl
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[must_use]
pub struct LoginUrl<'a> {
    url: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forward_text: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bot_username: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_write_access: Option<bool>,
}

impl<'a> LoginUrl<'a> {
    /// Constructs a new `LoginUrl`.
    pub fn new(url: impl Into<Cow<'a, str>>) -> Self {
        Self {
            url: url.into(),
            forward_text: None,
            bot_username: None,
            request_write_access: None,
        }
    }

    /// Create an owned `LoginUrl` from a reference without cloning.
    pub fn as_borrowed(&'a self) -> Self {
        Self {
            url: Cow::Borrowed(&self.url),
            forward_text: self
                .forward_text
                .as_deref()
                .map(|x| Cow::Borrowed(x)),
            bot_username: self
                .bot_username
                .as_deref()
                .map(|x| Cow::Borrowed(x)),
            request_write_access: self.request_write_access,
        }
    }

    /// Configures `forward_text`.
    pub fn forward_text(mut self, text: impl Into<Cow<'a, str>>) -> Self {
        self.forward_text = Some(text.into());
        self
    }

    /// Configures `bot_username`.
    pub fn bot_username(mut self, username: impl Into<Cow<'a, str>>) -> Self {
        self.bot_username = Some(username.into());
        self
    }

    /// Configures `request_write_access`.
    pub const fn request_write_access(mut self, should_request: bool) -> Self {
        self.request_write_access = Some(should_request);
        self
    }
}

use serde::Serialize;

/// Represents a [`LoginUrl`].
///
/// [`LoginUrl`]: https://core.telegram.org/bots/api#loginurl
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
pub struct LoginUrl<'a> {
    url: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    forward_text: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bot_username: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_write_access: Option<bool>,
}

impl<'a> LoginUrl<'a> {
    /// Constructs a new `LoginUrl`.
    pub const fn new(url: &'a str) -> Self {
        Self {
            url,
            forward_text: None,
            bot_username: None,
            request_write_access: None,
        }
    }

    /// Configures `forward_text`.
    pub fn forward_text(mut self, text: &'a str) -> Self {
        self.forward_text = Some(text);
        self
    }

    /// Configures `bot_username`.
    pub fn bot_username(mut self, username: &'a str) -> Self {
        self.bot_username = Some(username);
        self
    }

    /// Configures `request_write_access`.
    pub fn request_write_access(mut self, should_request: bool) -> Self {
        self.request_write_access = Some(should_request);
        self
    }
}

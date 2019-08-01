use super::*;
use crate::types::value;

/// Represents a [`LoginUrl`].
///
/// [`LoginUrl`]: https://core.telegram.org/bots/api#loginurl
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
pub struct LoginUrl<'a> {
    url: value::String<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forward_text: Option<value::String<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bot_username: Option<value::String<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_write_access: Option<bool>,
}

impl<'a> LoginUrl<'a> {
    /// Constructs a new `LoginUrl`.
    pub fn new(url: impl Into<value::String<'a>>) -> Self {
        Self {
            url: url.into(),
            forward_text: None,
            bot_username: None,
            request_write_access: None,
        }
    }

    /// Configures `forward_text`.
    pub fn forward_text(mut self, text: impl Into<value::String<'a>>) -> Self {
        self.forward_text = Some(text.into());
        self
    }

    /// Configures `bot_username`.
    pub fn bot_username(
        mut self,
        username: impl Into<value::String<'a>>,
    ) -> Self {
        self.bot_username = Some(username.into());
        self
    }

    /// Configures `request_write_access`.
    pub fn request_write_access(mut self, should_request: bool) -> Self {
        self.request_write_access = Some(should_request);
        self
    }
}

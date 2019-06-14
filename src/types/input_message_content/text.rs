use crate::{methods::parameters::WebPagePreviewState, types::ParseMode};
use serde::Serialize;

/// Represents an [`InputTextMessageContent`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inputtextmessagecontent
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
pub struct Text<'a> {
    message_text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<bool>,
}

impl<'a> Text<'a> {
    /// Constructs a new `Text`.
    pub fn new(message_text: &'a str) -> Self {
        Self {
            message_text,
            parse_mode: None,
            disable_web_page_preview: None,
        }
    }

    /// Configures the parse mode.
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    /// Configures if the web page preview will be shown.
    pub fn web_page_preview(mut self, state: WebPagePreviewState) -> Self {
        self.disable_web_page_preview = Some(state.is_disabled());
        self
    }
}

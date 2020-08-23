use crate::types::{InputMessageContent, InteriorBorrow};
use serde::Serialize;
use std::borrow::Cow;

/// Represents an [`InlineQueryResultCachedSticker`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inlinequeryresultcachedsticker
#[derive(Debug, PartialEq, Clone, Serialize)]
#[must_use]
pub struct Sticker<'a> {
    #[serde(rename = "sticker_file_id")]
    id: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent<'a>>,
}

impl<'a> Sticker<'a> {
    /// Constructs a `Sticker`.
    pub fn new(id: impl Into<Cow<'a, str>>) -> Self {
        Self {
            id: id.into(),
            input_message_content: None,
        }
    }

    /// Configures the content shown after sending the message.
    pub fn input_message_content(
        mut self,
        content: impl Into<InputMessageContent<'a>>,
    ) -> Self {
        self.input_message_content = Some(content.into());
        self
    }
}

impl<'a> InteriorBorrow<'a> for Sticker<'a> {
    fn borrow_inside(&'a self) -> Self {
        Self {
            id: self.id.borrow_inside(),
            input_message_content: self.input_message_content.borrow_inside(),
        }
    }
}

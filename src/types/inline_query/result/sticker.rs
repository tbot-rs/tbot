use crate::types::{
    value::{self, Ref},
    InputMessageContent,
};
use serde::Serialize;

/// Represents an [`InlineQueryResultCachedSticker`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inlinequeryresultcachedsticker
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Sticker<'a> {
    #[serde(rename = "sticker_file_id")]
    id: value::String<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<Ref<'a, InputMessageContent<'a>>>,
}

impl<'a> Sticker<'a> {
    /// Constructs a `Sticker`.
    pub fn new(id: impl Into<value::String<'a>>) -> Self {
        Self {
            id: id.into(),
            input_message_content: None,
        }
    }

    /// Configures the content shown after sending the message.
    pub fn input_message_content(
        mut self,
        content: impl Into<Ref<'a, InputMessageContent<'a>>>,
    ) -> Self {
        self.input_message_content = Some(content.into());
        self
    }
}

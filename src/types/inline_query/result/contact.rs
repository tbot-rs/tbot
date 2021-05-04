use super::Thumb;
use crate::types::InputMessageContent;
use serde::Serialize;

/// Represents an [`InlineQueryResultContact`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inlinequeryresultcontact
#[derive(Debug, PartialEq, Clone, Serialize)]
#[must_use]
pub struct Contact {
    phone_number: String,
    first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcard: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb: Option<Thumb>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
}

impl Contact {
    /// Constructs a 'Contact'.
    pub fn new(
        phone_number: impl Into<String>,
        first_name: impl Into<String>,
    ) -> Self {
        Self {
            phone_number: phone_number.into(),
            first_name: first_name.into(),
            last_name: None,
            vcard: None,
            thumb: None,
            input_message_content: None,
        }
    }

    /// Configures the last name of the contact.
    pub fn last_name(mut self, name: impl Into<String>) -> Self {
        self.last_name = Some(name.into());
        self
    }

    /// Configures the contact's additional data.
    pub fn vcard(mut self, vcard: impl Into<String>) -> Self {
        self.vcard = Some(vcard.into());
        self
    }

    /// Configures the thumb of the contact.
    #[allow(clippy::missing_const_for_fn)]
    pub fn thumb(mut self, thumb: Thumb) -> Self {
        self.thumb = Some(thumb);
        self
    }

    /// Configures the content shown after sending the message.
    pub fn input_message_content(
        mut self,
        content: impl Into<InputMessageContent>,
    ) -> Self {
        self.input_message_content = Some(content.into());
        self
    }
}

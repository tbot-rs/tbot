use super::Thumb;
use crate::types::InputMessageContent;
use serde::Serialize;

/// Represents an [`InlineQueryResultContact`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inlinequeryresultcontact
#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub struct Contact<'a> {
    phone_number: &'a str,
    first_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcard: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb: Option<Thumb<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent<'a>>,
}

impl<'a> Contact<'a> {
    /// Constructs a 'Contact'.
    pub const fn new(phone_number: &'a str, first_name: &'a str) -> Self {
        Self {
            phone_number,
            first_name,
            last_name: None,
            vcard: None,
            thumb: None,
            input_message_content: None,
        }
    }

    /// Configures the last name of the contact.
    pub fn last_name(mut self, name: &'a str) -> Self {
        self.last_name = Some(name);
        self
    }

    /// Configures the contact's additional data.
    pub fn vcard(mut self, vcard: &'a str) -> Self {
        self.vcard = Some(vcard);
        self
    }

    /// Configures the thumb of the contact.
    pub fn thumb(mut self, thumb: Thumb<'a>) -> Self {
        self.thumb = Some(thumb);
        self
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

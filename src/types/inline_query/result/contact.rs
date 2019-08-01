use super::Thumb;
use crate::types::{
    value::{self, Ref},
    InputMessageContent,
};
use serde::Serialize;

/// Represents an [`InlineQueryResultContact`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inlinequeryresultcontact
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Contact<'a> {
    phone_number: value::String<'a>,
    first_name: value::String<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<value::String<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcard: Option<value::String<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb: Option<Ref<'a, Thumb<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<Ref<'a, InputMessageContent<'a>>>,
}

impl<'a> Contact<'a> {
    /// Constructs a 'Contact'.
    pub fn new(
        phone_number: impl Into<value::String<'a>>,
        first_name: impl Into<value::String<'a>>,
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
    pub fn last_name(mut self, name: impl Into<value::String<'a>>) -> Self {
        self.last_name = Some(name.into());
        self
    }

    /// Configures the contact's additional data.
    pub fn vcard(mut self, vcard: impl Into<value::String<'a>>) -> Self {
        self.vcard = Some(vcard.into());
        self
    }

    /// Configures the thumb of the contact.
    pub fn thumb(mut self, thumb: impl Into<Ref<'a, Thumb<'a>>>) -> Self {
        self.thumb = Some(thumb.into());
        self
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

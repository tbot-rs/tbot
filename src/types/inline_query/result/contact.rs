use super::Thumb;
use crate::types::{InputMessageContent, InteriorBorrow};
use serde::Serialize;
use std::borrow::Cow;

/// Represents an [`InlineQueryResultContact`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inlinequeryresultcontact
#[derive(Debug, PartialEq, Clone, Serialize)]
#[must_use]
pub struct Contact<'a> {
    phone_number: Cow<'a, str>,
    first_name: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcard: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb: Option<Thumb<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent<'a>>,
}

impl<'a> Contact<'a> {
    /// Constructs a 'Contact'.
    pub fn new(
        phone_number: impl Into<Cow<'a, str>>,
        first_name: impl Into<Cow<'a, str>>,
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
    pub fn last_name(mut self, name: impl Into<Cow<'a, str>>) -> Self {
        self.last_name = Some(name.into());
        self
    }

    /// Configures the contact's additional data.
    pub fn vcard(mut self, vcard: impl Into<Cow<'a, str>>) -> Self {
        self.vcard = Some(vcard.into());
        self
    }

    /// Configures the thumb of the contact.
    #[allow(clippy::missing_const_for_fn)]
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

impl<'a> InteriorBorrow<'a> for Contact<'a> {
    fn borrow_inside(&'a self) -> Self {
        Self {
            phone_number: self.phone_number.borrow_inside(),
            first_name: self.first_name.borrow_inside(),
            last_name: self.last_name.borrow_inside(),
            vcard: self.vcard.borrow_inside(),
            thumb: self.thumb.borrow_inside(),
            input_message_content: self.input_message_content.borrow_inside(),
        }
    }
}

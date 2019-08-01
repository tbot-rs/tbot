//! Types related to input message contents.

use crate::types::value::Ref;
use serde::Serialize;

mod contact;
mod location;
mod text;
mod venue;

pub use {contact::*, location::*, text::*, venue::*};

/// Represents [`InputMessageContext`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inputmessagecontent
#[derive(Debug, PartialEq, Clone, Serialize)]
#[serde(untagged)]
// todo: #[non_exhaustive]
pub enum InputMessageContent<'a> {
    /// A text message.
    Text(Ref<'a, Text<'a>>),
    /// A location.
    Location(Location),
    /// A venue.
    Venue(Venue<'a>),
    /// A contact.
    Contact(Contact<'a>),
}

impl InputMessageContent<'_> {
    /// Checks if `self` is `Text`.
    pub fn is_text(&self) -> bool {
        match self {
            InputMessageContent::Text(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Location`.
    pub fn is_location(&self) -> bool {
        match self {
            InputMessageContent::Location(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Venue`.
    pub fn is_venue(&self) -> bool {
        match self {
            InputMessageContent::Venue(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Contact.`
    pub fn is_contact(&self) -> bool {
        match self {
            InputMessageContent::Contact(..) => true,
            _ => false,
        }
    }
}

impl<'a> From<Text<'a>> for InputMessageContent<'a> {
    fn from(text: Text<'a>) -> Self {
        InputMessageContent::Text(text.into())
    }
}

impl<'a> From<&'a Text<'a>> for InputMessageContent<'a> {
    fn from(text: &'a Text<'a>) -> Self {
        InputMessageContent::Text(text.into())
    }
}

impl<'a> From<Location> for InputMessageContent<'a> {
    fn from(location: Location) -> Self {
        InputMessageContent::Location(location)
    }
}

impl<'a> From<Venue<'a>> for InputMessageContent<'a> {
    fn from(venue: Venue<'a>) -> Self {
        InputMessageContent::Venue(venue)
    }
}

impl<'a> From<Contact<'a>> for InputMessageContent<'a> {
    fn from(contact: Contact<'a>) -> Self {
        InputMessageContent::Contact(contact)
    }
}

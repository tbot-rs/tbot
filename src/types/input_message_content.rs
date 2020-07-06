//! Types related to input message contents.

use is_macro::Is;
use serde::Serialize;

mod contact;
mod location;
mod text;
mod venue;

pub use {contact::Contact, location::Location, text::Text, venue::Venue};

/// Represents [`InputMessageContext`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inputmessagecontent
#[derive(Debug, PartialEq, Clone, Serialize, Is)]
#[serde(untagged)]
#[non_exhaustive]
pub enum InputMessageContent<'a> {
    /// A text message.
    Text(Text<'a>),
    /// A location.
    Location(Location),
    /// A venue.
    Venue(Venue<'a>),
    /// A contact.
    Contact(Contact<'a>),
}

impl<'a> From<Text<'a>> for InputMessageContent<'a> {
    #[must_use]
    fn from(text: Text<'a>) -> Self {
        InputMessageContent::Text(text)
    }
}

impl<'a> From<Location> for InputMessageContent<'a> {
    #[must_use]
    fn from(location: Location) -> Self {
        InputMessageContent::Location(location)
    }
}

impl<'a> From<Venue<'a>> for InputMessageContent<'a> {
    #[must_use]
    fn from(venue: Venue<'a>) -> Self {
        InputMessageContent::Venue(venue)
    }
}

impl<'a> From<Contact<'a>> for InputMessageContent<'a> {
    #[must_use]
    fn from(contact: Contact<'a>) -> Self {
        InputMessageContent::Contact(contact)
    }
}

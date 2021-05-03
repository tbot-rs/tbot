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
pub enum InputMessageContent {
    /// A text message.
    Text(Text),
    /// A location.
    Location(Location),
    /// A venue.
    Venue(Venue),
    /// A contact.
    Contact(Contact),
}

impl From<Text> for InputMessageContent {
    #[must_use]
    fn from(text: Text) -> Self {
        Self::Text(text)
    }
}

impl From<Location> for InputMessageContent {
    #[must_use]
    fn from(location: Location) -> Self {
        Self::Location(location)
    }
}

impl From<Venue> for InputMessageContent {
    #[must_use]
    fn from(venue: Venue) -> Self {
        Self::Venue(venue)
    }
}

impl From<Contact> for InputMessageContent {
    #[must_use]
    fn from(contact: Contact) -> Self {
        Self::Contact(contact)
    }
}

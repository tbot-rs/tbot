use super::Thumb;
use crate::types::InputMessageContent;
use serde::Serialize;

/// Represents an [`InlineQueryResultVenue`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inlinequeryresultvenue
#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub struct Venue<'a> {
    latitude: f64,
    longitude: f64,
    title: &'a str,
    address: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_type: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    thumb: Option<Thumb<'a>>,
}

impl<'a> Venue<'a> {
    /// Constructs a `Venue`.
    pub const fn new(
        title: &'a str,
        (latitude, longitude): (f64, f64),
        address: &'a str,
    ) -> Self {
        Self {
            latitude,
            longitude,
            title,
            address,
            foursquare_id: None,
            foursquare_type: None,
            input_message_content: None,
            thumb: None,
        }
    }

    /// Configures the Foursquare ID.
    pub fn foursquare_id(mut self, id: &'a str) -> Self {
        self.foursquare_id = Some(id);
        self
    }

    /// Configures the Foursquare type.
    pub fn foursquare_type(mut self, foursquare_type: &'a str) -> Self {
        self.foursquare_type = Some(foursquare_type);
        self
    }

    /// Configures the thumb of the venue.
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

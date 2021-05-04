use super::Thumb;
use crate::types::InputMessageContent;
use serde::Serialize;

/// Represents an [`InlineQueryResultVenue`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inlinequeryresultvenue
#[derive(Debug, PartialEq, Clone, Serialize)]
#[must_use]
pub struct Venue {
    latitude: f64,
    longitude: f64,
    title: String,
    address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_place_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_place_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    thumb: Option<Thumb>,
}

impl Venue {
    /// Constructs a `Venue`.
    pub fn new(
        title: impl Into<String>,
        (latitude, longitude): (f64, f64),
        address: impl Into<String>,
    ) -> Self {
        Self {
            latitude,
            longitude,
            title: title.into(),
            address: address.into(),
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
            input_message_content: None,
            thumb: None,
        }
    }

    /// Configures the Foursquare ID.
    pub fn foursquare_id(mut self, id: impl Into<String>) -> Self {
        self.foursquare_id = Some(id.into());
        self
    }

    /// Configures the Foursquare type.
    pub fn foursquare_type(
        mut self,
        foursquare_type: impl Into<String>,
    ) -> Self {
        self.foursquare_type = Some(foursquare_type.into());
        self
    }

    /// Configures the Google Places ID.
    pub fn google_place_id(mut self, id: impl Into<String>) -> Self {
        self.google_place_id = Some(id.into());
        self
    }

    /// Configures the Google Places type.
    pub fn google_place_type(
        mut self,
        google_place_type: impl Into<String>,
    ) -> Self {
        self.google_place_type = Some(google_place_type.into());
        self
    }

    /// Configures the thumb of the venue.
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

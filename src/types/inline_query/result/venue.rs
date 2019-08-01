use super::Thumb;
use crate::types::{
    value::{self, Ref},
    InputMessageContent,
};
use serde::Serialize;

/// Represents an [`InlineQueryResultVenue`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inlinequeryresultvenue
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Venue<'a> {
    latitude: f64,
    longitude: f64,
    title: value::String<'a>,
    address: value::String<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_id: Option<value::String<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_type: Option<value::String<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<Ref<'a, InputMessageContent<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    thumb: Option<Ref<'a, Thumb<'a>>>,
}

impl<'a> Venue<'a> {
    /// Constructs a `Venue`.
    pub fn new(
        title: impl Into<value::String<'a>>,
        (latitude, longitude): (f64, f64),
        address: impl Into<value::String<'a>>,
    ) -> Self {
        Self {
            latitude,
            longitude,
            title: title.into(),
            address: address.into(),
            foursquare_id: None,
            foursquare_type: None,
            input_message_content: None,
            thumb: None,
        }
    }

    /// Configures the Foursquare ID.
    pub fn foursquare_id(mut self, id: impl Into<value::String<'a>>) -> Self {
        self.foursquare_id = Some(id.into());
        self
    }

    /// Configures the Foursquare type.
    pub fn foursquare_type(
        mut self,
        foursquare_type: impl Into<value::String<'a>>,
    ) -> Self {
        self.foursquare_type = Some(foursquare_type.into());
        self
    }

    /// Configures the thumb of the venue.
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

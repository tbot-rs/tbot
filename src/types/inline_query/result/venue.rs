use super::Thumb;
use crate::types::{InputMessageContent, InteriorBorrow};
use serde::Serialize;
use std::borrow::Cow;

/// Represents an [`InlineQueryResultVenue`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inlinequeryresultvenue
#[derive(Debug, PartialEq, Clone, Serialize)]
#[must_use]
pub struct Venue<'a> {
    latitude: f64,
    longitude: f64,
    title: Cow<'a, str>,
    address: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_id: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_type: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    thumb: Option<Thumb<'a>>,
}

impl<'a> Venue<'a> {
    /// Constructs a `Venue`.
    pub fn new(
        title: impl Into<Cow<'a, str>>,
        (latitude, longitude): (f64, f64),
        address: impl Into<Cow<'a, str>>,
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
    pub fn foursquare_id(mut self, id: impl Into<Cow<'a, str>>) -> Self {
        self.foursquare_id = Some(id.into());
        self
    }

    /// Configures the Foursquare type.
    pub fn foursquare_type(
        mut self,
        foursquare_type: impl Into<Cow<'a, str>>,
    ) -> Self {
        self.foursquare_type = Some(foursquare_type.into());
        self
    }

    /// Configures the thumb of the venue.
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

impl<'a> InteriorBorrow<'a> for Venue<'a> {
    fn borrow_inside(&'a self) -> Self {
        Self {
            title: self.title.borrow_inside(),
            address: self.address.borrow_inside(),
            foursquare_id: self.foursquare_id.borrow_inside(),
            foursquare_type: self.foursquare_type.borrow_inside(),
            input_message_content: self.input_message_content.borrow_inside(),
            thumb: self.thumb.borrow_inside(),
            ..*self
        }
    }
}

use serde::Serialize;
use std::borrow::Cow;

/// Represents an [`InputVenueMessageContent`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inputvenuemessagecontent
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
}

impl<'a> Venue<'a> {
    /// Constructs a `Venue`.
    #[allow(clippy::unused_self)] // https://github.com/rust-lang/rust-clippy/issues/5351
    pub fn new(
        (latitude, longitude): (f64, f64),
        title: impl Into<Cow<'a, str>>,
        address: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            latitude,
            longitude,
            title: title.into(),
            address: address.into(),
            foursquare_id: None,
            foursquare_type: None,
        }
    }

    /// Configures the Foursquare ID.
    pub fn foursquare_id(mut self, id: impl Into<Cow<'a, str>>) -> Self {
        self.foursquare_id = Some(id.into());
        self
    }

    /// Configures the Foursquare type.
    pub fn foursquare_type(mut self, foursquare_type: impl Into<Cow<'a, str>>) -> Self {
        self.foursquare_type = Some(foursquare_type.into());
        self
    }
}

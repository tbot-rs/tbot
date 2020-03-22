use serde::Serialize;

/// Represents an [`InputVenueMessageContent`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inputvenuemessagecontent
#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
#[must_use]
pub struct Venue<'a> {
    latitude: f64,
    longitude: f64,
    title: &'a str,
    address: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_type: Option<&'a str>,
}

impl<'a> Venue<'a> {
    /// Constructs a `Venue`.
    #[allow(clippy::unused_self)] // https://github.com/rust-lang/rust-clippy/issues/5351
    pub const fn new(
        (latitude, longitude): (f64, f64),
        title: &'a str,
        address: &'a str,
    ) -> Self {
        Self {
            latitude,
            longitude,
            title,
            address,
            foursquare_id: None,
            foursquare_type: None,
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
}

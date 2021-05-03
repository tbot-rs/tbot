use serde::Serialize;

/// Represents an [`InputVenueMessageContent`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inputvenuemessagecontent
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
}

impl Venue {
    /// Constructs a `Venue`.
    pub fn new(
        (latitude, longitude): (f64, f64),
        title: impl Into<String>,
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
}

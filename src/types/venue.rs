use super::*;

/// Represents a [`Venue`].
///
/// [`Venue`]: https://core.telegram.org/bots/api#venue
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Venue {
    /// The location of the venue.
    pub location: Location,
    /// The title of the venue.
    pub title: String,
    /// The address of the venue.
    pub address: String,
    /// The foursquare ID of the venue.
    pub foursquare_id: Option<String>,
    /// The foursquare type of the venue.
    pub foursquare_type: Option<String>,
}

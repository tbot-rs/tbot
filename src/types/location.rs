use super::*;

/// Represents a [`Location`].
///
/// [`Location`]: https://core.telegram.org/bots/api#location
#[derive(Debug, PartialEq, Clone, Copy, Deserialize)]
pub struct Location {
    /// The longitude of the location.
    pub longitude: f64,
    /// The latitude of the location.
    pub latitude: f64,
}

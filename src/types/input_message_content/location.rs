use crate::types::parameters::LiveLocation;
use serde::Serialize;

/// Represents an [`InputLocationMessageContent`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inputlocationmessagecontent
#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
#[must_use]
pub struct Location {
    latitude: f64,
    longitude: f64,
    #[serde(flatten)]
    live_location: Option<LiveLocation>,
}

impl Location {
    /// Constructs a `Location`.
    #[allow(clippy::unused_self)] // https://github.com/rust-lang/rust-clippy/issues/5351
    pub const fn new((latitude, longitude): (f64, f64)) -> Self {
        Self {
            latitude,
            longitude,
            live_location: None,
        }
    }

    /// Configures a live location.
    pub const fn live_location(mut self, live_location: LiveLocation) -> Self {
        self.live_location = Some(live_location);
        self
    }
}

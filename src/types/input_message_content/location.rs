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
    #[serde(skip_serializing_if = "Option::is_none")]
    horizontal_accuracy: Option<f64>,
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
            horizontal_accuracy: None,
            live_location: None,
        }
    }

    /// Confgiures a horizontal accuracy.
    ///
    /// # Panics
    ///
    /// Panics if `horizontal_accuracy` is not in range `1.0..=1500.0`.
    pub fn horizontal_accuracy(mut self, horizontal_accuracy: f64) -> Self {
        assert!(
            (1.0..=1500.0).contains(&horizontal_accuracy),
            "[tbot] Received invalid `horizontal_accuracy` in \
                 `Location::horizontal_accuracy`: \
                 {}, must be in range `1.0..=1500.0`",
            horizontal_accuracy,
        );

        self.horizontal_accuracy = Some(horizontal_accuracy);
        self
    }

    /// Configures a live location.
    pub const fn live_location(mut self, live_location: LiveLocation) -> Self {
        self.live_location = Some(live_location);
        self
    }
}

//! Types representing (live) locations.

use serde::{Deserialize, Serialize};

/// Information about a live location.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
#[non_exhaustive]
#[must_use = "a `location::Live` must be used via `SendLocation`, \
`input_message_content::Location` or `inline_query::Location`"]
pub struct Live {
    /// Time relative to the message sending date, during which the
    /// location can be updated, in seconds. For active live locations only.
    pub live_period: u32,
    /// The direction in which user is moving, in degrees and in range
    /// `1..=360`. For active live locations only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<u16>,
    /// Maximum distance for proximity alerts about approaching another
    /// chat member, in meters. For sent live locations only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<u32>,
}

/// Represents a [`Location`].
///
/// [`Location`]: https://core.telegram.org/bots/api#location
#[derive(Debug, PartialEq, Clone, Copy, Deserialize)]
#[non_exhaustive]
pub struct Location {
    /// The longitude of the location.
    pub longitude: f64,
    /// The latitude of the location.
    pub latitude: f64,
    /// The radius of uncertainty for the location, measured in meters and is
    /// in range `0..=1500`.
    pub horizontal_accuracy: Option<f64>,
    /// If this location is a live location, information about it.
    #[serde(flatten)]
    pub live_location: Option<Live>,
}

impl Live {
    /// Creates a new live location. The `live_period` must be in range
    /// `60..=86_400`; it reflects the `live_period` parameter.
    ///
    /// # Panics
    ///
    /// Panics if the duration is not in range `60..=86_400`.
    pub fn new(live_period: u32) -> Self {
        assert!(
            (60..=86_400).contains(&live_period),
            "[tbot] Received invalid `live_period` in `LiveLocation::new`: \
             {}, must be in range `60..=86_400`",
            live_period,
        );

        Self {
            live_period,
            heading: None,
            proximity_alert_radius: None,
        }
    }

    /// Configures the direction in which the user is headed. The value must be
    /// in range `1..=360`. Reflects the `heading` parameter.
    ///
    /// # Panics
    ///
    /// Panics if `heading` is not in range `1..=360`.
    pub fn heading(mut self, heading: u16) -> Self {
        assert!(
            (1..=360).contains(&heading),
            "[tbot] Received invalid `heading` in `LiveLocation::heading`: \
             {}, must be in range `1..=360`",
            heading,
        );

        self.heading = Some(heading);
        self
    }

    /// Configures the maximum distance for proximity alerts about a user
    /// approaching another, in meters. The value must be in range
    /// `1..=100_000`. Reflects the `proximity_alert_radius` paramter.
    ///
    /// # Panics
    ///
    /// Panics if the value is not in range `1..=100_000`.
    pub fn proximity_alert_radius(mut self, radius: u32) -> Self {
        assert!(
            (1..=100_000).contains(&radius),
            "[tbot] Received invalid `radius` in \
             `LiveLocation::proximity_alert_radius`: {}, must be in range \
             `1..=100_000`",
            radius,
        );

        self.proximity_alert_radius = Some(radius);
        self
    }
}

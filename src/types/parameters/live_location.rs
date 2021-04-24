use serde::Serialize;
use std::{
    convert::TryInto,
    num::{NonZeroU16, NonZeroU32},
};

/// Represents a live location to be sent.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[must_use = "a `LiveLocation` must be used via `SendLocation`, \
`input_message_content::Location` or `inline_query::Location`"]
pub struct LiveLocation {
    live_period: NonZeroU32,
    #[serde(skip_serializing_if = "Option::is_none")]
    heading: Option<NonZeroU16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proximity_alert_radius: Option<NonZeroU32>,
}

impl LiveLocation {
    /// Creates a new `LiveLocation`. The `live_period` must be in range \
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
            live_period: live_period.try_into().unwrap(),
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

        self.heading = Some(heading.try_into().unwrap());
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

        self.proximity_alert_radius = Some(radius.try_into().unwrap());
        self
    }
}

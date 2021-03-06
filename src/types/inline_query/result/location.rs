use super::Thumb;
use crate::types::{location, InputMessageContent};
use serde::Serialize;

/// Represents an [`InlineQueryResultLocation`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inlinequeryresultlocation
#[derive(Debug, PartialEq, Clone, Serialize)]
#[must_use]
pub struct Location {
    latitude: f64,
    longitude: f64,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    horizontal_accuracy: Option<f64>,
    #[serde(flatten)]
    live_location: Option<location::Live>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    thumb: Option<Thumb>,
}

impl Location {
    /// Constructs a `Location`.
    pub fn new(
        title: impl Into<String>,
        (latitude, longitude): (f64, f64),
    ) -> Self {
        Self {
            latitude,
            longitude,
            title: title.into(),
            horizontal_accuracy: None,
            live_location: None,
            input_message_content: None,
            thumb: None,
        }
    }

    /// Configures the radius of uncertainty for the location in meters, in range `1.0..=1500.0`.
    ///
    /// # Panics
    ///
    /// Panics if `horizontal_accuracy` is not in range `1.0..=1500.0`.
    pub fn horizontal_accuracy(mut self, horizontal_accuracy: f64) -> Self {
        assert!(
            (1.0..=1500.0).contains(&horizontal_accuracy),
            "[tbot] Received invalid `horizontal_accuracy` in \
             `Location::horizontal_accuracy`: {}, must be in range \
             `1.0..=1500.0`",
            horizontal_accuracy,
        );

        self.horizontal_accuracy = Some(horizontal_accuracy);
        self
    }

    /// Configures a live location.
    pub const fn live_location(
        mut self,
        live_location: location::Live,
    ) -> Self {
        self.live_location = Some(live_location);
        self
    }

    /// Configures the content shown after sending the message.
    pub fn input_message_content(
        mut self,
        content: impl Into<InputMessageContent>,
    ) -> Self {
        self.input_message_content = Some(content.into());
        self
    }

    /// Configures the thumb of the location.
    #[allow(clippy::missing_const_for_fn)]
    pub fn thumb(mut self, thumb: Thumb) -> Self {
        self.thumb = Some(thumb);
        self
    }
}

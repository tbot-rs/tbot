use super::Thumb;
use crate::types::InputMessageContent;
use serde::Serialize;

/// Represents an [`InlineQueryResultLocation`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inlinequeryresultlocation
#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
#[must_use]
pub struct Location<'a> {
    latitude: f64,
    longitude: f64,
    title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    live_period: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    thumb: Option<Thumb<'a>>,
}

impl<'a> Location<'a> {
    /// Constructs a `Location`.
    pub const fn new(
        title: &'a str,
        (latitude, longitude): (f64, f64),
    ) -> Self {
        Self {
            latitude,
            longitude,
            title,
            live_period: None,
            input_message_content: None,
            thumb: None,
        }
    }

    /// Configures the period while the location will be live.
    pub fn live_period(mut self, period: u64) -> Self {
        self.live_period = Some(period);
        self
    }

    /// Configures the content shown after sending the message.
    pub fn input_message_content(
        mut self,
        content: impl Into<InputMessageContent<'a>>,
    ) -> Self {
        self.input_message_content = Some(content.into());
        self
    }

    /// Configures the thumb of the location.
    pub fn thumb(mut self, thumb: Thumb<'a>) -> Self {
        self.thumb = Some(thumb);
        self
    }
}

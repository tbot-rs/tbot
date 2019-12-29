use is_macro::Is;
use serde::{Deserialize, Serialize};

/// Represents update types to subscribe with [`Webhook`] or [`Polling`].
///
/// [`Webhook`]: ../../event_loop/struct.Webhook.html
/// [`Polling`]: ../../event_loop/struct.Polling.html
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize, Is,
)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
#[must_use]
pub enum UpdateKind {
    /// Handles chat messages of any kind.
    Message,
    /// Handles chat message edits.
    EditedMessage,
    /// Handles channel posts of any kind.
    ChannelPost,
    /// Handles channel post edits.
    EditedChannelPost,
    /// Handles inline queries.
    InlineQuery,
    /// Handles chosen inline results.
    ChosenInlineResult,
    /// Handles inline button clicks.
    CallbackQuery,
    /// Handles shpping query.
    ShippingQuery,
    /// Handles pre-checkout query.
    PreCheckoutQuery,
    /// Handles poll state updates.
    Poll,
}

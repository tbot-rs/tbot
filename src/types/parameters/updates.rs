use serde::{Deserialize, Serialize};

/// Represents update types to subscribe with [`Webhook`] or [`Polling`].
///
/// [`Webhook`]: ../../event_loop/struct.Webhook.html
/// [`Polling`]: ../../event_loop/struct.Polling.html
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
// todo: #[non_exhaustive]
pub enum Updates {
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

impl Updates {
    /// Checks if `self` is `Message`.
    pub fn is_message(self) -> bool {
        self == Self::Message
    }

    /// Checks if `self` is `EditedMessage`.
    pub fn is_edited_message(self) -> bool {
        self == Self::EditedMessage
    }

    /// Checks if `self` is ChanelPost``.
    pub fn is_channel_post(self) -> bool {
        self == Self::ChannelPost
    }

    /// Checks if `self` is `EditedChannelPost`.
    pub fn is_edited_channel_post(self) -> bool {
        self == Self::EditedChannelPost
    }

    /// Checks if `self` is `InlineQuery`.
    pub fn is_inline_query(self) -> bool {
        self == Self::InlineQuery
    }

    /// Checks if `self` is `ChosenInlineResult`.
    pub fn is_chosen_inline_result(self) -> bool {
        self == Self::ChosenInlineResult
    }

    /// Checks if `self` is `CallbackQuery`.
    pub fn is_callback_query(self) -> bool {
        self == Self::CallbackQuery
    }

    /// Checks if `self` is `ShippingQuery`.
    pub fn is_shipping_query(self) -> bool {
        self == Self::ShippingQuery
    }

    /// Checks if `self` is `PreCheckoutQuery`.
    pub fn is_pre_checkout_query(self) -> bool {
        self == Self::PreCheckoutQuery
    }

    /// Checks if `self` is `Poll`.
    pub fn is_poll(self) -> bool {
        self == Self::Poll
    }
}

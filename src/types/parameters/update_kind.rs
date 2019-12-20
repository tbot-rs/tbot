use serde::{Deserialize, Serialize};

/// Represents update types to subscribe with [`Webhook`] or [`Polling`].
///
/// [`Webhook`]: ../../event_loop/struct.Webhook.html
/// [`Polling`]: ../../event_loop/struct.Polling.html
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
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

impl UpdateKind {
    /// Checks if `self` is `Message`.
    #[must_use]
    pub fn is_message(self) -> bool {
        self == Self::Message
    }

    /// Checks if `self` is `EditedMessage`.
    #[must_use]
    pub fn is_edited_message(self) -> bool {
        self == Self::EditedMessage
    }

    /// Checks if `self` is `ChanelPost`.
    #[must_use]
    pub fn is_channel_post(self) -> bool {
        self == Self::ChannelPost
    }

    /// Checks if `self` is `EditedChannelPost`.
    #[must_use]
    pub fn is_edited_channel_post(self) -> bool {
        self == Self::EditedChannelPost
    }

    /// Checks if `self` is `InlineQuery`.
    #[must_use]
    pub fn is_inline_query(self) -> bool {
        self == Self::InlineQuery
    }

    /// Checks if `self` is `ChosenInlineResult`.
    #[must_use]
    pub fn is_chosen_inline_result(self) -> bool {
        self == Self::ChosenInlineResult
    }

    /// Checks if `self` is `CallbackQuery`.
    #[must_use]
    pub fn is_callback_query(self) -> bool {
        self == Self::CallbackQuery
    }

    /// Checks if `self` is `ShippingQuery`.
    #[must_use]
    pub fn is_shipping_query(self) -> bool {
        self == Self::ShippingQuery
    }

    /// Checks if `self` is `PreCheckoutQuery`.
    #[must_use]
    pub fn is_pre_checkout_query(self) -> bool {
        self == Self::PreCheckoutQuery
    }

    /// Checks if `self` is `Poll`.
    #[must_use]
    pub fn is_poll(self) -> bool {
        self == Self::Poll
    }
}

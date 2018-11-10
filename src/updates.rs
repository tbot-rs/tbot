/// Represents updates names to subscribe with Webhooks or getUpdates.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UpdateTypes {
    /// Handles messages in a chat of any kind.
    Message,
    /// Handles a message edit.
    EditedMessage,
    /// Handles a message in a channel of any kind.
    ChannelPost,
    /// Handles a channel message edit.
    EditedChannelPost,
    /// Handles inline queries (when you type your bot's username in the
    /// beginning of a message)
    InlineQuery,
    /// When subscribed for chosen inline results, handles them.
    ChosenInlineResult,
    /// Handles inline buttons clicks.
    CallbackQuery,
    /// Handles shpping query.
    ShippingQuery,
    /// Handles pre-checkout query.
    PreCheckoutQuery,
}

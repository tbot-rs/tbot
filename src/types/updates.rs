use super::*;

/// Represents updates names to subscribe with Webhooks or getUpdates.
#[derive(Serialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Updates {
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

#[derive(Deserialize, Debug)]
pub(crate) struct Update {
    pub update_id: u64,
    pub message: Option<raw::Message>,
    pub edited_message: Option<raw::Message>,
    pub channel_post: Option<raw::Message>,
    pub edited_channel_post: Option<raw::Message>,
}

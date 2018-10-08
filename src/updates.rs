/// Represents updates names to subscribe with Webhooks or getUpdates.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UpdateTypes {
    Message,
    EditedMessage,
    ChannelPost,
    EditedChannelPost,
    InlineQuery,
    ChosenInlineResult,
    CallbackQuery,
    ShippingQuery,
    PreCheckoutQuery,
}

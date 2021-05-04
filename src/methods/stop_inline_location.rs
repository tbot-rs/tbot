use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{keyboard::inline, InlineMessageId},
};
use serde::Serialize;

/// Stops a live location sent via the inline mode.
///
/// Reflects the [`stopMessageLiveLocation`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#stopmessagelivelocation
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct StopInlineLocation<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    inline_message_id: InlineMessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<inline::Keyboard<'a>>,
}

impl<'a> StopInlineLocation<'a> {
    pub(crate) const fn new(
        bot: &'a InnerBot,
        inline_message_id: InlineMessageId,
    ) -> Self {
        Self {
            bot,
            inline_message_id,
            reply_markup: None,
        }
    }

    /// Configures an inline keyboard for the message.
    /// Reflects the `reply_markup` parameter.
    pub const fn reply_markup(mut self, markup: inline::Keyboard<'a>) -> Self {
        self.reply_markup = Some(markup);
        self
    }
}

impl StopInlineLocation<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.bot,
            "stopMessageLiveLocation",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}

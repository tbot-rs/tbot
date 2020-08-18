use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{inline_message_id, keyboard::inline},
};
use serde::Serialize;

/// Edits the inline keyboard of a message sent via the inline mode.
///
/// Reflects the [`editMessageReplyMarkup`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagereplymarkup
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditInlineReplyMarkup<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    inline_message_id: inline_message_id::Ref<'a>,
    reply_markup: inline::Keyboard<'a>,
}

impl<'a> EditInlineReplyMarkup<'a> {
    pub(crate) const fn new(
        bot: &'a InnerBot,
        inline_message_id: inline_message_id::Ref<'a>,
        reply_markup: inline::Keyboard<'a>,
    ) -> Self {
        Self {
            bot,
            inline_message_id,
            reply_markup,
        }
    }
}

impl EditInlineReplyMarkup<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.bot,
            "editMessageReplyMarkup",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}

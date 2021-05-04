use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{
        inline_message_id::InlineMessageId,
        keyboard::inline,
        parameters::{ParseMode, Text},
    },
};
use serde::Serialize;

/// Edits the caption of a media message sent via the inline mode.
///
/// Reflects the [`editMessageCaption`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagecaption
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditInlineCaption<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    inline_message_id: InlineMessageId,
    caption: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<inline::Keyboard>,
}

impl<'a> EditInlineCaption<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        inline_message_id: InlineMessageId,
        caption: impl Into<Text>,
    ) -> Self {
        let caption = caption.into();

        Self {
            bot,
            inline_message_id,
            caption: caption.text,
            parse_mode: caption.parse_mode,
            reply_markup: None,
        }
    }

    /// Configures an inline keyboard for the message.
    /// Reflects the `reply_markup` parameter.
    #[allow(clippy::missing_const_for_fn)]
    pub fn reply_markup(mut self, markup: inline::Keyboard) -> Self {
        self.reply_markup = Some(markup);
        self
    }
}

impl EditInlineCaption<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.bot,
            "editMessageCaption",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}

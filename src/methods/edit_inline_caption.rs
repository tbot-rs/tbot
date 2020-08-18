use super::call_method;
use crate::{
    connectors::Client,
    errors, token,
    types::{
        inline_message_id::InlineMessageId,
        keyboard::inline,
        parameters::{ParseMode, Text},
    },
};
use serde::Serialize;
use std::borrow::Cow;

/// Edits the caption of a media message sent via the inline mode.
///
/// Reflects the [`editMessageCaption`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagecaption
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditInlineCaption<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    inline_message_id: InlineMessageId<'a>,
    caption: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<inline::Keyboard<'a>>,
}

impl<'a> EditInlineCaption<'a> {
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        inline_message_id: InlineMessageId<'a>,
        caption: impl Into<Text<'a>>,
    ) -> Self {
        let caption = caption.into();

        Self {
            client,
            token,
            inline_message_id,
            caption: caption.text,
            parse_mode: caption.parse_mode,
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

impl EditInlineCaption<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.client,
            self.token,
            "editMessageCaption",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}

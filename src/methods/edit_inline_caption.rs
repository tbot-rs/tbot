use super::send_method;
use crate::{
    connectors::Connector,
    errors,
    internal::Client,
    types::{
        inline_message_id,
        keyboard::inline,
        parameters::{ParseMode, Text},
    },
    Token,
};
use serde::Serialize;

/// Edits the caption of a media message sent via the inline mode.
///
/// Reflects the [`editMessageCaption`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagecaption
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditInlineCaption<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    inline_message_id: inline_message_id::Ref<'a>,
    caption: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<inline::Keyboard<'a>>,
}

impl<'a, C> EditInlineCaption<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        inline_message_id: inline_message_id::Ref<'a>,
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
    pub fn reply_markup(mut self, markup: inline::Keyboard<'a>) -> Self {
        self.reply_markup = Some(markup);
        self
    }
}

impl<C: Connector> EditInlineCaption<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        send_method::<bool, _>(
            self.client,
            &self.token,
            "editMessageCaption",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}

use super::send_method;
use crate::{
    connectors::Client,
    errors, token,
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
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    inline_message_id: inline_message_id::Ref<'a>,
    reply_markup: inline::Keyboard<'a>,
}

impl<'a> EditInlineReplyMarkup<'a> {
    pub(crate) const fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        inline_message_id: inline_message_id::Ref<'a>,
        reply_markup: inline::Keyboard<'a>,
    ) -> Self {
        Self {
            client,
            token,
            inline_message_id,
            reply_markup,
        }
    }
}

impl EditInlineReplyMarkup<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        send_method::<bool>(
            self.client,
            self.token,
            "editMessageReplyMarkup",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}

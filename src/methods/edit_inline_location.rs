use super::call_method;
use crate::{
    connectors::Client,
    errors, token,
    types::{inline_message_id::InlineMessageId, keyboard::inline},
};
use serde::Serialize;

/// Edits a live location sent via the inline mode.
///
/// Reflects the [`editMessageLiveLocation`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagelivelocation
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditInlineLocation<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    inline_message_id: InlineMessageId<'a>,
    latitude: f64,
    longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<inline::Keyboard<'a>>,
}

impl<'a> EditInlineLocation<'a> {
    pub(crate) const fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        inline_message_id: InlineMessageId<'a>,
        (latitude, longitude): (f64, f64),
    ) -> Self {
        Self {
            client,
            token,
            inline_message_id,
            latitude,
            longitude,
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

impl EditInlineLocation<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.client,
            self.token,
            "editMessageLiveLocation",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}

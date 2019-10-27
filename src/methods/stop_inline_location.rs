use super::send_method;
use crate::{
    connectors::Connector,
    errors,
    internal::Client,
    types::{inline_message_id, keyboard::inline},
    token,
};
use serde::Serialize;

/// Stops a live location sent via the inline mode.
///
/// Reflects the [`stopMessageLiveLocation`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#stopmessagelivelocation
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct StopInlineLocation<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: token::Ref<'a>,
    inline_message_id: inline_message_id::Ref<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<inline::Keyboard<'a>>,
}

impl<'a, C> StopInlineLocation<'a, C> {
    pub(crate) const fn new(
        client: &'a Client<C>,
        token: token::Ref<'a>,
        inline_message_id: inline_message_id::Ref<'a>,
    ) -> Self {
        Self {
            client,
            token,
            inline_message_id,
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

impl<C: Connector> StopInlineLocation<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        send_method::<bool, _>(
            self.client,
            self.token,
            "stopMessageLiveLocation",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}

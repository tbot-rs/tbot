use super::*;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    types::{inline_message_id, keyboard::inline},
};

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
    token: Token,
    inline_message_id: inline_message_id::Ref<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<inline::Keyboard<'a>>,
}

impl<'a, C> StopInlineLocation<'a, C> {
    pub(crate) const fn new(
        client: &'a Client<C>,
        token: Token,
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

impl<C> IntoFuture for StopInlineLocation<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = ();
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        Box::new(
            send_method::<bool, C>(
                self.client,
                &self.token,
                "stopMessageLiveLocation",
                None,
                serde_json::to_vec(&self).unwrap(),
            )
            .map(|_| ()), // Only `true` is returned on success
        )
    }
}

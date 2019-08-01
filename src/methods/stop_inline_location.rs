use super::*;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    types::{
        keyboard::inline,
        value::{InlineMessageId, Ref},
    },
};

/// Represents the [`stopMessageLiveLocation`][docs] method for inline messages.
///
/// [docs]: https://core.telegram.org/bots/api#stopmessagelivelocation
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct StopInlineLocation<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    inline_message_id: InlineMessageId<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<Ref<'a, inline::Keyboard<'a>>>,
}

impl<'a, C> StopInlineLocation<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        inline_message_id: impl Into<InlineMessageId<'a>>,
    ) -> Self {
        Self {
            client,
            token,
            inline_message_id: inline_message_id.into(),
            reply_markup: None,
        }
    }

    /// Configures `reply_markup`.
    pub fn reply_markup(
        mut self,
        markup: impl Into<Ref<'a, inline::Keyboard<'a>>>,
    ) -> Self {
        self.reply_markup = Some(markup.into());
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

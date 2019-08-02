use super::*;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    types::{
        callback,
        parameters::CallbackAction,
        value::{self, Ref, Value},
    },
};

/// Represents the [`answerCallbackQuery`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#answercallbackquery
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct AnswerCallbackQuery<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    callback_query_id: &'a callback::query::Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<value::String<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_alert: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<value::String<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_time: Option<u64>,
}

impl<'a, C> AnswerCallbackQuery<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        callback_query_id: &'a callback::query::Id,
        action: impl Into<Ref<'a, CallbackAction<'a>>>,
    ) -> Self {
        let (text, show_alert, url) = match action.into() {
            Value::Owned(CallbackAction::None)
            | Value::Borrowed(CallbackAction::None) => (None, None, None),
            Value::Owned(CallbackAction::Text(text, show_alert)) => {
                (Some(text), Some(show_alert), None)
            }
            Value::Owned(CallbackAction::Url(url)) => (None, None, Some(url)),
            Value::Borrowed(CallbackAction::Text(text, show_alert)) => {
                (Some(text.into()), Some(*show_alert), None)
            }
            Value::Borrowed(CallbackAction::Url(url)) => {
                (None, None, Some(url.into()))
            }
        };

        Self {
            client,
            token,
            callback_query_id,
            text,
            show_alert,
            url,
            cache_time: None,
        }
    }

    /// Configures `cache_time`.
    pub fn cache_time(mut self, time: u64) -> Self {
        self.cache_time = Some(time);
        self
    }
}

impl<C> IntoFuture for AnswerCallbackQuery<'_, C>
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
                "answerCallbackQuery",
                None,
                serde_json::to_vec(&self).unwrap(),
            )
            .map(|_| ()), // Only `true` is returned on success
        )
    }
}

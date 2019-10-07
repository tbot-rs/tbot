use super::*;
use crate::{
    connectors::Connector,
    errors,
    internal::{BoxFuture, Client},
    types::{callback, parameters::CallbackAction},
};

/// Answers a callback query.
///
/// Reflects the [`answerCallbackQuery`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#answercallbackquery
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct AnswerCallbackQuery<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    callback_query_id: callback::query::id::Ref<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_alert: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_time: Option<u64>,
}

impl<'a, C> AnswerCallbackQuery<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        callback_query_id: callback::query::id::Ref<'a>,
        action: CallbackAction<'a>,
    ) -> Self {
        Self {
            client,
            token,
            callback_query_id,
            text: action.to_text(),
            show_alert: action.to_show_alert(),
            url: action.to_url(),
            cache_time: None,
        }
    }

    /// Configures the amount of time (in seconds) for which the answer may be
    /// cached. Reflects the `cache_time` parameter.
    pub fn cache_time(mut self, time: u64) -> Self {
        self.cache_time = Some(time);
        self
    }
}

impl<C: Connector> IntoFuture for AnswerCallbackQuery<'_, C> {
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

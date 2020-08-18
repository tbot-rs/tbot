use super::call_method;
use crate::{
    connectors::Client,
    errors, token,
    types::{callback, parameters::CallbackAction},
};
use serde::Serialize;
use std::borrow::Cow;

/// Answers a callback query.
///
/// Reflects the [`answerCallbackQuery`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#answercallbackquery
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct AnswerCallbackQuery<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    callback_query_id: callback::query::Id<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_alert: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_time: Option<u64>,
}

impl<'a> AnswerCallbackQuery<'a> {
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        callback_query_id: callback::query::Id<'a>,
        action: CallbackAction<'a>,
    ) -> Self {
        match action {
            CallbackAction::None => Self {
                client,
                token,
                callback_query_id,
                text: None,
                show_alert: None,
                url: None,
                cache_time: None,
            },
            CallbackAction::Url(url) => Self {
                client,
                token,
                callback_query_id,
                text: None,
                show_alert: None,
                url: Some(url),
                cache_time: None,
            },
            CallbackAction::Text(text, show_alert) => Self {
                client,
                token,
                callback_query_id,
                text: Some(text),
                show_alert: Some(show_alert),
                url: None,
                cache_time: None,
            },
        }
    }

    /// Configures the amount of time (in seconds) for which the answer may be
    /// cached. Reflects the `cache_time` parameter.
    pub const fn cache_time(mut self, time: u64) -> Self {
        self.cache_time = Some(time);
        self
    }
}

impl AnswerCallbackQuery<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.client,
            self.token,
            "answerCallbackQuery",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}

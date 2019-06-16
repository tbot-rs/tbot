// use super::*;
use super::{send_method, DeliveryError};
use crate::{internal::Client, prelude::*, types::inline_query, Token};
use serde::Serialize;

/// Represents the [`answerInlineQuery`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#answerinlinequery
#[derive(Debug, Clone, Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct AnswerInlineQuery<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    inline_query_id: inline_query::id::Ref<'a>,
    results: &'a [inline_query::Result<'a>],
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_personal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_offset: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    switch_pm_text: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    switch_pm_parameter: Option<&'a str>,
}

impl<'a, C> AnswerInlineQuery<'a, C> {
    pub(crate) const fn new(
        client: &'a Client<C>,
        token: Token,
        inline_query_id: inline_query::id::Ref<'a>,
        results: &'a [inline_query::Result<'a>],
    ) -> Self {
        Self {
            client,
            token,
            inline_query_id,
            results,
            cache_time: None,
            is_personal: None,
            next_offset: None,
            switch_pm_text: None,
            switch_pm_parameter: None,
        }
    }

    /// Configures `cache_time`.
    pub fn cache_time(mut self, time: u64) -> Self {
        self.cache_time = Some(time);
        self
    }

    /// Configures `is_personal`
    pub fn personal(mut self, is_personal: bool) -> Self {
        self.is_personal = Some(is_personal);
        self
    }

    /// Configures `next_offset`.
    pub fn next_offset(mut self, offset: &'a str) -> Self {
        self.next_offset = Some(offset);
        self
    }

    /// Configures `switch_pm_text` and `switch_pm_parameter`.
    pub fn switch_pm(mut self, text: &'a str, parameter: &'a str) -> Self {
        self.switch_pm_text = Some(text);
        self.switch_pm_parameter = Some(parameter);
        self
    }
}

impl<C> IntoFuture for AnswerInlineQuery<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = ();
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(
            send_method::<bool, C>(
                self.client,
                &self.token,
                "answerInlineQuery",
                None,
                serde_json::to_vec(&self).unwrap(),
            )
            .map(|_| ()), // Only `true` is returned on success
        )
    }
}

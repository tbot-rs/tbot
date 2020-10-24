//! Types for representing [`InlineQueryResult::Article`][docs].
//!
//! [docs]: ../enum.InlineQueryResult.html#variant.Article

use super::Thumb;
#[allow(deprecated)]
use crate::types::{parameters::UrlVisibility, InputMessageContent};
use serde::Serialize;

/// Represents an [`InlineQueryResultArticle`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inlinequeryresultarticle
#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
#[must_use]
pub struct Article<'a> {
    title: &'a str,
    input_message_content: InputMessageContent<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hide_url: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    thumb: Option<Thumb<'a>>,
}

impl<'a> Article<'a> {
    /// Constructs an `Article`.
    pub fn new(
        title: &'a str,
        input_message_content: impl Into<InputMessageContent<'a>>,
    ) -> Self {
        Self {
            title,
            input_message_content: input_message_content.into(),
            url: None,
            hide_url: None,
            description: None,
            thumb: None,
        }
    }

    /// Configures the URL of article.
    pub fn url(mut self, url: &'a str) -> Self {
        self.url = Some(url);
        self
    }

    /// Configures the article's URL visibility.
    pub fn is_url_hidden(mut self, is_hidden: bool) -> Self {
        self.hide_url = Some(is_hidden);
        self
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "use`is_url_hidden` which takes a `bool`"
    )]
    #[allow(deprecated)]
    pub fn url_visibility(self, visibility: UrlVisibility) -> Self {
        self.is_url_hidden(visibility.is_hidden())
    }

    /// Configures the thumb of the article.
    pub fn thumb(mut self, thumb: Thumb<'a>) -> Self {
        self.thumb = Some(thumb);
        self
    }

    /// Configures the description of the result.
    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }
}

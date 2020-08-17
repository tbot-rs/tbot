//! Types for representing [`InlineQueryResult::Article`][docs].
//!
//! [docs]: ../enum.InlineQueryResult.html#variant.Article

use super::Thumb;
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
    pub const fn url(mut self, url: &'a str) -> Self {
        self.url = Some(url);
        self
    }

    /// Configures the article's URL visibility.
    pub fn url_visibility(mut self, visibility: UrlVisibility) -> Self {
        self.hide_url = Some(visibility.is_hidden());
        self
    }

    /// Configures the thumb of the article.
    pub const fn thumb(mut self, thumb: Thumb<'a>) -> Self {
        self.thumb = Some(thumb);
        self
    }

    /// Configures the description of the result.
    pub const fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }
}

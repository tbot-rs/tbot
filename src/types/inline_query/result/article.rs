//! Types for representing [`inline_query::result::Kind::Article`].
//!
//! [`inline_query::result::Kind::Article`]: super::Kind::Article

use super::Thumb;
use crate::types::InputMessageContent;
use serde::Serialize;

/// Represents an [`InlineQueryResultArticle`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inlinequeryresultarticle
#[derive(Debug, PartialEq, Clone, Serialize)]
#[must_use]
pub struct Article {
    title: String,
    input_message_content: InputMessageContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hide_url: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    thumb: Option<Thumb>,
}

impl Article {
    /// Constructs an `Article`.
    pub fn new(
        title: impl Into<String>,
        input_message_content: impl Into<InputMessageContent>,
    ) -> Self {
        Self {
            title: title.into(),
            input_message_content: input_message_content.into(),
            url: None,
            hide_url: None,
            description: None,
            thumb: None,
        }
    }

    /// Configures the URL of article.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Configures whether the article's URL is hidden.
    pub const fn is_url_hidden(mut self, is_hidden: bool) -> Self {
        self.hide_url = Some(is_hidden);
        self
    }

    /// Configures the thumb of the article.
    #[allow(clippy::missing_const_for_fn)]
    pub fn thumb(mut self, thumb: Thumb) -> Self {
        self.thumb = Some(thumb);
        self
    }

    /// Configures the description of the result.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

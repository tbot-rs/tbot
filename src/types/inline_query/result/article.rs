//! Types for representing [`InlineQueryResult::Article`][docs].
//!
//! [docs]: ../enum.InlineQueryResult.html#variant.Article

use super::Thumb;
use crate::types::{
    parameters::UrlVisibility,
    value::{self, Ref},
    InputMessageContent,
};
use serde::Serialize;

/// Represents an [`InlineQueryResultArticle`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inlinequeryresultarticle
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Article<'a> {
    title: value::String<'a>,
    input_message_content: Ref<'a, InputMessageContent<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<value::String<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hide_url: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<value::String<'a>>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    thumb: Option<Ref<'a, Thumb<'a>>>,
}

impl<'a> Article<'a> {
    /// Constructs an `Article`.
    pub fn new(
        title: impl Into<value::String<'a>>,
        input_message_content: impl Into<Ref<'a, InputMessageContent<'a>>>,
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
    pub fn url(mut self, url: impl Into<value::String<'a>>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Configures the article's URL visibility.
    pub fn url_visibility(mut self, visibility: UrlVisibility) -> Self {
        self.hide_url = Some(visibility.is_hidden());
        self
    }

    /// Configures the thumb of the article.
    pub fn thumb(mut self, thumb: impl Into<Ref<'a, Thumb<'a>>>) -> Self {
        self.thumb = Some(thumb.into());
        self
    }

    /// Configures the description of the result.
    pub fn description(
        mut self,
        description: impl Into<value::String<'a>>,
    ) -> Self {
        self.description = Some(description.into());
        self
    }
}

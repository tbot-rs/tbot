//! Types for representing [`InlineQueryResult::Document`][docs].
//!
//! [docs]: ../enum.InlineQueryResult.html#variant.Document

use super::Thumb;
use crate::types::{
    parameters::{ParseMode, Text},
    InputMessageContent,
};
use is_macro::Is;
use serde::Serialize;
use std::borrow::Cow;

/// Represents possible MIME types for a fresh document.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Is)]
#[non_exhaustive]
#[must_use]
pub enum MimeType {
    /// The `application/pdf` MIME type.
    #[serde(rename = "application/pdf")]
    ApplicationPdf,
    /// The `application/zip` MIME type.
    #[serde(rename = "application/zip")]
    ApplicationZip,
}

/// Represents a non-cached document.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[must_use]
pub struct Fresh<'a> {
    #[serde(rename = "document_url")]
    url: Cow<'a, str>,
    mime_type: MimeType,
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    thumb: Option<Thumb<'a>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[serde(untagged)]
#[must_use]
enum Kind<'a> {
    Cached {
        #[serde(rename = "document_file_id")]
        id: Cow<'a, str>,
    },
    Fresh(Fresh<'a>),
}

/// Represents an [`InlineQueryResultDocument`]/[`InlineQueryResultCachedDocument`].
///
/// [`InlineQueryResultDocument`]: https://core.telegram.org/bots/api#inlinequeryresultdocument
/// [`InlineQueryResultCachedDocument`]: https://core.telegram.org/bots/api#inlinequeryresultcacheddocument
#[derive(Debug, PartialEq, Clone, Serialize)]
#[must_use]
pub struct Document<'a> {
    #[serde(flatten)]
    kind: Kind<'a>,
    title: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent<'a>>,
}

impl<'a> Fresh<'a> {
    /// Constructs a `Fresh` document.
    pub fn new(url: impl Into<Cow<'a, str>>, mime_type: MimeType) -> Self {
        Self {
            url: url.into(),
            mime_type,
            thumb: None,
        }
    }

    /// Configures the thumb of the document.
    pub fn thumb(mut self, thumb: Thumb<'a>) -> Self {
        self.thumb = Some(thumb);
        self
    }
}

impl<'a> Document<'a> {
    fn new(title: impl Into<Cow<'a, str>>, kind: Kind<'a>) -> Self {
        Self {
            kind,
            title: title.into(),
            description: None,
            caption: None,
            parse_mode: None,
            input_message_content: None,
        }
    }

    /// Constructs a cached `Document` result.
    pub fn cached(
        title: impl Into<Cow<'a, str>>,
        id: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self::new(title, Kind::Cached { id: id.into() })
    }

    /// Constructs a fresh `Document` result.
    pub fn fresh(title: impl Into<Cow<'a, str>>, document: Fresh<'a>) -> Self {
        Self::new(title, Kind::Fresh(document))
    }

    /// Configures the description of the result.
    pub fn description(mut self, description: impl Into<Cow<'a, str>>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Configures the caption of the document.
    pub fn caption(mut self, caption: impl Into<Text<'a>>) -> Self {
        let caption = caption.into();

        self.caption = Some(caption.text);
        self.parse_mode = caption.parse_mode;
        self
    }

    /// Configures the content shown after sending the message.
    pub fn input_message_content(
        mut self,
        content: impl Into<InputMessageContent<'a>>,
    ) -> Self {
        self.input_message_content = Some(content.into());
        self
    }
}

//! Types for representing [`inline_query::result::Kind::Document`].
//!
//! [`inline_query::result::Kind::Document`]: super::Kind::Document

use super::Thumb;
use crate::types::{
    file,
    parameters::{ParseMode, Text},
    InputMessageContent,
};
use is_macro::Is;
use serde::Serialize;

/// Represents possible MIME types for a fresh document.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Is)]
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
pub struct Fresh {
    #[serde(rename = "document_url")]
    url: String,
    mime_type: MimeType,
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    thumb: Option<Thumb>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[serde(untagged)]
#[must_use]
enum Kind {
    Cached {
        #[serde(rename = "document_file_id")]
        id: file::Id,
    },
    Fresh(Fresh),
}

/// Represents an [`InlineQueryResultDocument`]/[`InlineQueryResultCachedDocument`].
///
/// [`InlineQueryResultDocument`]: https://core.telegram.org/bots/api#inlinequeryresultdocument
/// [`InlineQueryResultCachedDocument`]: https://core.telegram.org/bots/api#inlinequeryresultcacheddocument
#[derive(Debug, PartialEq, Clone, Serialize)]
#[must_use]
pub struct Document {
    #[serde(flatten)]
    kind: Kind,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
}

impl Fresh {
    /// Constructs a `Fresh` document.
    pub fn new(url: impl Into<String>, mime_type: MimeType) -> Self {
        Self {
            url: url.into(),
            mime_type,
            thumb: None,
        }
    }

    /// Configures the thumb of the document.
    #[allow(clippy::missing_const_for_fn)]
    pub fn thumb(mut self, thumb: Thumb) -> Self {
        self.thumb = Some(thumb);
        self
    }
}

impl Document {
    fn new(title: impl Into<String>, kind: Kind) -> Self {
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
    pub fn with_cached(title: impl Into<String>, id: file::Id) -> Self {
        Self::new(title, Kind::Cached { id })
    }

    /// Constructs a fresh `Document` result.
    pub fn with_fresh(title: impl Into<String>, document: Fresh) -> Self {
        Self::new(title, Kind::Fresh(document))
    }

    /// Configures the description of the result.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Configures the caption of the document.
    pub fn caption(mut self, caption: impl Into<Text>) -> Self {
        let caption = caption.into();

        self.caption = Some(caption.text);
        self.parse_mode = caption.parse_mode;
        self
    }

    /// Configures the content shown after sending the message.
    pub fn input_message_content(
        mut self,
        content: impl Into<InputMessageContent>,
    ) -> Self {
        self.input_message_content = Some(content.into());
        self
    }
}

//! Types for representing [`InlineQueryResult::Document`][docs].
//!
//! [docs]: ../enum.InlineQueryResult.html#variant.Document

use super::Thumb;
use crate::types::{
    parameters::{ParseMode, Text},
    value::{self, FileId, Ref},
    InputMessageContent,
};
use serde::Serialize;

/// Represents possible MIME types for a fresh document.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
// todo: #[non_exhaustive]
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
pub struct Fresh<'a> {
    #[serde(rename = "document_url")]
    url: value::String<'a>,
    mime_type: MimeType,
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    thumb: Option<Ref<'a, Thumb<'a>>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[serde(untagged)]
enum Kind<'a> {
    Cached {
        #[serde(rename = "document_file_id")]
        id: FileId<'a>,
    },
    Fresh(Ref<'a, Fresh<'a>>),
}

/// Represents an [`InlineQueryResultDocument`]/[`InlineQueryResultCachedDocument`].
///
/// [`InlineQueryResultDocument`]: https://core.telegram.org/bots/api#inlinequeryresultdocument
/// [`InlineQueryResultCachedDocument`]: https://core.telegram.org/bots/api#inlinequeryresultcacheddocument
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Document<'a> {
    #[serde(flatten)]
    kind: Kind<'a>,
    title: value::String<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<value::String<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<value::String<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<Ref<'a, InputMessageContent<'a>>>,
}

impl MimeType {
    /// Checks if the MIME type is `application/pdf`.
    pub fn is_pdf(self) -> bool {
        self == MimeType::ApplicationPdf
    }

    /// Checks if the MIME type is `application/zip`.
    pub fn is_zip(self) -> bool {
        self == MimeType::ApplicationZip
    }
}

impl<'a> Fresh<'a> {
    /// Constructs a `Fresh` document.
    pub fn new(url: impl Into<value::String<'a>>, mime_type: MimeType) -> Self {
        Self {
            url: url.into(),
            mime_type,
            thumb: None,
        }
    }

    /// Configures the thumb of the document.
    pub fn thumb(mut self, thumb: impl Into<Ref<'a, Thumb<'a>>>) -> Self {
        self.thumb = Some(thumb.into());
        self
    }
}

impl<'a> Document<'a> {
    const fn new(title: value::String<'a>, kind: Kind<'a>) -> Self {
        Self {
            kind,
            title,
            description: None,
            caption: None,
            parse_mode: None,
            input_message_content: None,
        }
    }

    /// Constructs a cached `Document` result.
    pub fn cached(
        title: impl Into<value::String<'a>>,
        id: impl Into<FileId<'a>>,
    ) -> Self {
        Self::new(
            title.into(),
            Kind::Cached {
                id: id.into(),
            },
        )
    }

    /// Constructs a fresh `Document` result.
    pub fn fresh(
        title: impl Into<value::String<'a>>,
        document: impl Into<Ref<'a, Fresh<'a>>>,
    ) -> Self {
        Self::new(title.into(), Kind::Fresh(document.into()))
    }

    /// Configures the description of the result.
    pub fn description(
        mut self,
        description: impl Into<value::String<'a>>,
    ) -> Self {
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
        content: impl Into<Ref<'a, InputMessageContent<'a>>>,
    ) -> Self {
        self.input_message_content = Some(content.into());
        self
    }
}

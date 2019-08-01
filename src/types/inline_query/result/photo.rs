//! Types for representing [`InlineQueryResult::Photo`][docs].
//!
//! [docs]: ../enum.InlineQueryResult.html#variant.Photo

use crate::types::{
    parameters::{ParseMode, Text},
    value::{self, FileId, Ref},
    InputMessageContent,
};
use serde::Serialize;

/// Represents a non-cached photo.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
pub struct Fresh<'a> {
    thumb_url: value::String<'a>,
    #[serde(rename = "photo_url")]
    url: value::String<'a>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "photo_width")]
    width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "photo_height")]
    height: Option<usize>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
enum Kind<'a> {
    Cached {
        #[serde(rename = "photo_file_id")]
        id: FileId<'a>,
    },
    Fresh(Ref<'a, Fresh<'a>>),
}

/// Represents an [`InlineQueryResultPhoto`]/[`InlineQueryResultCachedPhoto`].
///
/// [`InlineQueryResultPhoto`]: https://core.telegram.org/bots/api#inlinequeryresultphoto
/// [`InlineQueryResultCachedPhoto`]: https://core.telegram.org/bots/api#inlinequeryresultcachedphoto
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Photo<'a> {
    kind: Kind<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<value::String<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<value::String<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<value::String<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<Ref<'a, InputMessageContent<'a>>>,
}

impl<'a> Fresh<'a> {
    /// Constructs a `Fresh` photo.
    pub fn new(
        thumb_url: impl Into<value::String<'a>>,
        url: impl Into<value::String<'a>>,
    ) -> Self {
        Self {
            thumb_url: thumb_url.into(),
            url: url.into(),
            width: None,
            height: None,
        }
    }

    /// Configures the width of the photo.
    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Configures the height of the photo.
    pub fn height(mut self, height: usize) -> Self {
        self.height = Some(height);
        self
    }
}

impl<'a> Photo<'a> {
    const fn new(kind: Kind<'a>) -> Self {
        Self {
            kind,
            title: None,
            description: None,
            caption: None,
            parse_mode: None,
            input_message_content: None,
        }
    }

    /// Constructs a cached `Photo` result.
    pub fn cached(id: impl Into<FileId<'a>>) -> Self {
        Self::new(Kind::Cached {
            id: id.into(),
        })
    }

    /// Constructs a fresh `Photo` result.
    pub fn fresh(photo: impl Into<Ref<'a, Fresh<'a>>>) -> Self {
        Self::new(Kind::Fresh(photo.into()))
    }

    /// Configures the title of the photo.
    pub fn title(mut self, title: impl Into<value::String<'a>>) -> Self {
        self.title = Some(title.into());
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

    /// Configures the caption of the photo.
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

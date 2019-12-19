//! Types for representing [`InlineQueryResult::Video`][docs].
//!
//! [docs]: ../enum.InlineQueryResult.html#variant.Video

use crate::types::{
    parameters::{ParseMode, Text},
    InputMessageContent,
};
use serde::Serialize;

/// Represents possible MIME types.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[non_exhaustive]
pub enum MimeType {
    /// The `text/html` MIME type.
    #[serde(rename = "text/html")]
    TextHtml,
    /// The `video/mp4` MIME type.
    #[serde(rename = "video/mp4")]
    VideoMp4,
}

/// Represents a non-cached video.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
pub struct Fresh<'a> {
    #[serde(rename = "video_url")]
    url: &'a str,
    mime_type: MimeType,
    thumb_url: &'a str,
    #[serde(skip_serializing_if = "Option::is_none", rename = "video_width")]
    width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "video_height")]
    height: Option<usize>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "video_duration"
    )]
    duration: Option<usize>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
enum Kind<'a> {
    Cached {
        #[serde(rename = "video_file_id")]
        id: &'a str,
    },
    Fresh(Fresh<'a>),
}

/// Represents an [`InlineQueryResultVideo`]/[`InlineQueryResultCachedVideo`].
///
/// [`InlineQueryResultVideo`]: https://core.telegram.org/bots/api#inlinequeryresultvideo
/// [`InlineQueryResultCachedVideo`]: https://core.telegram.org/bots/api#inlinequeryresultcachedvideo
#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub struct Video<'a> {
    kind: Kind<'a>,
    title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent<'a>>,
}

impl MimeType {
    /// Checks if the MIME type is `text/html`.
    pub fn is_html(self) -> bool {
        self == Self::TextHtml
    }

    /// Checks if the MIME type is `video/mp4`.
    pub fn is_zip(self) -> bool {
        self == Self::VideoMp4
    }
}

impl<'a> Fresh<'a> {
    /// Constructs a `Fresh` video.
    pub const fn new(
        url: &'a str,
        mime_type: MimeType,
        thumb_url: &'a str,
    ) -> Self {
        Self {
            url,
            mime_type,
            thumb_url,
            width: None,
            height: None,
            duration: None,
        }
    }

    /// Configures the width of the video.
    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Configures the height of the video.
    pub fn height(mut self, height: usize) -> Self {
        self.height = Some(height);
        self
    }

    /// Configures the duration of the video.
    pub fn duration(mut self, duration: usize) -> Self {
        self.duration = Some(duration);
        self
    }
}

impl<'a> Video<'a> {
    const fn new(title: &'a str, kind: Kind<'a>) -> Self {
        Self {
            kind,
            title,
            description: None,
            caption: None,
            parse_mode: None,
            input_message_content: None,
        }
    }

    /// Constructs a cached `Video` result.
    pub fn cached(title: &'a str, id: &'a str) -> Self {
        Self::new(title, Kind::Cached { id })
    }

    /// Constructs a fresh `Video` result.
    pub fn fresh(title: &'a str, video: Fresh<'a>) -> Self {
        Self::new(title, Kind::Fresh(video))
    }

    /// Configures the description of the result.
    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    /// Configures the caption of the video.
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

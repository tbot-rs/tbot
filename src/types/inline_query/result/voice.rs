//! Types for representing [`InlineQueryResult::Voice`][docs].
//!
//! [docs]: ../enum.InlineQueryResult.html#variant.Voice

use crate::types::{
    file,
    parameters::{ParseMode, Text},
    InputMessageContent, InteriorBorrow,
};
use serde::Serialize;
use std::borrow::Cow;

/// Represents a non-cached voice.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[must_use]
pub struct Fresh<'a> {
    #[serde(rename = "voice_url")]
    url: Cow<'a, str>,
    #[serde(
        rename = "voice_duration",
        skip_serializing_if = "Option::is_none"
    )]
    duration: Option<usize>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[serde(untagged)]
#[must_use]
enum Kind<'a> {
    Cached {
        #[serde(rename = "voice_file_id")]
        id: file::Id<'a>,
    },
    Fresh(Fresh<'a>),
}

/// Represents an [`InlineQueryResultVoice`]/[`InlineQueryResultCachedVoice`].
///
/// [`InlineQueryResultVoice`]: https://core.telegram.org/bots/api#inlinequeryresultvoice
/// [`InlineQueryResultCachedVoice`]: https://core.telegram.org/bots/api#inlinequeryresultcachedvoice
#[derive(Debug, PartialEq, Clone, Serialize)]
#[must_use]
pub struct Voice<'a> {
    #[serde(flatten)]
    kind: Kind<'a>,
    title: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent<'a>>,
}

impl<'a> Fresh<'a> {
    /// Constructs a `Fresh` voice.
    pub fn new(url: impl Into<Cow<'a, str>>) -> Self {
        Self {
            url: url.into(),
            duration: None,
        }
    }

    /// Configures the duration of the voice.
    pub const fn duration(mut self, duration: usize) -> Self {
        self.duration = Some(duration);
        self
    }
}

impl<'a> Voice<'a> {
    fn new(title: impl Into<Cow<'a, str>>, kind: Kind<'a>) -> Self {
        Self {
            kind,
            title: title.into(),
            caption: None,
            parse_mode: None,
            input_message_content: None,
        }
    }

    /// Constructs a cached `Voice` result.
    pub fn with_cached(
        title: impl Into<Cow<'a, str>>,
        id: file::Id<'a>,
    ) -> Self {
        Self::new(title, Kind::Cached { id })
    }

    /// Constructs a fresh `Voice` result.
    pub fn with_fresh(
        title: impl Into<Cow<'a, str>>,
        voice: Fresh<'a>,
    ) -> Self {
        Self::new(title, Kind::Fresh(voice))
    }

    /// Configures the caption of the voice.
    pub fn caption(mut self, caption: impl Into<Text>) -> Self {
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

impl<'a> InteriorBorrow<'a> for Fresh<'a> {
    fn borrow_inside(&'a self) -> Self {
        Self {
            url: self.url.borrow_inside(),
            ..*self
        }
    }
}

impl<'a> InteriorBorrow<'a> for Kind<'a> {
    fn borrow_inside(&'a self) -> Self {
        match self {
            Self::Cached { id } => Self::Cached {
                id: id.borrow_inside(),
            },
            Self::Fresh(fresh) => Self::Fresh(fresh.borrow_inside()),
        }
    }
}

//! Types for representing [`InlineQueryResult::Voice`][docs].
//!
//! [docs]: ../enum.InlineQueryResult.html#variant.Voice

use crate::types::{
    parameters::{ParseMode, Text},
    InputMessageContent,
};
use serde::Serialize;

/// Represents a non-cached voice.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[must_use]
pub struct Fresh<'a> {
    #[serde(rename = "voice_url")]
    url: &'a str,
    #[serde(
        rename = "voice_duration",
        skip_serializing_if = "Option::is_none"
    )]
    duration: Option<usize>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[serde(untagged)]
#[must_use]
enum Kind<'a> {
    Cached {
        #[serde(rename = "voice_file_id")]
        id: &'a str,
    },
    Fresh(Fresh<'a>),
}

/// Represents an [`InlineQueryResultVoice`]/[`InlineQueryResultCachedVoice`].
///
/// [`InlineQueryResultVoice`]: https://core.telegram.org/bots/api#inlinequeryresultvoice
/// [`InlineQueryResultCachedVoice`]: https://core.telegram.org/bots/api#inlinequeryresultcachedvoice
#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
#[must_use]
pub struct Voice<'a> {
    #[serde(flatten)]
    kind: Kind<'a>,
    title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent<'a>>,
}

impl<'a> Fresh<'a> {
    /// Constructs a `Fresh` voice.
    pub const fn new(url: &'a str) -> Self {
        Self {
            url,
            duration: None,
        }
    }

    /// Configures the duration of the voice.
    pub fn duration(mut self, duration: usize) -> Self {
        self.duration = Some(duration);
        self
    }
}

impl<'a> Voice<'a> {
    const fn new(title: &'a str, kind: Kind<'a>) -> Self {
        Self {
            kind,
            title,
            caption: None,
            parse_mode: None,
            input_message_content: None,
        }
    }

    /// Constructs a cached `Voice` result.
    pub fn cached(title: &'a str, id: &'a str) -> Self {
        Self::new(title, Kind::Cached { id })
    }

    /// Constructs a fresh `Voice` result.
    pub fn fresh(title: &'a str, voice: Fresh<'a>) -> Self {
        Self::new(title, Kind::Fresh(voice))
    }

    /// Configures the caption of the voice.
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

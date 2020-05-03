//! Types for representing [`InlineQueryResult::Audio`][docs].
//!
//! [docs]: ../enum.InlineQueryResult.html#variant.Audio

use crate::types::{
    parameters::{ParseMode, Text},
    InputMessageContent,
};
use serde::Serialize;
use std::borrow::Cow;

/// Represents a non-cached audio.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[must_use]
pub struct Fresh<'a> {
    #[serde(rename = "audio_url")]
    url: Cow<'a, str>,
    title: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performer: Option<Cow<'a, str>>,
    #[serde(
        rename = "audio_duration",
        skip_serializing_if = "Option::is_none"
    )]
    duration: Option<usize>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[serde(untagged)]
#[must_use]
enum Kind<'a> {
    Cached {
        #[serde(rename = "audio_file_id")]
        id: Cow<'a, str>,
    },
    Fresh(Fresh<'a>),
}

/// Represents an [`InlineQueryResultAudio`]/[`InlineQueryResultCachedAudio`].
///
/// [`InlineQueryResultAudio`]: https://core.telegram.org/bots/api#inlinequeryresultaudio
/// [`InlineQueryResultCachedAudio`]: https://core.telegram.org/bots/api#inlinequeryresultcachedaudio
#[derive(Debug, PartialEq, Clone, Serialize)]
#[must_use]
pub struct Audio<'a> {
    #[serde(flatten)]
    kind: Kind<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent<'a>>,
}

impl<'a> Fresh<'a> {
    /// Constructs a `Fresh` audio.
    pub fn new(
        url: impl Into<Cow<'a, str>>,
        title: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            url: url.into(),
            title: title.into(),
            performer: None,
            duration: None,
        }
    }

    /// Configures the performer of the audio.
    pub fn performer(mut self, performer: impl Into<Cow<'a, str>>) -> Self {
        self.performer = Some(performer.into());
        self
    }

    /// Configures the duration of the audio.
    pub fn duration(mut self, duration: usize) -> Self {
        self.duration = Some(duration);
        self
    }
}

impl<'a> Audio<'a> {
    const fn new(kind: Kind<'a>) -> Self {
        Self {
            kind,
            caption: None,
            parse_mode: None,
            input_message_content: None,
        }
    }

    /// Constructs a cached `Audio` result.
    pub fn cached(id: impl Into<Cow<'a, str>>) -> Self {
        Self::new(Kind::Cached { id: id.into() })
    }

    /// Constructs a fresh `Audio` result.
    pub fn fresh(audio: Fresh<'a>) -> Self {
        Self::new(Kind::Fresh(audio))
    }

    /// Configures the caption of the audio.
    pub fn caption(mut self, caption: impl Into<Text<'a>>) -> Self {
        let caption = caption.into();

        self.caption = Some(caption.text.into());
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

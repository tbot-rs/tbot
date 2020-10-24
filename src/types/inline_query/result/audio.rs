//! Types for representing [`InlineQueryResult::Audio`][docs].
//!
//! [docs]: ../enum.InlineQueryResult.html#variant.Audio

use crate::types::{
    file,
    parameters::{ParseMode, Text},
    InputMessageContent,
};
use serde::Serialize;

/// Represents a non-cached audio.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[must_use]
pub struct Fresh<'a> {
    #[serde(rename = "audio_url")]
    url: &'a str,
    title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    performer: Option<&'a str>,
    #[serde(
        rename = "audio_duration",
        skip_serializing_if = "Option::is_none"
    )]
    duration: Option<usize>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[serde(untagged)]
#[must_use]
enum Kind<'a> {
    Cached {
        #[serde(rename = "audio_file_id")]
        id: &'a str,
    },
    Fresh(Fresh<'a>),
}

/// Represents an [`InlineQueryResultAudio`]/[`InlineQueryResultCachedAudio`].
///
/// [`InlineQueryResultAudio`]: https://core.telegram.org/bots/api#inlinequeryresultaudio
/// [`InlineQueryResultCachedAudio`]: https://core.telegram.org/bots/api#inlinequeryresultcachedaudio
#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
#[must_use]
pub struct Audio<'a> {
    #[serde(flatten)]
    kind: Kind<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent<'a>>,
}

impl<'a> Fresh<'a> {
    /// Constructs a `Fresh` audio.
    pub const fn new(url: &'a str, title: &'a str) -> Self {
        Self {
            url,
            title,
            performer: None,
            duration: None,
        }
    }

    /// Configures the performer of the audio.
    pub fn performer(mut self, performer: &'a str) -> Self {
        self.performer = Some(performer);
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
    pub fn with_cached(id: file::id::Ref<'a>) -> Self {
        Self::new(Kind::Cached { id: id.0 })
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "use `with_cached` which takes a `file::id::Ref<'a>`"
    )]
    pub fn cached(id: &'a str) -> Self {
        Self::with_cached(file::id::Ref(id))
    }

    /// Constructs a fresh `Audio` result.
    pub fn with_fresh(audio: Fresh<'a>) -> Self {
        Self::new(Kind::Fresh(audio))
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "this method is renamed to `with_fresh`"
    )]
    pub fn fresh(audio: Fresh<'a>) -> Self {
        Self::with_fresh(audio)
    }

    /// Configures the caption of the audio.
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

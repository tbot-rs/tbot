//! Types for representing [`InlineQueryResult::Audio`][docs].
//!
//! [docs]: ../enum.InlineQueryResult.html#variant.Audio

use crate::types::{
    parameters::{ParseMode, Text},
    value::{self, FileId, Ref},
    InputMessageContent,
};
use serde::Serialize;

/// Represents a non-cached audio.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
pub struct Fresh<'a> {
    #[serde(rename = "audio_url")]
    url: value::String<'a>,
    title: value::String<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performer: Option<value::String<'a>>,
    #[serde(
        rename = "audio_duration",
        skip_serializing_if = "Option::is_none"
    )]
    duration: Option<usize>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[serde(untagged)]
enum Kind<'a> {
    Cached {
        #[serde(rename = "audio_file_id")]
        id: FileId<'a>,
    },
    Fresh(Ref<'a, Fresh<'a>>),
}

/// Represents an [`InlineQueryResultAudio`]/[`InlineQueryResultCachedAudio`].
///
/// [`InlineQueryResultAudio`]: https://core.telegram.org/bots/api#inlinequeryresultaudio
/// [`InlineQueryResultCachedAudio`]: https://core.telegram.org/bots/api#inlinequeryresultcachedaudio
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Audio<'a> {
    #[serde(flatten)]
    kind: Kind<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<value::String<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<Ref<'a, InputMessageContent<'a>>>,
}

impl<'a> Fresh<'a> {
    /// Constructs a `Fresh` audio.
    pub fn new(
        url: impl Into<value::String<'a>>,
        title: impl Into<value::String<'a>>,
    ) -> Self {
        Self {
            url: url.into(),
            title: title.into(),
            performer: None,
            duration: None,
        }
    }

    /// Configures the performer of the audio.
    pub fn performer(
        mut self,
        performer: impl Into<value::String<'a>>,
    ) -> Self {
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
    pub fn cached(id: impl Into<FileId<'a>>) -> Self {
        Self::new(Kind::Cached {
            id: id.into(),
        })
    }

    /// Constructs a fresh `Audio` result.
    pub fn fresh(audio: impl Into<Ref<'a, Fresh<'a>>>) -> Self {
        Self::new(Kind::Fresh(audio.into()))
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
        content: impl Into<Ref<'a, InputMessageContent<'a>>>,
    ) -> Self {
        self.input_message_content = Some(content.into());
        self
    }
}

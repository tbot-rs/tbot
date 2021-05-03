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
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[must_use]
pub struct Fresh {
    #[serde(rename = "audio_url")]
    url: String,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    performer: Option<String>,
    #[serde(
        rename = "audio_duration",
        skip_serializing_if = "Option::is_none"
    )]
    duration: Option<usize>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[serde(untagged)]
#[must_use]
enum Kind {
    Cached {
        #[serde(rename = "audio_file_id")]
        id: file::Id,
    },
    Fresh(Fresh),
}

/// Represents an [`InlineQueryResultAudio`]/[`InlineQueryResultCachedAudio`].
///
/// [`InlineQueryResultAudio`]: https://core.telegram.org/bots/api#inlinequeryresultaudio
/// [`InlineQueryResultCachedAudio`]: https://core.telegram.org/bots/api#inlinequeryresultcachedaudio
#[derive(Debug, PartialEq, Clone, Serialize)]
#[must_use]
pub struct Audio {
    #[serde(flatten)]
    kind: Kind,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
}

impl Fresh {
    /// Constructs a `Fresh` audio.
    pub fn new(url: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            title: title.into(),
            performer: None,
            duration: None,
        }
    }

    /// Configures the performer of the audio.
    pub fn performer(mut self, performer: impl Into<String>) -> Self {
        self.performer = Some(performer.into());
        self
    }

    /// Configures the duration of the audio.
    pub const fn duration(mut self, duration: usize) -> Self {
        self.duration = Some(duration);
        self
    }
}

impl Audio {
    const fn new(kind: Kind) -> Self {
        Self {
            kind,
            caption: None,
            parse_mode: None,
            input_message_content: None,
        }
    }

    /// Constructs a cached `Audio` result.
    pub const fn with_cached(id: file::Id) -> Self {
        Self::new(Kind::Cached { id })
    }

    /// Constructs a fresh `Audio` result.
    pub const fn with_fresh(audio: Fresh) -> Self {
        Self::new(Kind::Fresh(audio))
    }

    /// Configures the caption of the audio.
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

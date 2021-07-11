//! Types for representing [`inline_query::result::Kind::Voice`].
//!
//! [`inline_query::result::Kind::Voice`]: super::Kind::Voice

use crate::types::{
    file,
    parameters::{ParseMode, Text},
    InputMessageContent,
};
use serde::Serialize;

/// Represents a non-cached voice.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[must_use]
pub struct Fresh {
    #[serde(rename = "voice_url")]
    url: String,
    #[serde(
        rename = "voice_duration",
        skip_serializing_if = "Option::is_none"
    )]
    duration: Option<usize>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[serde(untagged)]
#[must_use]
enum Kind {
    Cached {
        #[serde(rename = "voice_file_id")]
        id: file::Id,
    },
    Fresh(Fresh),
}

/// Represents an [`InlineQueryResultVoice`]/[`InlineQueryResultCachedVoice`].
///
/// [`InlineQueryResultVoice`]: https://core.telegram.org/bots/api#inlinequeryresultvoice
/// [`InlineQueryResultCachedVoice`]: https://core.telegram.org/bots/api#inlinequeryresultcachedvoice
#[derive(Debug, PartialEq, Clone, Serialize)]
#[must_use]
pub struct Voice {
    #[serde(flatten)]
    kind: Kind,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
}

impl Fresh {
    /// Constructs a `Fresh` voice.
    pub fn new(url: impl Into<String>) -> Self {
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

impl Voice {
    fn new(title: impl Into<String>, kind: Kind) -> Self {
        Self {
            kind,
            title: title.into(),
            caption: None,
            parse_mode: None,
            input_message_content: None,
        }
    }

    /// Constructs a cached `Voice` result.
    pub fn with_cached(title: impl Into<String>, id: file::Id) -> Self {
        Self::new(title, Kind::Cached { id })
    }

    /// Constructs a fresh `Voice` result.
    pub fn with_fresh(title: impl Into<String>, voice: Fresh) -> Self {
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
        content: impl Into<InputMessageContent>,
    ) -> Self {
        self.input_message_content = Some(content.into());
        self
    }
}

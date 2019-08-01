//! Types for representing [`InlineQueryResult::Voice`][docs].
//!
//! [docs]: ../enum.InlineQueryResult.html#variant.Voice

use crate::types::{
    parameters::{ParseMode, Text},
    value::{self, FileId, Ref},
    InputMessageContent,
};
use serde::Serialize;

/// Represents a non-cached voice.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
pub struct Fresh<'a> {
    #[serde(rename = "voice_url")]
    url: value::String<'a>,
    #[serde(
        rename = "voice_duration",
        skip_serializing_if = "Option::is_none"
    )]
    duration: Option<usize>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[serde(untagged)]
enum Kind<'a> {
    Cached {
        #[serde(rename = "voice_file_id")]
        id: FileId<'a>,
    },
    Fresh(Ref<'a, Fresh<'a>>),
}

/// Represents an [`InlineQueryResultVoice`]/[`InlineQueryResultCachedVoice`].
///
/// [`InlineQueryResultVoice`]: https://core.telegram.org/bots/api#inlinequeryresultvoice
/// [`InlineQueryResultCachedVoice`]: https://core.telegram.org/bots/api#inlinequeryresultcachedvoice
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Voice<'a> {
    #[serde(flatten)]
    kind: Kind<'a>,
    title: value::String<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<value::String<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<Ref<'a, InputMessageContent<'a>>>,
}

impl<'a> Fresh<'a> {
    /// Constructs a `Fresh` voice.
    pub fn new(url: impl Into<value::String<'a>>) -> Self {
        Self {
            url: url.into(),
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
    const fn new(title: value::String<'a>, kind: Kind<'a>) -> Self {
        Self {
            kind,
            title,
            caption: None,
            parse_mode: None,
            input_message_content: None,
        }
    }

    /// Constructs a cached `Voice` result.
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

    /// Constructs a fresh `Voice` result.
    pub fn fresh(
        title: impl Into<value::String<'a>>,
        voice: impl Into<Ref<'a, Fresh<'a>>>,
    ) -> Self {
        Self::new(title.into(), Kind::Fresh(voice.into()))
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
        content: impl Into<Ref<'a, InputMessageContent<'a>>>,
    ) -> Self {
        self.input_message_content = Some(content.into());
        self
    }
}

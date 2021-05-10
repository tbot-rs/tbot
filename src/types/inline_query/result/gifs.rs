macro_rules! doc {
    (
        $doc:expr,
        $($item:tt)+
    ) => {
        #[doc = $doc]
        $($item)+
    }
}

/// Represents a `Fresh` GIF's thumb.
pub struct GifThumb {
    url: String,
    mime: &'static str,
}

impl GifThumb {
    /// Constructs a JPEG thumb.
    #[must_use]
    pub fn jpeg(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            mime: "image/jpeg",
        }
    }

    /// Constructs a GIF thumb.
    #[must_use]
    pub fn gif(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            mime: "image/gif",
        }
    }

    /// Constructs a MP4 thumb.
    #[must_use]
    pub fn mp4(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            mime: "video/mp4",
        }
    }
}

impl<T: Into<String>> From<T> for GifThumb {
    fn from(url: T) -> Self {
        Self::jpeg(url)
    }
}

#[rustfmt::skip] // it messes up multiline attributes
macro_rules! gif_base {
    (
      url: $url:literal,
      width: $width:literal,
      height: $height:literal,
      duration: $duration:literal,
      file_id: $file_id:literal,
      struct: $struct:ident,
      doc_link_part: $doc_link_part:literal,
    ) => {
        use super::GifThumb;
        use crate::types::{
            InputMessageContent, file,
            parameters::{ParseMode, Text},
        };
        use serde::Serialize;

        /// Represents a non-cached GIF.
        #[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
        #[must_use]
        pub struct Fresh {
            thumb_url: String,
            thumb_mime_type: &'static str,
            #[serde(rename = $url)]
            url: String,
            #[serde(
                skip_serializing_if = "Option::is_none",
                rename = $width
            )]
            width: Option<usize>,
            #[serde(
                skip_serializing_if = "Option::is_none",
                rename = $height
            )]
            height: Option<usize>,
            #[serde(
                skip_serializing_if = "Option::is_none",
                rename = $duration
            )]
            duration: Option<usize>,
        }

        #[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
        #[serde(untagged)]
        #[must_use]
        enum Kind {
            Cached {
                #[serde(rename = $file_id)]
                id: file::Id,
            },
            Fresh(Fresh),
        }

        doc! {
            concat!(
                "Represents an [`InlineQueryResult", stringify!($struct), "`]/",
                "[`InlineQueryResultCached", stringify!($struct), "`].\n\n",

                "[`InlineQueryResult", stringify!($struct), "`]:",
                "https://core.telegram.org/bots/api#inlinequeryresult",
                $doc_link_part, "\n",
                "[`InlineQueryResultCached", stringify!($struct), "`]:",
                "https://core.telegram.org/bots/api#inlinequeryresultcached",
                $doc_link_part,
            ),
            #[derive(Debug, PartialEq, Clone, Serialize)]
            #[must_use]
            pub struct $struct {
                #[serde(flatten)]
                kind: Kind,
                #[serde(skip_serializing_if = "Option::is_none")]
                title: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                caption: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                parse_mode: Option<ParseMode>,
                #[serde(skip_serializing_if = "Option::is_none")]
                input_message_content: Option<InputMessageContent>,
            }
        }

        impl Fresh {
            /// Constructs a `Fresh` GIF.
            pub fn new(thumb: impl Into<GifThumb>, url: impl Into<String>) -> Self {
                let thumb = thumb.into();

                Self {
                    thumb_url: thumb.url,
                    thumb_mime_type: thumb.mime,
                    url: url.into(),
                    width: None,
                    height: None,
                    duration: None,
                }
            }

            /// Configures the width of the GIF.
            pub const fn width(mut self, width: usize) -> Self {
                self.width = Some(width);
                self
            }

            /// Configures the height of the GIF.
            pub const fn height(mut self, height: usize) -> Self {
                self.height = Some(height);
                self
            }

            /// Configures the duration of the GIF.
            pub const fn duration(mut self, duration: usize) -> Self {
                self.duration = Some(duration);
                self
            }
        }

        impl $struct {
            const fn new(kind: Kind) -> Self {
                Self {
                    kind,
                    title: None,
                    caption: None,
                    parse_mode: None,
                    input_message_content: None,
                }
            }

            doc! {
                concat!(
                    "Constructs a cached `", stringify!($struct), "` result.",
                ),
                pub const fn with_cached(id: file::Id) -> Self {
                    Self::new(Kind::Cached { id })
                }
            }

            doc! {
                concat!(
                    "Constructs a fresh `", stringify!($struct), "` result.",
                ),
                pub const fn with_fresh(gif: Fresh) -> Self {
                    Self::new(Kind::Fresh(gif))
                }
            }

            /// Configures the title of the GIF.
            pub fn title(mut self, title: impl Into<String>) -> Self {
                self.title = Some(title.into());
                self
            }

            /// Configures the caption of the GIF.
            pub fn caption(mut self, caption: impl Into<Text>) -> Self {
                let caption = caption.into();

                self.caption = Some(caption.text.into());
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
    };
}

pub mod gif {
    //! Types for representing [`inline_query::result::Kind::Gif`].
    //!
    //! [`inline_query::result::Kind::Gif`]: super::super::Kind::Gif

    gif_base! {
        url: "gif_url",
        width: "gif_width",
        height: "gif_height",
        duration: "gif_duration",
        file_id: "gif_file_id",
        struct: Gif,
        doc_link_part: "gif",
    }
}

pub mod mpeg4 {
    //! Types for representing [`inline_query::result::Kind::Mpeg4Gif`].
    //!
    //! [`inline_query::result::Kind::Mpeg4Gif`]: super::super::Kind::Mpeg4Gif

    gif_base! {
        url: "mpeg4_url",
        width: "mpeg4_width",
        height: "mpeg4_height",
        duration: "mpeg4_duration",
        file_id: "mpeg4_file_id",
        struct: Mpeg4Gif,
        doc_link_part: "mpeg4gif",
    }
}

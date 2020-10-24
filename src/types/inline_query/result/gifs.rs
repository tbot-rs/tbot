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
pub struct GifThumb<'a> {
    url: &'a str,
    mime: &'a str,
}

impl<'a> GifThumb<'a> {
    /// Constructs a JPEG thumb.
    #[must_use]
    pub const fn jpeg(url: &'a str) -> Self {
        Self {
            url,
            mime: "image/jpeg",
        }
    }

    /// Constructs a GIF thumb.
    #[must_use]
    pub const fn gif(url: &'a str) -> Self {
        Self {
            url,
            mime: "image/gif",
        }
    }

    /// Constructs a MP4 thumb.
    #[must_use]
    pub const fn mp4(url: &'a str) -> Self {
        Self {
            url,
            mime: "video/mp4",
        }
    }
}

impl<'a> From<&'a str> for GifThumb<'a> {
    fn from(url: &'a str) -> Self {
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
        use crate::types::{file, InputMessageContent, parameters::{ParseMode, Text}};
        use serde::Serialize;

        /// Represents a non-cached GIF.
        #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
        #[must_use]
        pub struct Fresh<'a> {
            thumb_url: &'a str,
            thumb_mime_type: &'a str,
            #[serde(rename = $url)]
            url: &'a str,
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

        #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
        #[serde(untagged)]
        #[must_use]
        enum Kind<'a> {
            Cached {
                #[serde(rename = $file_id)]
                id: &'a str,
            },
            Fresh(Fresh<'a>),
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
            #[derive(Debug, PartialEq, Clone, Copy, Serialize)]
            #[must_use]
            pub struct $struct<'a> {
                #[serde(flatten)]
                kind: Kind<'a>,
                #[serde(skip_serializing_if = "Option::is_none")]
                title: Option<&'a str>,
                #[serde(skip_serializing_if = "Option::is_none")]
                caption: Option<&'a str>,
                #[serde(skip_serializing_if = "Option::is_none")]
                parse_mode: Option<ParseMode>,
                #[serde(skip_serializing_if = "Option::is_none")]
                input_message_content: Option<InputMessageContent<'a>>,
            }
        }

        impl<'a> Fresh<'a> {
            /// Constructs a `Fresh` GIF.
            pub fn new(thumb: impl Into<GifThumb<'a>>, url: &'a str) -> Self {
                let thumb = thumb.into();

                Self {
                    thumb_url: thumb.url,
                    thumb_mime_type: thumb.mime,
                    url,
                    width: None,
                    height: None,
                    duration: None,
                }
            }

            /// Configures the width of the GIF.
            pub fn width(mut self, width: usize) -> Self {
                self.width = Some(width);
                self
            }

            /// Configures the height of the GIF.
            pub fn height(mut self, height: usize) -> Self {
                self.height = Some(height);
                self
            }

            /// Configures the duration of the GIF.
            pub fn duration(mut self, duration: usize) -> Self {
                self.duration = Some(duration);
                self
            }
        }

        impl<'a> $struct<'a> {
            const fn new(kind: Kind<'a>) -> Self {
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
                pub fn with_cached(id: file::id::Ref<'a>) -> Self {
                    Self::new(Kind::Cached {
                        id: id.0
                    })
                }
            }

            #[doc(hidden)]
            #[deprecated(
                since = "0.6.6",
                note = "use `with_cached` which takes a `file::id::Ref<'a>`"
            )]
            pub fn cached(id: &'a str) -> Self {
                Self::with_cached(file::id::Ref(id))
            }

            doc! {
                concat!(
                    "Constructs a fresh `", stringify!($struct), "` result.",
                ),
                pub fn with_fresh(gif: Fresh<'a>) -> Self {
                    Self::new(Kind::Fresh(gif))
                }
            }

            #[doc(hidden)]
            #[deprecated(
                since = "0.6.6",
                note = "this method is renamed to `with_fresh`"
            )]
            pub fn fresh(gif: Fresh<'a>) -> Self {
                Self::with_fresh(gif)
            }

            /// Configures the title of the GIF.
            pub fn title(mut self, title: &'a str) -> Self {
                self.title = Some(title);
                self
            }

            /// Configures the caption of the GIF.
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
    };
}

pub mod gif {
    //! Types for representing [`InlineQueryResult::Gif`][docs].
    //!
    //! [docs]: ../enum.InlineQueryResult.html#variant.Gif

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
    //! Types for representing [`InlineQueryResult::Mpeg4Gif`][docs].
    //!
    //! [docs]: ../enum.InlineQueryResult.html#variant.Mpeg4Gif

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

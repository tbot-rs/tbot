macro_rules! doc {
    (
        $doc:expr,
        $($item:tt)+
    ) => {
        #[doc = $doc]
        $($item)+
    }
}

#[rustfmt::skip] // it messes up multiline attributes
macro_rules! gif_base {
    (
      prefix: $prefix:literal,
      struct: $struct:ident,
      doc_link_part: $doc_link_part:literal,
    ) => {
        use crate::types::{InputMessageContent, ParseMode};
        use serde::Serialize;

        /// Represents a non-cached GIF.
        #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
        pub struct Fresh<'a> {
            thumb_url: &'a str,
            #[serde(rename = concat!($prefix, "_url"))]
            url: &'a str,
            #[serde(
                skip_serializing_if = "Option::is_none",
                rename = concat!($prefix, "_width")
            )]
            width: Option<usize>,
            #[serde(
                skip_serializing_if = "Option::is_none",
                rename = concat!($prefix, "_height")
            )]
            height: Option<usize>,
            #[serde(
                skip_serializing_if = "Option::is_none",
                rename = concat!($prefix, "_duration")
            )]
            duration: Option<usize>,
        }

        #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
        #[serde(untagged)]
        enum Kind<'a> {
            Cached {
                #[serde(rename = concat!($prefix, "_file_id"))]
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
            pub const fn new(thumb_url: &'a str, url: &'a str) -> Self {
                Self {
                    thumb_url,
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
                pub fn cached(id: &'a str) -> Self {
                    Self::new(Kind::Cached {
                        id,
                    })
                }
            }

            doc! {
                concat!(
                    "Constructs a fresh `", stringify!($struct), "` result.",
                ),
                pub fn fresh(gif: Fresh<'a>) -> Self {
                    Self::new(Kind::Fresh(gif))
                }
            }

            /// Configures the title of the GIF.
            pub fn title(mut self, title: &'a str) -> Self {
                self.title = Some(title);
                self
            }

            /// Configures the caption of the GIF.
            pub fn caption(mut self, caption: &'a str) -> Self {
                self.caption = Some(caption);
                self
            }

            /// Configures the parse mode of the GIF's caption.
            pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
                self.parse_mode = Some(parse_mode);
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
        prefix: "gif",
        struct: Gif,
        doc_link_part: "gif",
    }
}

pub mod mpeg4 {
    //! Types for representing [`InlineQueryResult::Mpeg4Gif`][docs].
    //!
    //! [docs]: ../enum.InlineQueryResult.html#variant.Mpeg4Gif

    gif_base! {
        prefix: "mpeg4",
        struct: Mpeg4Gif,
        doc_link_part: "mpeg4gif",
    }
}

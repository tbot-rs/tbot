//! Utilities for working with markup.
//!
//! The purpose of this module is to make it easy and painless to work with
//! different markups correctly. This module contains abstract formatters like
//! [`bold`], [`italic`], [`link`], etc. You can pass strings,
//! slices/arrays/vectors of strings to them:
//!
//! ```
//! use tbot::markup::bold;
//! let bold = bold(vec!["*This will <b>e in </b>old", ", and this too!"]);
//! ```
//!
//! [`bold`]: ./fn.bold.html
//! [`italic`]: ./fn.italic.html
//! [`link`]: ./fn.link.html
//!
//! However, you can't use their return values directly — indeed, how do they
//! know if they need to formt their inputs as HTML, Markdown or MarkdownV2?
//! That's where markup formatters [`html`], [`markdown`] and [`markdown_v2`]
//! come into play. They take the return values from the abstract utilities and
//! returns values that can finally be turned into strings:
//!
//! ```
//! # use tbot::markup::bold;
//! # let bold = bold(vec!["*This will <b>e in </b>old", ", and this too!"]);
//! use tbot::markup::markdown_v2;
//! let message = format!("{}", markdown_v2(bold));
//! assert_eq!(message, r#"*\*This will <b\>e in </b\>old, and this too!*"#);
//! ```
//!
//! As you can see, you can fearlessly pass any strings to formatters and they'll
//! be automatically properly escaped. Magic!
//!
//! [`html`]: ./html/fn.html.html
//! [`markdown`]: ./markdown/fn.markdown.html
//! [`markdown_v2`]: ./markdown_v2/fn.markdown_v2.html

pub mod markdown_v2;

pub use markdown_v2::markdown_v2;

mod bold;
mod code_block;
mod inline_code;
mod italic;
mod raw;
mod strikethrough;
mod underline;

pub use bold::{bold, Bold};
pub use code_block::{code_block, CodeBlock};
pub use inline_code::{inline_code, InlineCode};
pub use italic::{italic, Italic};
pub use raw::{raw, Raw};
pub use strikethrough::{strikethrough, Strikethrough};
pub use underline::{underline, Underline};

/// A value that can be formatted in all markups.
pub trait Formattable: markdown_v2::Formattable {}

impl<T> Formattable for T where T: markdown_v2::Formattable {}

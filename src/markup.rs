//! Utilities for working with markup.
//!
//! The purpose of this module is to make it easy and painless to work with
//! different markups correctly. This module contains abstract formatters like
//! [`bold`], [`italic`], [`link`], etc. You can pass strings, other formatters,
//! tuples, slices/arrays and vectors of strings and formatters to them:
//!
//! ```
//! use tbot::markup::{italic, bold};
//! let message = bold((
//!     "*This is <b>old, ",
//!     italic("and this is bold and italic!"),
//! ));
//! ```
//!
//! [`bold`]: ./fn.bold.html
//! [`italic`]: ./fn.italic.html
//! [`link`]: ./fn.link.html
//!
//! However, you can't use their return values directly — indeed, how do they
//! know if they need to format their inputs as HTML or MarkdownV2? That's where
//! markup formatters [`html`] and [`markdown_v2`] come into play. They take
//! the return values from the abstract utilities and return values that can
//! finally be turned into [`Text`] instances:
//!
//! ```
//! # let message = tbot::markup::bold((
//! #     "*This is <b>old, ",
//! #     tbot::markup::italic("and this is bold and italic!"),
//! # ));
//! use tbot::{markup::markdown_v2, types::parameters::Text};
//! assert_eq!(
//!     Text::from(markdown_v2(message)),
//!     Text::markdown_v2(
//!         // the extra `\r`s are needed for correct parsing in edge cases
//!         "*\\*This is <b\\>old, \r_and this is bold and italic\\!\r_*",
//!     ),
//! );
//! ```
//!
//! As you can see, you can fearlessly pass any strings to formatters
//! and they'll be automatically properly escaped. Magic!
//!
//! Note that methods that support sending markup take `impl Into<Text<'_>>`,
//! so you don't need to turn formatters into `Text` manually:
//!
//! ```no_run
//! # async fn foo() {
//! use tbot::{Bot, markup::html, types::chat};
//!
//! let bot = Bot::from_env("BOT_TOKEN");
//! bot
//!     .send_message(chat::Id(42), html("<escaped text, sent as html!>"))
//!     .call()
//!     .await
//!     .unwrap();
//! }
//! ```
//!
//! [`html`]: ./html/fn.html.html
//! [`markdown_v2`]: ./markdown_v2/fn.markdown_v2.html

macro_rules! impl_primitive {
    ($trait:ty, $($primitive:ty)+) => {
        $(impl $trait for $primitive {
            fn format(
                &self,
                formatter: &mut Formatter,
                _: Nesting,
            ) -> fmt::Result {
                write!(formatter, "{}", self)
            }
        })+
    };
}

macro_rules! impl_primitives {
    ($trait:ty) => {
        impl_primitive!(
            $trait, u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64
            bool
        );
    };
}

macro_rules! impl_tuple {
    ($trait:ident; $($type:ident)+) => {
        impl<$($type,)+> $trait for ($($type,)+)
        where
            $($type: $trait,)+
        {
            fn format(
                &self,
                formatter: &mut Formatter,
                nesting: Nesting,
            ) -> fmt::Result {
                #[allow(non_snake_case)]
                let ($($type,)+) = self;
                $($type.format(formatter, nesting)?;)+
                Ok(())
            }
        }
    };
}

macro_rules! impl_tuples {
    ($trait:ident) => {
        impl_tuple!($trait; A B);
        impl_tuple!($trait; A B C);
        impl_tuple!($trait; A B C D);
        impl_tuple!($trait; A B C D E);
        impl_tuple!($trait; A B C D E F);
        impl_tuple!($trait; A B C D E F G);
        impl_tuple!($trait; A B C D E F G H);
        impl_tuple!($trait; A B C D E F G H I);
        impl_tuple!($trait; A B C D E F G H I J);
        impl_tuple!($trait; A B C D E F G H I J K);
        // 11 ought to be enough for anybody
    };
}

pub mod html;
pub mod markdown_v2;

pub use html::html;
pub use markdown_v2::markdown_v2;

mod bold;
mod code_block;
mod inline_code;
mod italic;
mod link;
mod raw;
mod strikethrough;
mod underline;

pub use bold::{bold, Bold};
pub use code_block::{code_block, CodeBlock};
pub use inline_code::{inline_code, InlineCode};
pub use italic::{italic, Italic};
pub use link::{link, mention, Link};
pub use raw::{raw, Raw};
pub use strikethrough::{strikethrough, Strikethrough};
pub use underline::{underline, Underline};

/// A value that can be formatted in all markups.
pub trait Formattable: markdown_v2::Formattable + html::Formattable {}
impl<T: markdown_v2::Formattable + html::Formattable> Formattable for T {}

#[doc(hidden)]
#[derive(Clone, Copy, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct Nesting {
    bold: bool,
    italic: bool,
    strikethrough: bool,
    underline: bool,
}

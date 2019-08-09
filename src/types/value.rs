//! A helper type to allow both owned and borrowed values in input types and its
//! aliases.

use crate::{
    internal::AsInnerRef,
    types::{
        self, file, inline_message_id, inline_query,
        input_file::{self, InputFile},
        input_message_content, keyboard, parameters, passport, shipping, File,
        InputMessageContent, LabeledPrice, LoginUrl,
    },
};
use serde::Serialize;

/// A helper type to allow both owned and borrowed values in input types.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[serde(untagged)]
pub enum Value<O, B> {
    /// An owned value.
    Owned(O),
    /// A borrowed value.
    Borrowed(B),
}

impl<O, B> crate::internal::Sealed for Value<O, B> {}

impl<O, B> Value<O, B> {
    /// Checks if `self` is `Owned`.
    pub fn is_owned(&self) -> bool {
        match self {
            Value::Owned(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Borrowed`.
    pub fn is_borrowed(&self) -> bool {
        match self {
            Value::Borrowed(..) => true,
            _ => false,
        }
    }
}

/// An alias for the case when a value is either `T` or `&T`.
pub type Ref<'a, T> = Value<T, &'a T>;

impl<T> AsRef<T> for Ref<'_, T> {
    fn as_ref(&self) -> &T {
        match self {
            Value::Owned(value) => value,
            Value::Borrowed(value) => value,
        }
    }
}

impl<'a, T> AsInnerRef<'a, T> for Option<Ref<'a, T>> {
    fn as_inner_ref(&'a self) -> Option<&'a T> {
        match self {
            Some(value) => Some(value.as_ref()),
            None => None,
        }
    }
}

// impl<T> From<T> for Ref<'_, T> {
//     fn from(value: T) -> Self {
//         Value::Owned(value)
//     }
// }

// impl<'a, T> From<&'a T> for Ref<'a, T> {
//     fn from(value: &'a T) -> Self {
//         Value::Borrowed(value)
//     }
// }

impl<'a> From<File> for Ref<'a, File> {
    fn from(file: File) -> Self {
        Value::Owned(file)
    }
}

impl<'a> From<&'a File> for Ref<'a, File> {
    fn from(file: &'a File) -> Self {
        Value::Borrowed(file)
    }
}

macro_rules! from_for_ref {
    ($lifetime:tt: $($type:ty,)+) => {
        $(
            impl<$lifetime> From<$type> for Ref<$lifetime, $type> {
                fn from(value: $type) -> Self {
                    Value::Owned(value)
                }
            }

            impl<$lifetime> From<&$lifetime $type> for Ref<$lifetime, $type> {
                fn from(value: &$lifetime $type) -> Self {
                    Value::Borrowed(value)
                }
            }
        )+
    }
}

macro_rules! from_for_enum_ref {
    (
        $lifetime:tt, $enum_type:ty:
            $($variant_name:path: $variant_type:ty,)+
    ) => {
        $(
            impl<$lifetime> From<$variant_type> for Ref<$lifetime, $enum_type> {
                fn from(value: $variant_type) -> Self {
                    $variant_name(value.into()).into()
                }
            }

            impl<$lifetime> From<&$lifetime $variant_type> for Ref<$lifetime, $enum_type> {
                fn from(value: &$lifetime $variant_type) -> Self {
                    $variant_name(value.into()).into()
                }
            }
        )+
    };
}

from_for_ref! {
    'a: keyboard::inline::Button<'a>,
        keyboard::reply::Button<'a>,
        keyboard::inline::Keyboard<'a>,
        keyboard::reply::Keyboard<'a>,
        keyboard::Any<'a>, //
        input_file::Animation<'a>,
        input_file::Audio<'a>,
        input_file::ChatPhoto<'a>,
        input_file::Document<'a>,
        input_file::EditableMedia<'a>,
        input_file::GroupMedia<'a>,
        input_file::Photo<'a>,
        input_file::PngSticker<'a>,
        input_file::Sticker<'a>,
        input_file::Thumb<'a>,
        input_file::Video<'a>,
        input_file::VideoNote<'a>,
        input_file::Voice<'a>,
        InputFile<'a>,
        input_message_content::Contact<'a>,
        input_message_content::Location,
        input_message_content::Text<'a>,
        input_message_content::Venue<'a>,
        InputMessageContent<'a>, //
        inline_query::result::Article<'a>,
        inline_query::result::Audio<'a>,
        inline_query::result::Contact<'a>,
        inline_query::result::Document<'a>,
        inline_query::result::Game<'a>,
        inline_query::result::Gif<'a>,
        inline_query::result::Location<'a>,
        inline_query::result::Mpeg4Gif<'a>,
        inline_query::result::Photo<'a>,
        inline_query::result::Sticker<'a>,
        inline_query::result::Venue<'a>,
        inline_query::result::Video<'a>,
        inline_query::result::Voice<'a>,
        inline_query::result::Kind<'a>, //
        inline_query::Result<'a>, //
        inline_query::result::audio::Fresh<'a>,
        inline_query::result::document::Fresh<'a>,
        inline_query::result::gif::Fresh<'a>,
        inline_query::result::mpeg4_gif::Fresh<'a>,
        inline_query::result::photo::Fresh<'a>,
        inline_query::result::video::Fresh<'a>,
        inline_query::result::voice::Fresh<'a>,
        parameters::CallbackAction<'a>,
        parameters::Photo<'a>,
        LabeledPrice<'a>,
        LoginUrl<'a>,
        passport::element::Error<'a>,
        shipping::Option<'a>,
}

from_for_enum_ref! {
    'a, keyboard::Any<'a>:
        keyboard::Any::Inline: keyboard::inline::Keyboard<'a>,
        keyboard::Any::Inline: keyboard::inline::Markup<'a>,
        keyboard::Any::Inline: Vec<Vec<keyboard::inline::Button<'a>>>,
        keyboard::Any::Reply: keyboard::reply::Keyboard<'a>,
        // ForceReply: keyboard::ForceReply,
        // RemoveReply: keyboard::reply::Remove,
}

impl<'a, T> From<&'a Ref<'a, T>> for Ref<'a, T> {
    fn from(value: &'a Ref<'a, T>) -> Self {
        Value::Borrowed(value.as_ref())
    }
}

/// An alias for the case when a value is a string.
pub type String<'a> = Value<std::string::String, &'a str>;

impl<'a> String<'a> {
    /// Borrows the inner string.
    pub fn as_str(&self) -> &str {
        match self {
            Value::Owned(string) => string,
            Value::Borrowed(string) => string,
        }
    }

    /// Converts `String` to `Bytes`.
    pub fn into_bytes(self) -> Bytes<'a> {
        match self {
            Value::Owned(string) => Value::Owned(string.into_bytes()),
            Value::Borrowed(string) => Value::Borrowed(string.as_bytes()),
        }
    }
}

impl From<std::string::String> for String<'_> {
    fn from(string: std::string::String) -> Self {
        Value::Owned(string)
    }
}

impl<'a> From<&'a str> for String<'a> {
    fn from(string: &'a str) -> Self {
        Value::Borrowed(string)
    }
}

impl<'a> From<&'a std::string::String> for String<'a> {
    fn from(string: &'a std::string::String) -> Self {
        Value::Borrowed(string.as_str())
    }
}

impl<'a> From<&'a String<'a>> for String<'a> {
    fn from(string: &'a String<'a>) -> Self {
        Value::Borrowed(string.as_str())
    }
}

/// An alias for the case when a value is a file ID.
pub type FileId<'a> = Value<file::Id, file::id::Ref<'a>>;

impl FileId<'_> {
    /// Borrows the inner file ID.
    pub fn as_ref(&self) -> file::id::Ref<'_> {
        match self {
            Value::Owned(id) => id.as_ref(),
            Value::Borrowed(id) => *id,
        }
    }
}

impl From<file::Id> for FileId<'_> {
    fn from(id: file::Id) -> Self {
        Value::Owned(id)
    }
}

impl<'a> From<&'a file::Id> for FileId<'a> {
    fn from(id: &'a file::Id) -> Self {
        Value::Borrowed(id.as_ref())
    }
}

impl<'a> From<file::id::Ref<'a>> for FileId<'a> {
    fn from(id: file::id::Ref<'a>) -> Self {
        Value::Borrowed(id)
    }
}

impl<'a> From<&'a FileId<'a>> for FileId<'a> {
    fn from(id: &'a FileId<'a>) -> Self {
        Value::Borrowed(id.as_ref())
    }
}

/// An alias for the case when a value is an inline message ID.
pub type InlineMessageId<'a> =
    Value<types::InlineMessageId, inline_message_id::Ref<'a>>;

impl InlineMessageId<'_> {
    /// Borrows the inner inline message ID.
    pub fn as_ref(&self) -> inline_message_id::Ref<'_> {
        match self {
            Value::Owned(id) => id.as_ref(),
            Value::Borrowed(id) => *id,
        }
    }
}

impl From<types::InlineMessageId> for InlineMessageId<'_> {
    fn from(id: types::InlineMessageId) -> Self {
        Value::Owned(id)
    }
}

impl<'a> From<&'a types::InlineMessageId> for InlineMessageId<'a> {
    fn from(id: &'a types::InlineMessageId) -> Self {
        Value::Borrowed(id.as_ref())
    }
}

impl<'a> From<inline_message_id::Ref<'a>> for InlineMessageId<'a> {
    fn from(id: inline_message_id::Ref<'a>) -> Self {
        Value::Borrowed(id)
    }
}

impl<'a> From<&'a InlineMessageId<'a>> for InlineMessageId<'a> {
    fn from(id: &'a InlineMessageId<'a>) -> Self {
        Value::Borrowed(id.as_ref())
    }
}

/// An alias for the case when a value is a sequence.
pub type Seq<'a, T> = Value<std::vec::Vec<T>, &'a [T]>;

impl<T> Seq<'_, T> {
    /// Borrows the inner slice.
    pub fn as_slice(&self) -> &[T] {
        match self {
            Value::Owned(vec) => &vec[..],
            Value::Borrowed(slice) => *slice,
        }
    }
}

impl<T> From<std::vec::Vec<T>> for Seq<'_, T> {
    fn from(vec: std::vec::Vec<T>) -> Self {
        Value::Owned(vec)
    }
}

impl<'a, T> From<&'a std::vec::Vec<T>> for Seq<'a, T> {
    fn from(vec: &'a std::vec::Vec<T>) -> Self {
        Value::Borrowed(&vec[..])
    }
}

impl<'a, T> From<&'a [T]> for Seq<'a, T> {
    fn from(slice: &'a [T]) -> Self {
        Value::Borrowed(slice)
    }
}

impl<'a, T> From<&'a Seq<'a, T>> for Seq<'a, T> {
    fn from(vec: &'a Seq<'a, T>) -> Self {
        Value::Borrowed(vec.as_slice())
    }
}

impl<'a, T> From<Vec<T>> for Seq<'a, Ref<'a, T>> {
    fn from(value: Vec<T>) -> Self {
        value.into_iter().map(Value::Owned).collect::<Vec<_>>().into()
    }
}

impl<'a, T> From<Vec<&'a T>> for Seq<'a, Ref<'a, T>> {
    fn from(value: Vec<&'a T>) -> Self {
        value.into_iter().map(Value::Borrowed).collect::<Vec<_>>().into()
    }
}

impl<'a, T> From<&'a Vec<T>> for Seq<'a, Ref<'a, T>> {
    fn from(value: &'a Vec<T>) -> Self {
        value.iter().map(Value::Borrowed).collect::<Vec<_>>().into()
    }
}

/// An alias for the case when a value is a sequence of bytes.
pub type Bytes<'a> = Seq<'a, u8>;

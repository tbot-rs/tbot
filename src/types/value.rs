//! A helper type to allow both owned and borrowed values in input types and its
//! aliases.

use crate::{
    internal::AsInnerRef,
    types::{
        self, callback, file, inline_message_id, inline_query,
        pre_checkout_query, shipping,
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

impl<T> From<T> for Ref<'_, T> {
    fn from(value: T) -> Self {
        Value::Owned(value)
    }
}

impl<'a, T> From<&'a T> for Ref<'a, T> {
    fn from(value: &'a T) -> Self {
        Value::Borrowed(value)
    }
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

/// An alias for the case when a value is an inline query ID.
pub type InlineQueryId<'a> = Value<inline_query::Id, inline_query::id::Ref<'a>>;

impl InlineQueryId<'_> {
    /// Borrows the inner inline query ID.
    pub fn as_ref(&self) -> inline_query::id::Ref<'_> {
        match self {
            Value::Owned(id) => id.as_ref(),
            Value::Borrowed(id) => *id,
        }
    }
}

impl From<inline_query::Id> for InlineQueryId<'_> {
    fn from(id: inline_query::Id) -> Self {
        Value::Owned(id)
    }
}

impl<'a> From<&'a inline_query::Id> for InlineQueryId<'a> {
    fn from(id: &'a inline_query::Id) -> Self {
        Value::Borrowed(id.as_ref())
    }
}

impl<'a> From<inline_query::id::Ref<'a>> for InlineQueryId<'a> {
    fn from(id: inline_query::id::Ref<'a>) -> Self {
        Value::Borrowed(id)
    }
}

impl<'a> From<&'a InlineQueryId<'a>> for InlineQueryId<'a> {
    fn from(id: &'a InlineQueryId<'a>) -> Self {
        Value::Borrowed(id.as_ref())
    }
}

/// An alias for the case when a value is a pre-checkout query ID.
pub type PreCheckoutQueryId<'a> =
    Value<pre_checkout_query::Id, pre_checkout_query::id::Ref<'a>>;

impl PreCheckoutQueryId<'_> {
    /// Borrows the inner pre-checkout query ID.
    pub fn as_ref(&self) -> pre_checkout_query::id::Ref<'_> {
        match self {
            Value::Owned(id) => id.as_ref(),
            Value::Borrowed(id) => *id,
        }
    }
}

impl From<pre_checkout_query::Id> for PreCheckoutQueryId<'_> {
    fn from(id: pre_checkout_query::Id) -> Self {
        Value::Owned(id)
    }
}

impl<'a> From<&'a pre_checkout_query::Id> for PreCheckoutQueryId<'a> {
    fn from(id: &'a pre_checkout_query::Id) -> Self {
        Value::Borrowed(id.as_ref())
    }
}

impl<'a> From<pre_checkout_query::id::Ref<'a>> for PreCheckoutQueryId<'a> {
    fn from(id: pre_checkout_query::id::Ref<'a>) -> Self {
        Value::Borrowed(id)
    }
}

impl<'a> From<&'a PreCheckoutQueryId<'a>> for PreCheckoutQueryId<'a> {
    fn from(id: &'a PreCheckoutQueryId<'a>) -> Self {
        Value::Borrowed(id.as_ref())
    }
}

/// An alias for the case when a value is a shipping query ID.
pub type ShippingQueryId<'a> =
    Value<shipping::query::Id, shipping::query::id::Ref<'a>>;

impl ShippingQueryId<'_> {
    /// Borrows the inner shipping query ID.
    pub fn as_ref(&self) -> shipping::query::id::Ref<'_> {
        match self {
            Value::Owned(id) => id.as_ref(),
            Value::Borrowed(id) => *id,
        }
    }
}

impl From<shipping::query::Id> for ShippingQueryId<'_> {
    fn from(id: shipping::query::Id) -> Self {
        Value::Owned(id)
    }
}

impl<'a> From<&'a shipping::query::Id> for ShippingQueryId<'a> {
    fn from(id: &'a shipping::query::Id) -> Self {
        Value::Borrowed(id.as_ref())
    }
}

impl<'a> From<shipping::query::id::Ref<'a>> for ShippingQueryId<'a> {
    fn from(id: shipping::query::id::Ref<'a>) -> Self {
        Value::Borrowed(id)
    }
}

impl<'a> From<&'a ShippingQueryId<'a>> for ShippingQueryId<'a> {
    fn from(id: &'a ShippingQueryId<'a>) -> Self {
        Value::Borrowed(id.as_ref())
    }
}

/// Ana alias for the case when a value is a callback query ID.
pub type CallbackQueryId<'a> =
    Value<callback::query::Id, callback::query::id::Ref<'a>>;

impl CallbackQueryId<'_> {
    /// Borrows the inner callback query ID.
    pub fn as_ref(&self) -> callback::query::id::Ref<'_> {
        match self {
            Value::Owned(id) => id.as_ref(),
            Value::Borrowed(id) => *id,
        }
    }
}

impl<'a> From<callback::query::Id> for CallbackQueryId<'_> {
    fn from(id: callback::query::Id) -> Self {
        Value::Owned(id)
    }
}

impl<'a> From<&'a callback::query::Id> for CallbackQueryId<'a> {
    fn from(id: &'a callback::query::Id) -> Self {
        Value::Borrowed(id.as_ref())
    }
}

impl<'a> From<callback::query::id::Ref<'a>> for CallbackQueryId<'a> {
    fn from(id: callback::query::id::Ref<'a>) -> Self {
        Value::Borrowed(id)
    }
}
impl<'a> From<&'a CallbackQueryId<'a>> for CallbackQueryId<'a> {
    fn from(id: &'a CallbackQueryId<'a>) -> Self {
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

/// An alias for the case when a value is a sequence of bytes.
pub type Bytes<'a> = Seq<'a, u8>;

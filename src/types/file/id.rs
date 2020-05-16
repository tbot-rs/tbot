//! Types representing a file ID.

use crate::internal::Sealed;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Represents a file ID.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
#[serde(transparent)]
pub struct Id(pub String);

/// Contains a reference to a file ID.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[serde(transparent)]
pub struct Ref<'a>(pub Cow<'a, str>);

impl Id {
    /// Constructs a file ID [`Ref`] based on `self`.
    ///
    /// [`IdRef`]: ./struct.Ref.html
    #[must_use]
    pub fn as_ref(&self) -> Ref<'_> {
        Ref(self.0.as_str().into())
    }
}

impl<'a> Ref<'a> {
    /// Constructs a file [`Id`] based on `self`.
    ///
    /// [`Id`]: ./struct.Id.html
    #[must_use]
    pub fn to_owned(&self) -> Id {
        Id(self.0.clone().into_owned())
    }
}

impl From<String> for Id {
    #[must_use]
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl<'a> From<&'a str> for Ref<'a> {
    #[must_use]
    fn from(id: &'a str) -> Self {
        Self(id.into())
    }
}

impl<'a> PartialEq<Ref<'a>> for Id {
    #[must_use]
    fn eq(&self, other: &Ref<'a>) -> bool {
        self.0 == other.0
    }
}

impl<'a> PartialEq<Id> for Ref<'a> {
    #[must_use]
    fn eq(&self, other: &Id) -> bool {
        self.0.as_ref() == other.0
    }
}

impl Sealed for Id {}
impl Sealed for Ref<'_> {}

/// Allows a type with a unique file ID to act as [`file::Id`].
///
/// [`file::Id`]: ./struct.Id.html
#[allow(clippy::module_name_repetitions)] // can't think of a better name
pub trait AsFileId<'a>: Sealed {
    #[doc(hidden)]
    fn as_file_id(&'a self) -> Ref<'a>;
}

impl<'a> AsFileId<'a> for Id {
    #[must_use]
    fn as_file_id(&self) -> Ref<'_> {
        self.as_ref()
    }
}

impl<'a> AsFileId<'a> for Ref<'a> {
    #[must_use]
    fn as_file_id(&'a self) -> Ref<'a> {
        Self(Cow::Borrowed(&self.0))
    }
}

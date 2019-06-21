//! Types representing a file ID.

use crate::internal::Sealed;
use serde::{Deserialize, Serialize};

/// Represents a file ID.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
#[serde(transparent)]
pub struct Id(pub String);

/// Contains a reference to a file ID.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[serde(transparent)]
pub struct Ref<'a>(pub &'a str);

impl Id {
    /// Constructs a file ID [`Ref`] based on `self`.
    ///
    /// [`IdRef`]: ./struct.Ref.html
    pub fn as_ref(&self) -> Ref<'_> {
        Ref(&self.0)
    }
}

impl<'a> Ref<'a> {
    /// Constructs a file [`Id`] based on `self`.
    ///
    /// [`Id`]: ./struct.Id.html
    pub fn to_owned(&self) -> Id {
        Id(self.0.into())
    }
}

impl From<String> for Id {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl<'a> From<&'a str> for Ref<'a> {
    fn from(id: &'a str) -> Self {
        Self(id)
    }
}

impl<'a> PartialEq<Ref<'a>> for Id {
    fn eq(&self, other: &Ref<'a>) -> bool {
        self.0 == other.0
    }
}

impl<'a> PartialEq<Id> for Ref<'a> {
    fn eq(&self, other: &Id) -> bool {
        self.0 == other.0
    }
}

impl Sealed for Id {}
impl Sealed for Ref<'_> {}

/// Allows a type with a unique file ID to act as [`file::Id`].
///
/// [`file::Id`]: ./struct.Id.html
#[allow(clippy::module_name_repetitions)] // can't think of a better name
pub trait AsFileId: Sealed {
    #[doc(hidden)]
    fn as_file_id(&self) -> Ref<'_>;
}

impl AsFileId for Id {
    fn as_file_id(&self) -> Ref<'_> {
        self.as_ref()
    }
}

impl AsFileId for Ref<'_> {
    fn as_file_id(&self) -> Ref<'_> {
        *self
    }
}

//! Types representing a pre-checkout query ID.

use serde::{Deserialize, Serialize};

/// Represents a pre-checkout query ID.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
#[serde(transparent)]
pub struct Id(pub String);

/// Contains a reference to a pre-checkout query ID.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[serde(transparent)]
pub struct Ref<'a>(pub &'a str);

impl Id {
    /// Constructs a pre-checkout query ID [`Ref`] based on `self`.
    ///
    /// [`IdRef`]: ./struct.Ref.html
    #[must_use]
    pub fn as_ref(&self) -> Ref<'_> {
        Ref(&self.0)
    }
}

impl<'a> Ref<'a> {
    /// Constructs a pre-checkout query [`Id`] based on `self`.
    ///
    /// [`Id`]: ./struct.Id.html
    #[must_use]
    pub fn to_owned(&self) -> Id {
        Id(self.0.into())
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
        Self(id)
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
        self.0 == other.0
    }
}

//! Types representing a pre-checkout query ID.

use serde::{Deserialize, Serialize};

/// Represents a pre-checkout query ID.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
#[serde(transparent)]
pub struct Id(String);

/// Contains a reference to a pre-checkout query ID.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[serde(transparent)]
pub struct Ref<'a>(&'a str);

impl Id {
    /// Constructs a pre-checkout query ID.
    pub const fn new(id: String) -> Self {
        Self(id)
    }

    // https://github.com/rust-lang/rust-clippy/issues/4041
    #[allow(clippy::missing_const_for_fn)]
    /// Unwraps the ID.
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Constructs a pre-checkout query ID [`Ref`] based on `self`.
    ///
    /// [`IdRef`]: ./struct.Ref.html
    pub fn as_ref(&self) -> Ref<'_> {
        Ref(&self.0)
    }
}

impl<'a> Ref<'a> {
    /// Constructs a reference to a pre-checkout query ID.
    pub const fn new(id: &'a str) -> Self {
        Self(id)
    }

    /// Unwraps the ID.
    pub const fn into_inner(self) -> &'a str {
        self.0
    }

    /// Constructs a pre-checkout query [`Id`] based on `self`.
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

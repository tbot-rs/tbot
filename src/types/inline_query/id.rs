use serde::{Deserialize, Serialize};

/// Represents an inline query ID.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
#[serde(transparent)]
pub struct Id(String);

/// Contains a reference to an inline query ID.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[serde(transparent)]
pub struct IdRef<'a>(&'a str);

impl Id {
    /// Constructs an inline query ID.
    pub const fn new(id: String) -> Self {
        Self(id)
    }

    // https://github.com/rust-lang/rust-clippy/issues/4041
    #[allow(clippy::missing_const_for_fn)]
    /// Unwraps the ID.
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Constructs an inline query [`IdRef`] based on `self`.
    ///
    /// [`IdRef`]: ./struct.IdRef.html
    pub fn as_ref(&self) -> IdRef<'_> {
        IdRef(&self.0)
    }
}

impl<'a> IdRef<'a> {
    /// Constructs a reference to an inline query ID.
    pub const fn new(id: &'a str) -> Self {
        Self(id)
    }

    /// Unwraps the ID.
    pub const fn into_inner(self) -> &'a str {
        self.0
    }

    /// Constructs an inline query [`Id`] based on `self`.
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

impl<'a> From<&'a str> for IdRef<'a> {
    fn from(id: &'a str) -> Self {
        Self(id)
    }
}

impl<'a> PartialEq<IdRef<'a>> for Id {
    fn eq(&self, other: &IdRef<'a>) -> bool {
        self.0 == other.0
    }
}

impl<'a> PartialEq<Id> for IdRef<'a> {
    fn eq(&self, other: &Id) -> bool {
        self.0 == other.0
    }
}

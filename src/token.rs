use std::{
    fmt::{self, Debug, Formatter},
    sync::Arc,
};

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Token(Arc<String>);

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Ref<'a>(&'a str);

impl Token {
    pub fn new(token: String) -> Self {
        Self(Arc::new(token))
    }

    pub fn as_ref(&self) -> Ref<'_> {
        Ref(self.0.as_str())
    }
}

impl<'a> Ref<'a> {
    pub(crate) const fn as_str(self) -> &'a str {
        self.0
    }
}

impl Debug for Token {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("Token(..)")
    }
}

impl Debug for Ref<'_> {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("Token(..)")
    }
}

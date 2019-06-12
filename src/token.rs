use std::{
    fmt::{self, Debug, Formatter},
    sync::Arc,
};

/// Represents a token.
///
/// This is a wrapper around `Arc<String>` with the purpose of providing a safer
/// API. In addition, its `Debug` implementation doesn't reveal the token.
#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Token(Arc<String>);

impl Token {
    /// Constructs a `Token`.
    pub fn new(token: String) -> Self {
        Self(Arc::new(token))
    }

    pub(crate) fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Debug for Token {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("Token(..)")
    }
}

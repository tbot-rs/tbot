use super::*;

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
}

impl std::fmt::Debug for Token {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Token(..)")
    }
}

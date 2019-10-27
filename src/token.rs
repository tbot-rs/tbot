use std::{
    fmt::{self, Debug, Formatter},
    sync::Arc,
};

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Token(Arc<String>);

impl Token {
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

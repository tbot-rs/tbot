use std::fmt::{self, Debug, Formatter};

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Token(pub(crate) String);

impl Debug for Token {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("Token(..)")
    }
}

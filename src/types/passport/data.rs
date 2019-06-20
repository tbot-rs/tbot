use crate::types::passport;
use serde::Deserialize;

/// Represents [`PassportData`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#passportdata
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
// todo: #[non_exhaustive]
pub struct Data {
    /// Documents shared with the bot.
    pub data: Vec<passport::Element>,
    /// Credentials required to decrypt the data.
    pub credentials: passport::Credentials,
}

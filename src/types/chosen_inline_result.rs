use super::{Location, User};
use serde::Deserialize;

/// Represents a [`ChosenInlineResult`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#choseninlineresult
#[derive(Debug, PartialEq, Clone, Deserialize)]
// todo: #[non_exhaustive]
pub struct ChosenInlineResult {
    /// ID of the chosen result.
    pub result_id: String,
    /// The user who chose the result.
    pub from: User,
    /// The location of the user, if enabled and allowed.
    pub location: Option<Location>,
    /// The ID of the sent message.
    pub inline_message_id: Option<String>,
    /// The query used to obtain the result.
    pub query: String,
}

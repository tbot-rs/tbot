use super::*;

/// Represents a [`PollOption`].
///
/// [`PollOption`]: https://core.telegram.org/bots/api#polloption
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct PollOption {
    /// The option's text.
    pub text: String,
    /// How many people chose this option.
    pub voter_count: u64,
}

/// Represents a [`Poll`].
///
/// [`Poll`]: https://core.telegram.org/bots/api#poll
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct Poll {
    /// The id of the poll.
    pub id: String,
    /// The question of the poll.
    pub question: String,
    /// The options of the poll.
    pub options: Vec<PollOption>,
    /// Whether the poll is closed.
    pub is_closed: bool,
}

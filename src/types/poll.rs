//! Types related to polls.

use serde::Deserialize;
use std::option;

/// Represents the kind of the [`Poll`].
///
/// [`Poll`]: ./struct.Poll.html
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[non_exhaustive]
pub enum Kind {
    /// A regular poll.
    Regular {
        /// `true` if the poll allows multiple answers.
        allows_multiple_answers: bool,
    },
    /// The quiz poll.
    Quiz {
        /// The identifier of the correct answer option.
        correct_option_id: option::Option<u64>,
    },
}

/// Represents a [`PollOption`].
///
/// [`PollOption`]: https://core.telegram.org/bots/api#polloption
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
#[non_exhaustive]
pub struct Option {
    /// The text of the option.
    pub text: String,
    /// How many people chose this option.
    pub voter_count: u64,
}

/// Represents a [`Poll`].
///
/// [`Poll`]: https://core.telegram.org/bots/api#poll
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[non_exhaustive]
pub struct Poll {
    /// The kind of the poll.
    pub kind: Kind,
    /// The ID of the poll.
    pub id: String,
    /// The question of the poll.
    pub question: String,
    /// The options of the poll.
    pub options: Vec<Option>,
    /// How many people voted in this poll.
    pub total_voter_count: u64,
    /// `true` if the poll is closed.
    pub is_closed: bool,
    /// `true` if the poll is anonymous.
    pub is_anonymous: bool,
}

const ID: &str = "id";
const QUESTION: &str = "question";
const OPTIONS: &str = "options";
const TOTAL_VOTER_COUNT: &str = "total_voter_count";
const IS_CLOSED: &str = "is_closed";
const IS_ANONYMOUS: &str = "is_anonymous";
const KIND: &str = "type";
const ALLOWS_MULTIPLE_ANSWERS: &str = "allows_multiple_answers";
const CORRECT_OPTION_ID: &str = "correct_option_id";

const REGULAR: &str = "regular";
const QUIZ: &str = "quiz";

struct PollVisitor;

impl<'v> serde::de::Visitor<'v> for PollVisitor {
    type Value = Poll;

    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "struct Poll")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: serde::de::MapAccess<'v>,
    {
        let mut id = None;
        let mut question = None;
        let mut options = None;
        let mut total_voter_count = None;
        let mut is_closed = None;
        let mut is_anonymous = None;
        let mut kind = None;
        let mut allows_multiple_answers = None;
        let mut correct_option_id = None;

        while let Some(key) = map.next_key()? {
            match key {
                ID => id = Some(map.next_value()?),
                QUESTION => question = Some(map.next_value()?),
                OPTIONS => options = Some(map.next_value()?),
                TOTAL_VOTER_COUNT => {
                    total_voter_count = Some(map.next_value()?)
                }
                IS_CLOSED => is_closed = Some(map.next_value()?),
                IS_ANONYMOUS => is_anonymous = Some(map.next_value()?),
                KIND => kind = Some(map.next_value()?),
                ALLOWS_MULTIPLE_ANSWERS => {
                    allows_multiple_answers = Some(map.next_value()?)
                }
                CORRECT_OPTION_ID => {
                    correct_option_id = Some(map.next_value()?)
                }
                _ => {
                    let _ = map.next_value::<serde_json::Value>();
                }
            }
        }

        let kind = match kind {
            Some(REGULAR) => Kind::Regular {
                allows_multiple_answers: allows_multiple_answers.ok_or_else(
                    || serde::de::Error::missing_field(ALLOWS_MULTIPLE_ANSWERS),
                )?,
            },
            Some(QUIZ) => Kind::Quiz { correct_option_id },
            None => return Err(serde::de::Error::missing_field(KIND)),
            Some(unknown_kind) => {
                return Err(serde::de::Error::unknown_variant(
                    unknown_kind,
                    &[REGULAR, QUIZ],
                ))
            }
        };

        Ok(Poll {
            kind,
            id: id.ok_or_else(|| serde::de::Error::missing_field(ID))?,
            question: question
                .ok_or_else(|| serde::de::Error::missing_field(QUESTION))?,
            options: options
                .ok_or_else(|| serde::de::Error::missing_field(OPTIONS))?,
            total_voter_count: total_voter_count.ok_or_else(|| {
                serde::de::Error::missing_field(TOTAL_VOTER_COUNT)
            })?,
            is_closed: is_closed
                .ok_or_else(|| serde::de::Error::missing_field(IS_CLOSED))?,
            is_anonymous: is_anonymous
                .ok_or_else(|| serde::de::Error::missing_field(IS_ANONYMOUS))?,
        })
    }
}

impl<'de> Deserialize<'de> for Poll {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "Poll",
            &[
                ID,
                QUESTION,
                OPTIONS,
                TOTAL_VOTER_COUNT,
                IS_CLOSED,
                IS_ANONYMOUS,
                KIND,
                ALLOWS_MULTIPLE_ANSWERS,
                CORRECT_OPTION_ID,
            ],
            PollVisitor,
        )
    }
}

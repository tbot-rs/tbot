//! Types related to polls.

use super::{message::Text, User};
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::Deserialize;
use std::fmt;
use std::option;

/// Represents the kind of a [`Poll`].
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
    /// A quiz.
    Quiz {
        /// The index of the correct option.
        correct_option_id: option::Option<usize>,
        /// The explanation of the quiz.
        explanation: option::Option<Text>,
    },
}

/// Tells when the poll will be automatically closed.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[non_exhaustive]
pub struct AutoClose {
    /// The period in seconds while the poll is open.
    pub open_period: u16,
    /// The time instant when the poll is closed.
    pub close_date: i64,
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
    /// How many people voted in the poll.
    pub total_voter_count: u64,
    /// `true` if the poll is closed.
    pub is_closed: bool,
    /// `true` if the poll is anonymous.
    pub is_anonymous: bool,
    /// Tells when the poll will be automatically closed.
    pub auto_close: option::Option<AutoClose>,
}

/// Represents a [`PollAnswer`].
///
/// [`PollAnswer`]: https://core.telegram.org/bots/api#pollanswer
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct Answer {
    /// The ID of the poll.
    pub poll_id: String,
    /// The user who voted.
    pub user: User,
    /// The index of options choosen by user.
    pub option_ids: Vec<usize>,
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
const EXPLANATION: &str = "explanation";
const EXPLANATION_ENTITIES: &str = "explanation_entities";
const OPEN_PERIOD: &str = "open_period";
const CLOSE_DATE: &str = "close_date";

const REGULAR: &str = "regular";
const QUIZ: &str = "quiz";

struct PollVisitor;

impl<'v> Visitor<'v> for PollVisitor {
    type Value = Poll;

    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "struct Poll")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'v>,
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
        let mut explanation = None;
        let mut explanation_entities = None;
        let mut open_period = None;
        let mut close_date = None;

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
                EXPLANATION => explanation = Some(map.next_value()?),
                EXPLANATION_ENTITIES => {
                    explanation_entities = Some(map.next_value()?)
                }
                OPEN_PERIOD => open_period = Some(map.next_value()?),
                CLOSE_DATE => close_date = Some(map.next_value()?),
                _ => {
                    let _ = map.next_value::<de::IgnoredAny>();
                }
            }
        }

        let explanation = match explanation {
            Some(explanation) => Some(Text {
                value: explanation,
                entities: explanation_entities.unwrap_or_default(),
            }),
            None => None,
        };

        let kind = match kind {
            Some(REGULAR) => Kind::Regular {
                allows_multiple_answers: allows_multiple_answers.ok_or_else(
                    || de::Error::missing_field(ALLOWS_MULTIPLE_ANSWERS),
                )?,
            },
            Some(QUIZ) => Kind::Quiz {
                correct_option_id,
                explanation,
            },
            None => return Err(de::Error::missing_field(KIND)),
            Some(unknown_kind) => {
                return Err(de::Error::unknown_variant(
                    unknown_kind,
                    &[REGULAR, QUIZ],
                ));
            }
        };

        let auto_close = match open_period {
            Some(open_period) => Some(AutoClose {
                open_period,
                close_date: close_date
                    .ok_or_else(|| de::Error::missing_field(CLOSE_DATE))?,
            }),
            None => None,
        };

        Ok(Poll {
            kind,
            id: id.ok_or_else(|| de::Error::missing_field(ID))?,
            question: question
                .ok_or_else(|| de::Error::missing_field(QUESTION))?,
            options: options
                .ok_or_else(|| de::Error::missing_field(OPTIONS))?,
            total_voter_count: total_voter_count
                .ok_or_else(|| de::Error::missing_field(TOTAL_VOTER_COUNT))?,
            is_closed: is_closed
                .ok_or_else(|| de::Error::missing_field(IS_CLOSED))?,
            is_anonymous: is_anonymous
                .ok_or_else(|| de::Error::missing_field(IS_ANONYMOUS))?,
            auto_close,
        })
    }
}

impl<'de> Deserialize<'de> for Poll {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
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
                EXPLANATION,
                EXPLANATION_ENTITIES,
            ],
            PollVisitor,
        )
    }
}

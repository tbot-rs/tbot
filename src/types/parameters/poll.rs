//! Types related to polls.

use super::{ParseMode, Text};
use serde::Serialize;

/// Configures whether multiple answers are allowed in a poll.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub enum Answer {
    /// Only a single answer is allowed.
    Single,
    /// Multiple answers are allowed.
    Multiple,
}

/// Tells when the poll will be automatically closed.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum AutoClose {
    /// Reflects the `open_period` field.
    OpenPeriod(u16),
    /// Reflects the `close_date` field.
    CloseDate(i64),
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Kind<'a> {
    Quiz {
        correct_option_id: usize,
        #[serde(skip_serializing_if = "Option::is_none")]
        explanation: Option<&'a str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        explanation_parse_mode: Option<ParseMode>,
    },
    Regular {
        allows_multiple_answers: bool,
    },
}

/// Represents a poll that will be sent to a user.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Poll<'a> {
    #[serde(flatten)]
    kind: Kind<'a>,
    question: &'a str,
    options: &'a [&'a str],
    #[serde(skip_serializing_if = "Option::is_none")]
    is_closed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_anonymous: Option<bool>,
}

impl<'a> Poll<'a> {
    /// Constructs a quiz.
    #[must_use]
    pub fn quiz(
        question: &'a str,
        options: &'a [&'a str],
        correct_option_id: usize,
        explanation: Option<impl Into<Text<'a>>>,
    ) -> Self {
        let (explanation, explanation_parse_mode) = match explanation {
            Some(explanation) => {
                let explanation = explanation.into();

                (Some(explanation.text), explanation.parse_mode)
            }
            None => (None, None),
        };

        Self {
            kind: Kind::Quiz {
                correct_option_id,
                explanation,
                explanation_parse_mode,
            },
            question,
            options,
            is_closed: None,
            is_anonymous: None,
        }
    }

    /// Constructs a regular poll.
    #[must_use]
    pub fn regular(
        question: &'a str,
        options: &'a [&'a str],
        answers: Answer,
    ) -> Self {
        Self {
            kind: Kind::Regular {
                allows_multiple_answers: answers == Answer::Multiple,
            },
            question,
            options,
            is_closed: None,
            is_anonymous: None,
        }
    }

    /// Configures if the poll is immediately closed.
    #[must_use]
    pub fn immediately_closed(mut self, is_closed: bool) -> Self {
        self.is_closed = Some(is_closed);
        self
    }

    /// Comfigures if the poll is anonymous.
    #[must_use]
    pub fn anonymous(mut self, is_anonymous: bool) -> Self {
        self.is_anonymous = Some(is_anonymous);
        self
    }
}

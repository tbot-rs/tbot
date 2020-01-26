use serde::Serialize;

/// Configures whether multiple answers are allowed in a poll.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub enum Answer {
    /// Only a single answer is allowed.
    Single,
    /// Multiple answers are allowed.
    Multiple,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Kind {
    Quiz { correct_option_id: usize },
    Regular { allows_multiple_answers: bool },
}

/// Represents a poll that will be sent to a user.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Poll<'a> {
    #[serde(flatten)]
    kind: Kind,
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
    ) -> Self {
        Self {
            kind: Kind::Quiz { correct_option_id },
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

use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Kind {
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
    is_closed: Option<bool>,
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
        allows_multiple_answers: bool,
    ) -> Self {
        Self {
            kind: Kind::Regular {
                allows_multiple_answers,
            },
            question,
            options,
            is_closed: None,
            is_anonymous: None,
        }
    }

    /// Configures if the poll will be immediately closed.
    #[must_use]
    pub fn immediately_closed(mut self, is_closed: bool) -> Self {
        self.is_closed = Some(is_closed);
        self
    }

    /// Comfigures if the poll will be anonymous.
    #[must_use]
    pub fn anonymous(mut self, is_anonymous: bool) -> Self {
        self.is_anonymous = Some(is_anonymous);
        self
    }
}

//! Types representing inline keyboards.

use serde::{Deserialize, Serialize};

pub mod button;

use button::Button;

/// A shorthand for inline markup.
pub type Markup = Vec<Vec<Button>>;

/// Represents an [`InlineKeyboardMarkup`].
///
/// [`InlineKeyboardMarkup`]: https://core.telegram.org/bots/api#inlinekeyboardmarkup
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
#[non_exhaustive]
#[must_use]
pub struct Keyboard {
    /// The inline keyboard's markup.
    pub inline_keyboard: Markup,
}

impl Keyboard {
    /// Constructs an inline `Keyboard`.
    pub fn new(markup: impl Into<Markup>) -> Self {
        Self {
            inline_keyboard: markup.into(),
        }
    }
}

impl From<Markup> for Keyboard {
    fn from(markup: Markup) -> Self {
        Self::new(markup)
    }
}

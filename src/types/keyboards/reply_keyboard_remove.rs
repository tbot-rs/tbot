use serde::ser::SerializeMap;

/// Represents a [`ReplyKeyboardRemove`].
///
/// [`ReplyKeyboardRemove`]: https://core.telegram.org/bots/api#replykeyboardremove
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
#[must_use]
pub struct ReplyKeyboardRemove {
    // remove_keyboard is added when serializing
    selective: Option<bool>,
}

impl ReplyKeyboardRemove {
    /// Constructs a new `ReplyKeyboardRemove`.
    pub fn new() -> ReplyKeyboardRemove {
        ReplyKeyboardRemove {
            selective: None,
        }
    }

    /// Sets `selective` to `Some(is_selective)`.
    pub fn selective(mut self, is_selective: bool) -> Self {
        self.selective = Some(is_selective);
        self
    }
}

impl serde::Serialize for ReplyKeyboardRemove {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let len = if self.selective.is_some() {
            2
        } else {
            1
        };

        let mut map = s.serialize_map(Some(len))?;

        map.serialize_entry("remove_keyboard", &true)?;

        if let Some(selective) = self.selective {
            map.serialize_entry("selective", &selective)?;
        }

        map.end()
    }
}

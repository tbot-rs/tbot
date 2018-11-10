use serde::ser::SerializeMap;

/// Represents Telegram's [`ForceReply`].
///
/// [`ForceReply`]: https://core.telegram.org/bots/api#forcereply
#[derive(Debug, PartialEq, Clone)]
pub struct ForceReply {
    // force_reply is added when serialized
    selective: Option<bool>,
}

impl ForceReply {
    /// Constructs a new `ForceReply`.
    #[must_use]
    pub fn new() -> ForceReply {
        ForceReply {
            selective: None,
        }
    }

    /// Sets `selective` to `Some(is_selective)`.
    #[must_use]
    pub fn selective(mut self, is_selective: bool) -> Self {
        self.selective = Some(is_selective);
        self
    }
}

impl serde::Serialize for ForceReply {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let len = if let Some(_) = self.selective {
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

use super::*;
use serde::ser::SerializeMap;

/// Represents an [`InputMediaPhoto`].
///
/// [`InputMediaPhoto`]: https://core.telegram.org/bots/api#inputmediaphoto
pub struct InputMediaPhoto<'a> {
    pub(crate) media: InputFile<'a>,
    caption: Option<&'a str>,
    parse_mode: Option<ParseMode>,
}

impl<'a> InputMediaPhoto<'a> {
    pub(crate) fn new(media: InputFile<'a>) -> Self {
        Self {
            media,
            caption: None,
            parse_mode: None,
        }
    }

    /// Constructs a new `InputMediaPhoto` from bytes.
    pub fn bytes(bytes: &'a [u8]) -> Self {
        Self::new(InputFile::File {
            name: "photo".into(),
            filename: "photo.jpg",
            bytes,
        })
    }

    /// Constructs a new `InputMediaPhoto` from a file ID.
    ///
    /// # Panics
    ///
    /// Panicks if the ID starts with `attach://`.
    pub fn id(id: &'a str) -> Self {
        assert!(
            !id.starts_with("attach://"),
            "tbot: photo's ID cannot start with `attach://`",
        );

        Self::new(InputFile::Id(id))
    }

    /// Constructs a new `InputMediaPhoto` from a URL.
    ///
    /// # Panics
    ///
    /// Panicks if the URL starts with `attach://`.
    pub fn url(url: &'a str) -> Self {
        assert!(
            !url.starts_with("attach://"),
            "tbot: photo's URL cannot start with `attach://`",
        );

        Self::new(InputFile::Url(url))
    }

    /// Configures `caption`.
    pub fn caption(mut self, caption: &'a str) -> Self {
        self.caption = Some(caption);
        self
    }

    /// Configures `parse_mode`.
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }
}

impl<'a> serde::Serialize for InputMediaPhoto<'a> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = s.serialize_map(None)?;

        map.serialize_entry("type", "photo")?;
        map.serialize_entry("media", &self.media)?;

        if let Some(caption) = self.caption {
            map.serialize_entry("caption", caption)?;
        }

        if let Some(parse_mode) = self.parse_mode {
            map.serialize_entry("parse_mode", &parse_mode)?;
        }

        map.end()
    }
}

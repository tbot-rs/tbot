//! Types for representing an [`InlineQueryResult`][docs].
//!
//! [docs]: https://core.telegram.org/bots/api#inputmessagecontent

use crate::types::{
    keyboard::inline,
    value::{self, Ref},
};
use serde::Serialize;

pub mod article;
pub mod audio;
pub mod document;
pub mod photo;
pub mod video;
pub mod voice;

mod contact;
mod game;
mod location;
mod sticker;
mod venue;

mod gifs;
mod thumb;

pub use {
    article::Article,
    audio::Audio,
    contact::Contact,
    document::Document,
    game::Game,
    gifs::{
        gif::{self, Gif},
        mpeg4::{self as mpeg4_gif, Mpeg4Gif},
    },
    location::Location,
    photo::Photo,
    sticker::Sticker,
    thumb::Thumb,
    venue::Venue,
    video::Video,
    voice::Voice,
};

/// Represents different kinds of [`InlineQueryResult`].
///
/// [`InlineQueryResult`]: ./struct.InlineQueryResult.html
#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
// todo: #[non_exhaustive]
pub enum Kind<'a> {
    /// An article.
    Article(Ref<'a, Article<'a>>),
    /// An audio.
    Audio(Ref<'a, Audio<'a>>),
    /// A contact.
    Contact(Ref<'a, Contact<'a>>),
    /// A document.
    Document(Ref<'a, Document<'a>>),
    /// A game.
    Game(Ref<'a, Game<'a>>),
    /// A GIF.
    Gif(Ref<'a, Gif<'a>>),
    /// A location.
    Location(Ref<'a, Location<'a>>),
    /// A MPEG-4 GIF.
    Mpeg4Gif(Ref<'a, Mpeg4Gif<'a>>),
    /// A photo.
    Photo(Ref<'a, Photo<'a>>),
    /// A sticker.
    Sticker(Ref<'a, Sticker<'a>>),
    /// A venue.
    Venue(Ref<'a, Venue<'a>>),
    /// A video.
    Video(Ref<'a, Video<'a>>),
    /// A voice.
    Voice(Ref<'a, Voice<'a>>),
}

/// Represents an [`InlineQueryResult`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inputmessagecontent
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Result<'a> {
    id: value::String<'a>,
    #[serde(flatten)]
    kind: Ref<'a, Kind<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<inline::Keyboard<'a>>,
}

impl Kind<'_> {
    /// Checks if `self` is `Article`.
    pub fn is_article(&self) -> bool {
        match self {
            Kind::Article(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Audio`.
    pub fn is_audio(&self) -> bool {
        match self {
            Kind::Audio(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Contact`.
    pub fn is_contact(&self) -> bool {
        match self {
            Kind::Contact(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Document`.
    pub fn is_document(&self) -> bool {
        match self {
            Kind::Document(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Game`.
    pub fn is_game(&self) -> bool {
        match self {
            Kind::Game(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Gif`.
    pub fn is_gif(&self) -> bool {
        match self {
            Kind::Gif(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Location`.
    pub fn is_location(&self) -> bool {
        match self {
            Kind::Location(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is Mpeg4Gif``.
    pub fn is_mpeg4_gif(&self) -> bool {
        match self {
            Kind::Mpeg4Gif(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is ``.
    pub fn is_photo(&self) -> bool {
        match self {
            Kind::Photo(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Sticker`.
    pub fn is_sticker(&self) -> bool {
        match self {
            Kind::Sticker(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Venue`.
    pub fn is_venue(&self) -> bool {
        match self {
            Kind::Venue(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Video`.
    pub fn is_video(&self) -> bool {
        match self {
            Kind::Video(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Voice`.
    pub fn is_voice(&self) -> bool {
        match self {
            Kind::Voice(..) => true,
            _ => false,
        }
    }
}

impl<'a> Result<'a> {
    /// Constructs an inline query `Result`.
    pub fn new(
        id: impl Into<value::String<'a>>,
        kind: impl Into<Ref<'a, Kind<'a>>>,
    ) -> Self {
        Self {
            id: id.into(),
            kind: kind.into(),
            reply_markup: None,
        }
    }

    /// Configures `reply_markup`.
    pub fn reply_markup(mut self, markup: inline::Keyboard<'a>) -> Self {
        self.reply_markup = Some(markup);
        self
    }
}

impl<'a> From<Audio<'a>> for Kind<'a> {
    fn from(audio: Audio<'a>) -> Self {
        Kind::Audio(audio.into())
    }
}

impl<'a> From<&'a Audio<'a>> for Kind<'a> {
    fn from(audio: &'a Audio<'a>) -> Self {
        Kind::Audio(audio.into())
    }
}

impl<'a> From<Document<'a>> for Kind<'a> {
    fn from(document: Document<'a>) -> Self {
        Kind::Document(document.into())
    }
}

impl<'a> From<&'a Document<'a>> for Kind<'a> {
    fn from(document: &'a Document<'a>) -> Self {
        Kind::Document(document.into())
    }
}

impl<'a> From<Gif<'a>> for Kind<'a> {
    fn from(gif: Gif<'a>) -> Self {
        Kind::Gif(gif.into())
    }
}

impl<'a> From<&'a Gif<'a>> for Kind<'a> {
    fn from(gif: &'a Gif<'a>) -> Self {
        Kind::Gif(gif.into())
    }
}

impl<'a> From<Mpeg4Gif<'a>> for Kind<'a> {
    fn from(gif: Mpeg4Gif<'a>) -> Self {
        Kind::Mpeg4Gif(gif.into())
    }
}

impl<'a> From<&'a Mpeg4Gif<'a>> for Kind<'a> {
    fn from(gif: &'a Mpeg4Gif<'a>) -> Self {
        Kind::Mpeg4Gif(gif.into())
    }
}

impl<'a> From<Photo<'a>> for Kind<'a> {
    fn from(photo: Photo<'a>) -> Self {
        Kind::Photo(photo.into())
    }
}

impl<'a> From<&'a Photo<'a>> for Kind<'a> {
    fn from(photo: &'a Photo<'a>) -> Self {
        Kind::Photo(photo.into())
    }
}

impl<'a> From<Sticker<'a>> for Kind<'a> {
    fn from(sticker: Sticker<'a>) -> Self {
        Kind::Sticker(sticker.into())
    }
}

impl<'a> From<&'a Sticker<'a>> for Kind<'a> {
    fn from(sticker: &'a Sticker<'a>) -> Self {
        Kind::Sticker(sticker.into())
    }
}

impl<'a> From<Video<'a>> for Kind<'a> {
    fn from(video: Video<'a>) -> Self {
        Kind::Video(video.into())
    }
}

impl<'a> From<&'a Video<'a>> for Kind<'a> {
    fn from(video: &'a Video<'a>) -> Self {
        Kind::Video(video.into())
    }
}

impl<'a> From<Voice<'a>> for Kind<'a> {
    fn from(voice: Voice<'a>) -> Self {
        Kind::Voice(voice.into())
    }
}

impl<'a> From<&'a Voice<'a>> for Kind<'a> {
    fn from(voice: &'a Voice<'a>) -> Self {
        Kind::Voice(voice.into())
    }
}

impl<'a> From<Article<'a>> for Kind<'a> {
    fn from(article: Article<'a>) -> Self {
        Kind::Article(article.into())
    }
}

impl<'a> From<&'a Article<'a>> for Kind<'a> {
    fn from(article: &'a Article<'a>) -> Self {
        Kind::Article(article.into())
    }
}

impl<'a> From<Contact<'a>> for Kind<'a> {
    fn from(contact: Contact<'a>) -> Self {
        Kind::Contact(contact.into())
    }
}

impl<'a> From<&'a Contact<'a>> for Kind<'a> {
    fn from(contact: &'a Contact<'a>) -> Self {
        Kind::Contact(contact.into())
    }
}

impl<'a> From<Game<'a>> for Kind<'a> {
    fn from(game: Game<'a>) -> Self {
        Kind::Game(game.into())
    }
}

impl<'a> From<&'a Game<'a>> for Kind<'a> {
    fn from(game: &'a Game<'a>) -> Self {
        Kind::Game(game.into())
    }
}

impl<'a> From<Location<'a>> for Kind<'a> {
    fn from(location: Location<'a>) -> Self {
        Kind::Location(location.into())
    }
}

impl<'a> From<&'a Location<'a>> for Kind<'a> {
    fn from(location: &'a Location<'a>) -> Self {
        Kind::Location(location.into())
    }
}

impl<'a> From<Venue<'a>> for Kind<'a> {
    fn from(venue: Venue<'a>) -> Self {
        Kind::Venue(venue.into())
    }
}

impl<'a> From<&'a Venue<'a>> for Kind<'a> {
    fn from(venue: &'a Venue<'a>) -> Self {
        Kind::Venue(venue.into())
    }
}

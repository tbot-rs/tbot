//! Types for representing an [`InlineQueryResult`][docs].
//!
//! [docs]: https://core.telegram.org/bots/api#inputmessagecontent

use crate::types::keyboard::inline;
use is_macro::Is;
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
        GifThumb,
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
#[derive(Debug, PartialEq, Clone, Serialize, Is)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
#[must_use]
pub enum Kind<'a> {
    /// An article.
    Article(Article),
    /// An audio.
    Audio(Audio<'a>),
    /// A contact.
    Contact(Contact),
    /// A document.
    Document(Document<'a>),
    /// A game.
    Game(Game),
    /// A GIF.
    Gif(Gif<'a>),
    /// A location.
    Location(Location),
    /// A MPEG-4 GIF.
    #[is(name = "mpeg4_gif")]
    Mpeg4Gif(Mpeg4Gif<'a>),
    /// A photo.
    Photo(Photo<'a>),
    /// A sticker.
    Sticker(Sticker<'a>),
    /// A venue.
    Venue(Venue),
    /// A video.
    Video(Video<'a>),
    /// A voice.
    Voice(Voice<'a>),
}

/// Represents an [`InlineQueryResult`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inputmessagecontent
#[derive(Debug, PartialEq, Clone, Serialize)]
#[must_use]
pub struct Result<'a> {
    id: String,
    #[serde(flatten)]
    kind: Kind<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<inline::Keyboard<'a>>,
}

impl<'a> Result<'a> {
    /// Constructs an inline query `Result`.
    pub fn new(id: impl Into<String>, kind: impl Into<Kind<'a>>) -> Self {
        Self {
            id: id.into(),
            kind: kind.into(),
            reply_markup: None,
        }
    }

    /// Configures `reply_markup`.
    pub const fn reply_markup(mut self, markup: inline::Keyboard<'a>) -> Self {
        self.reply_markup = Some(markup);
        self
    }
}

impl<'a> From<Audio<'a>> for Kind<'a> {
    fn from(audio: Audio<'a>) -> Self {
        Self::Audio(audio)
    }
}

impl<'a> From<Document<'a>> for Kind<'a> {
    fn from(document: Document<'a>) -> Self {
        Self::Document(document)
    }
}

impl<'a> From<Gif<'a>> for Kind<'a> {
    fn from(gif: Gif<'a>) -> Self {
        Self::Gif(gif)
    }
}

impl<'a> From<Mpeg4Gif<'a>> for Kind<'a> {
    fn from(gif: Mpeg4Gif<'a>) -> Self {
        Self::Mpeg4Gif(gif)
    }
}

impl<'a> From<Photo<'a>> for Kind<'a> {
    fn from(photo: Photo<'a>) -> Self {
        Self::Photo(photo)
    }
}

impl<'a> From<Sticker<'a>> for Kind<'a> {
    fn from(sticker: Sticker<'a>) -> Self {
        Self::Sticker(sticker)
    }
}

impl<'a> From<Video<'a>> for Kind<'a> {
    fn from(video: Video<'a>) -> Self {
        Self::Video(video)
    }
}

impl<'a> From<Voice<'a>> for Kind<'a> {
    fn from(voice: Voice<'a>) -> Self {
        Self::Voice(voice)
    }
}

impl From<Article> for Kind<'_> {
    fn from(article: Article) -> Self {
        Self::Article(article)
    }
}

impl From<Contact> for Kind<'_> {
    fn from(contact: Contact) -> Self {
        Self::Contact(contact)
    }
}

impl From<Game> for Kind<'_> {
    fn from(game: Game) -> Self {
        Self::Game(game)
    }
}

impl From<Location> for Kind<'_> {
    fn from(location: Location) -> Self {
        Self::Location(location)
    }
}

impl From<Venue> for Kind<'_> {
    fn from(venue: Venue) -> Self {
        Self::Venue(venue)
    }
}

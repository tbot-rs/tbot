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

/// Represents different kinds of [`inline_query::Result`].
///
/// [`inline_query::Result`]: Result
#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq, Clone, Serialize, Is)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
#[must_use]
pub enum Kind {
    /// An article.
    Article(Article),
    /// An audio.
    Audio(Audio),
    /// A contact.
    Contact(Contact),
    /// A document.
    Document(Document),
    /// A game.
    Game(Game),
    /// A GIF.
    Gif(Gif),
    /// A location.
    Location(Location),
    /// A MPEG-4 GIF.
    #[is(name = "mpeg4_gif")]
    Mpeg4Gif(Mpeg4Gif),
    /// A photo.
    Photo(Photo),
    /// A sticker.
    Sticker(Sticker),
    /// A venue.
    Venue(Venue),
    /// A video.
    Video(Video),
    /// A voice.
    Voice(Voice),
}

/// Represents an [`InlineQueryResult`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inputmessagecontent
#[derive(Debug, PartialEq, Clone, Serialize)]
#[must_use]
pub struct Result {
    id: String,
    #[serde(flatten)]
    kind: Kind,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<inline::Keyboard>,
}

impl Result {
    /// Constructs an inline query `Result`.
    pub fn new(id: impl Into<String>, kind: impl Into<Kind>) -> Self {
        Self {
            id: id.into(),
            kind: kind.into(),
            reply_markup: None,
        }
    }

    /// Configures `reply_markup`.
    #[allow(clippy::missing_const_for_fn)]
    pub fn reply_markup(mut self, markup: inline::Keyboard) -> Self {
        self.reply_markup = Some(markup);
        self
    }
}

impl From<Audio> for Kind {
    fn from(audio: Audio) -> Self {
        Self::Audio(audio)
    }
}

impl From<Document> for Kind {
    fn from(document: Document) -> Self {
        Self::Document(document)
    }
}

impl From<Gif> for Kind {
    fn from(gif: Gif) -> Self {
        Self::Gif(gif)
    }
}

impl From<Mpeg4Gif> for Kind {
    fn from(gif: Mpeg4Gif) -> Self {
        Self::Mpeg4Gif(gif)
    }
}

impl From<Photo> for Kind {
    fn from(photo: Photo) -> Self {
        Self::Photo(photo)
    }
}

impl From<Sticker> for Kind {
    fn from(sticker: Sticker) -> Self {
        Self::Sticker(sticker)
    }
}

impl From<Video> for Kind {
    fn from(video: Video) -> Self {
        Self::Video(video)
    }
}

impl From<Voice> for Kind {
    fn from(voice: Voice) -> Self {
        Self::Voice(voice)
    }
}

impl From<Article> for Kind {
    fn from(article: Article) -> Self {
        Self::Article(article)
    }
}

impl From<Contact> for Kind {
    fn from(contact: Contact) -> Self {
        Self::Contact(contact)
    }
}

impl From<Game> for Kind {
    fn from(game: Game) -> Self {
        Self::Game(game)
    }
}

impl From<Location> for Kind {
    fn from(location: Location) -> Self {
        Self::Location(location)
    }
}

impl From<Venue> for Kind {
    fn from(venue: Venue) -> Self {
        Self::Venue(venue)
    }
}

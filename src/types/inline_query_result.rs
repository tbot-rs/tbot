//! Types for representing an [`InlineQueryResult`][docs].
//!
//! [docs]: https://core.telegram.org/bots/api#inputmessagecontent

use crate::types::InlineKeyboard;
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
#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
// todo: #[non_exhaustive]
pub enum Kind<'a> {
    /// An article.
    Article(Article<'a>),
    /// An audio.
    Audio(Audio<'a>),
    /// A contact.
    Contact(Contact<'a>),
    /// A document.
    Document(Document<'a>),
    /// A game.
    Game(Game<'a>),
    /// A GIF.
    Gif(Gif<'a>),
    /// A location.
    Location(Location<'a>),
    /// A MPEG-4 GIF.
    Mpeg4Gif(Mpeg4Gif<'a>),
    /// A photo.
    Photo(Photo<'a>),
    /// A sticker.
    Sticker(Sticker<'a>),
    /// A venue.
    Venue(Venue<'a>),
    /// A video.
    Video(Video<'a>),
    /// A voice.
    Voice(Voice<'a>),
}

/// Represents an [`InlineQueryResult`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inputmessagecontent
#[derive(Serialize)]
pub struct InlineQueryResult<'a> {
    id: &'a str,
    #[serde(flatten)]
    kind: Kind<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboard<'a>>,
}

impl<'a> InlineQueryResult<'a> {
    /// Constructs an `InlineQueryResult`.
    pub fn new(id: &'a str, kind: impl Into<Kind<'a>>) -> Self {
        Self {
            id,
            kind: kind.into(),
            reply_markup: None,
        }
    }

    /// Configures `reply_markup`.
    pub fn reply_markup(mut self, markup: InlineKeyboard<'a>) -> Self {
        self.reply_markup = Some(markup);
        self
    }
}

impl<'a> From<Audio<'a>> for Kind<'a> {
    fn from(audio: Audio<'a>) -> Self {
        Kind::Audio(audio)
    }
}

impl<'a> From<Document<'a>> for Kind<'a> {
    fn from(document: Document<'a>) -> Self {
        Kind::Document(document)
    }
}

impl<'a> From<Gif<'a>> for Kind<'a> {
    fn from(gif: Gif<'a>) -> Self {
        Kind::Gif(gif)
    }
}

impl<'a> From<Mpeg4Gif<'a>> for Kind<'a> {
    fn from(gif: Mpeg4Gif<'a>) -> Self {
        Kind::Mpeg4Gif(gif)
    }
}

impl<'a> From<Photo<'a>> for Kind<'a> {
    fn from(photo: Photo<'a>) -> Self {
        Kind::Photo(photo)
    }
}

impl<'a> From<Sticker<'a>> for Kind<'a> {
    fn from(sticker: Sticker<'a>) -> Self {
        Kind::Sticker(sticker)
    }
}

impl<'a> From<Video<'a>> for Kind<'a> {
    fn from(video: Video<'a>) -> Self {
        Kind::Video(video)
    }
}

impl<'a> From<Voice<'a>> for Kind<'a> {
    fn from(voice: Voice<'a>) -> Self {
        Kind::Voice(voice)
    }
}

impl<'a> From<Article<'a>> for Kind<'a> {
    fn from(article: Article<'a>) -> Self {
        Kind::Article(article)
    }
}

impl<'a> From<Contact<'a>> for Kind<'a> {
    fn from(contact: Contact<'a>) -> Self {
        Kind::Contact(contact)
    }
}

impl<'a> From<Game<'a>> for Kind<'a> {
    fn from(game: Game<'a>) -> Self {
        Kind::Game(game)
    }
}

impl<'a> From<Location<'a>> for Kind<'a> {
    fn from(location: Location<'a>) -> Self {
        Kind::Location(location)
    }
}

impl<'a> From<Venue<'a>> for Kind<'a> {
    fn from(venue: Venue<'a>) -> Self {
        Kind::Venue(venue)
    }
}

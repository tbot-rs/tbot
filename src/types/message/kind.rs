use super::{Message, Text};
use crate::types::{
    chat, passport, Animation, Audio, Contact, Document, Game, Invoice,
    Location, PhotoSize, Poll, Sticker, SuccessfulPayment, User, Venue, Video,
    VideoNote, Voice,
};
use is_macro::Is;

/// Represents kinds of messages.
#[derive(Debug, PartialEq, Clone, Is)]
// It warns on SuccessfulPayment — we'll need to consider to box it when we
// unraw payment types.
#[allow(clippy::large_enum_variant)]
#[non_exhaustive]
pub enum Kind {
    /// A text message.
    Text(Text),
    /// An audio. The second item is the caption.
    Audio(Audio, Text),
    /// A document. The second item is the caption.
    Document(Document, Text),
    /// An invitation to play a game.
    Game(Game),
    /// A photo. The second item is the caption, the third one is
    /// `media_group_id`, i.e. this photo belongs to an album with this ID.
    Photo(Vec<PhotoSize>, Text, Option<String>),
    /// A sticker.
    Sticker(Sticker),
    /// A video. The second item is the caption, the third one is
    /// `media_group_id`, i.e. this photo belongs to an album with this ID.
    Video(Video, Text, Option<String>),
    /// A voice message. The second item is the caption.
    Voice(Voice, Text),
    /// A video note.
    VideoNote(VideoNote),
    /// A contact.
    Contact(Contact),
    /// A location.
    Location(Location),
    /// A venue.
    Venue(Venue),
    /// An animation. The second item is the caption.
    Animation(Animation, Text),
    /// A poll.
    Poll(Poll),
    /// A service message about new chat members.
    NewChatMembers(Vec<User>),
    /// A service message about a member who left.
    LeftChatMember(User),
    /// A service message about the new chat title.
    NewChatTitle(String),
    /// A service message about the new chat photo.
    NewChatPhoto(Vec<PhotoSize>),
    /// A service message that the chat photo was deleted.
    ChatPhotoDeleted,
    /// A service message that the group was created.
    GroupCreated,
    /// A service message that the supergroup was created.
    SupergroupCreated,
    /// A service message that the channel was created.
    ChannelCreated,
    /// A service message that the group migrated to a supergroup with this ID.
    MigrateTo(chat::Id),
    /// A service message that the supergroup used to be a group with this ID.
    MigrateFrom(chat::Id),
    /// A service message that this message was pinned.
    Pinned(Box<Message>),
    /// An invoice.
    Invoice(Invoice),
    /// A service message about a successful payment.
    SuccessfulPayment(SuccessfulPayment),
    /// A connected website.
    ConnectedWebsite(String),
    /// Passport data.
    PassportData(passport::Data),
    /// Some unkonwn message kind. Probably means `tbot` is outdated.
    Unknown,
}

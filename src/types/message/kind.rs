use super::{AutoDeleteTimerChanged, Message, Text};
use crate::types::{
    chat, passport, voice_chat, Animation, Audio, Contact, Dice, Document,
    Game, Invoice, Location, PhotoSize, Poll, ProximityAlert, Sticker,
    SuccessfulPayment, User, Venue, Video, VideoNote, Voice,
};
use is_macro::Is;

/// Represents kinds of messages.
#[derive(Debug, PartialEq, Clone, Is)]
#[non_exhaustive]
pub enum Kind {
    /// A text message.
    Text(Text),
    /// An audio.
    Audio {
        /// The audio itself.
        audio: Box<Audio>,
        /// The audio's caption.
        caption: Text,
        /// If the audio is a part of an album, this is the album's ID.
        media_group_id: Option<String>,
    },
    /// A document.
    Document {
        /// The document itself.
        document: Box<Document>,
        /// The document's caption.
        caption: Text,
        /// If the document is a part of an album, this is the album's ID.
        media_group_id: Option<String>,
    },
    /// A dice.
    Dice(Dice),
    /// An invitation to play a game.
    Game(Box<Game>),
    /// A photo.
    Photo {
        /// The photo itself.
        photo: Vec<PhotoSize>,
        /// The photo's caption.
        caption: Text,
        /// If the photo is a part of an album, this is the album's ID.
        media_group_id: Option<String>,
    },
    /// A sticker.
    Sticker(Box<Sticker>),
    /// A video.
    Video {
        /// The video itself.
        video: Box<Video>,
        /// The video's caption.
        caption: Text,
        /// If the video is a part of an album, this is the album's ID.
        media_group_id: Option<String>,
    },
    /// A voice message. The second item is the caption.
    Voice {
        /// The voice itself.
        voice: Voice,
        /// The voice's caption.
        caption: Text,
    },
    /// A video note.
    VideoNote(VideoNote),
    /// A contact.
    Contact(Contact),
    /// A location.
    Location(Location),
    /// A venue.
    Venue(Venue),
    /// An animation.
    Animation {
        /// The animation itself.
        animation: Box<Animation>,
        /// The animation's caption.
        caption: Text,
    },
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
    SuccessfulPayment(Box<SuccessfulPayment>),
    /// A connected website.
    ConnectedWebsite(String),
    /// Passport data.
    PassportData(passport::Data),
    /// A proximity alert.
    ProximityAlert(ProximityAlert),
    /// A voice chat was started.
    VoiceChatStarted,
    /// Participants were invited to a voice chat.
    VoiceChatParticipantsInvited(voice_chat::ParticipantsInvited),
    /// A voice chat was ended.
    VoiceChatEnded(voice_chat::Ended),
    /// A voice chat was scheduled.
    VoiceChatScheduled(voice_chat::Scheduled),
    /// The auto-delete time was changed.
    AutoDeleteTimerChanged(AutoDeleteTimerChanged),
    /// Some unkonwn message kind. Probably means `tbot` is outdated.
    Unknown,
}

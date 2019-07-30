use super::Text;
use crate::types::*;

/// Represents kinds of messages.
#[derive(Debug, PartialEq, Clone)]
// It warns on SuccessfulPayment — we'll need to consider to box it when we
// unraw payment types.
#[allow(clippy::large_enum_variant)]
// todo: #[non_exhaustive]
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
    /// An animated sticker.
    AnimatedSticker(sticker::Animated),
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

impl Kind {
    /// Checks if `self` is `Text`.
    pub fn is_text(&self) -> bool {
        match self {
            Kind::Text(..) => true,
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

    /// Checks if `self` is `Photo`.
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

    /// Checks if `self` is `AnimatedSticker`.
    pub fn is_animated_sticker(&self) -> bool {
        match self {
            Kind::AnimatedSticker(..) => true,
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

    /// Checks if `self` is `VideoNote`.
    pub fn is_video_note(&self) -> bool {
        match self {
            Kind::VideoNote(..) => true,
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

    /// Checks if `self` is `Location`.
    pub fn is_location(&self) -> bool {
        match self {
            Kind::Location(..) => true,
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

    /// Checks if `self` is `Animation`.
    pub fn is_animation(&self) -> bool {
        match self {
            Kind::Animation(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Poll`.
    pub fn is_poll(&self) -> bool {
        match self {
            Kind::Poll(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `NewChatMembers`.
    pub fn is_new_chat_members(&self) -> bool {
        match self {
            Kind::NewChatMembers(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `LeftChatMember`.
    pub fn is_left_chat_member(&self) -> bool {
        match self {
            Kind::LeftChatMember(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `NewChatTitle`.
    pub fn is_new_chat_title(&self) -> bool {
        match self {
            Kind::NewChatTitle(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `NewChatPhoto`.
    pub fn is_new_chat_photo(&self) -> bool {
        match self {
            Kind::NewChatPhoto(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `ChatPhotoDeleted`.
    pub fn is_chat_photo_deleted(&self) -> bool {
        *self == Kind::ChatPhotoDeleted
    }

    /// Checks if `self` is `GroupCreated`.
    pub fn is_group_created(&self) -> bool {
        *self == Kind::GroupCreated
    }

    /// Checks if `self` is `SupergroupCreated`.
    pub fn is_supergroup_created(&self) -> bool {
        *self == Kind::SupergroupCreated
    }

    /// Checks if `self` is `ChannelCreated`.
    pub fn is_channel_created(&self) -> bool {
        *self == Kind::ChannelCreated
    }

    /// Checks if `self` is `MigrateTo`.
    pub fn is_migrate_to(&self) -> bool {
        match self {
            Kind::MigrateTo(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `MigrateFrom`.
    pub fn is_migrate_from(&self) -> bool {
        match self {
            Kind::MigrateFrom(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Pinned`.
    pub fn is_pinned(&self) -> bool {
        match self {
            Kind::Pinned(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Invoice`.
    pub fn is_invoice(&self) -> bool {
        match self {
            Kind::Invoice(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `SuccessfulPayment`.
    pub fn is_successful_payment(&self) -> bool {
        match self {
            Kind::SuccessfulPayment(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `ConnectedWebsite`.
    pub fn is_connected_website(&self) -> bool {
        match self {
            Kind::ConnectedWebsite(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `PassportData`.
    pub fn is_passport_data(&self) -> bool {
        match self {
            Kind::PassportData(..) => true,
            _ => false,
        }
    }
}

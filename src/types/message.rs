use super::*;

/// Represents a forward source.
#[derive(Debug, PartialEq, Clone)]
// It warns on Channel — we'll need to consider to box it when we unraw chat
// types.
#[allow(clippy::large_enum_variant)]
pub enum ForwardSource {
    /// The forward is from a user.
    User(User),
    /// The forward is from a user woh decided to hide their profile.
    HiddenUser(String),
    /// The forward is from a channel.
    Channel {
        /// Information about the channel.
        chat: raw::Chat,
        /// The original message's ID.
        message_id: u32,
        /// The author's signature.
        signature: Option<String>,
    },
}

/// Represents forward information.
#[derive(Debug, PartialEq, Clone)]
pub struct Forward {
    /// The original message's author.
    pub from: ForwardSource,
    /// The original message's date.
    pub date: i64,
}

/// Represents either a text message or a caption.
#[derive(Debug, PartialEq, Clone)]
pub struct Text {
    /// The text/caption. Empty string if no caption.
    pub text: String,
    /// The entities in the text/caption. Empty if none.
    pub entities: Vec<MessageEntity>,
}

/// Represents kinds of messages.
#[derive(Debug, PartialEq, Clone)]
// It warns on SuccessfulPayment — we'll need to consider to box it when we
// unraw payment types.
#[allow(clippy::large_enum_variant)]
pub enum MessageKind {
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
    Photo(Vec<PhotoSize>, Text, Option<i32>),
    /// A sticker.
    Sticker(Sticker),
    /// A video. The second item is the caption, the third one is
    /// `media_group_id`, i.e. this photo belongs to an album with this ID.
    Video(Video, Text, Option<i32>),
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
    MigrateTo(i64),
    /// A service message that the supergroup used to be a group with this ID.
    MigrateFrom(i64),
    /// A service message that this message was pinned.
    Pinned(Box<Message>),
    /// An invoice.
    Invoice(Invoice),
    /// A service message about a successful payment.
    SuccessfulPayment(raw::SuccessfulPayment),
    /// A connected website.
    ConnectedWebsite(String),
    /// Passport data.
    PassportData(raw::PassportData),
    /// Some unkonwn message kind. Probably means tbot is outdated.
    ///
    /// We couldn't return an error if tbot faces an unknown kind because, if
    /// we did return an error, parsing response would fail and it would be
    /// impossible to update the last offset, and it would lead to an endless
    /// loop.
    Unknown,
}

/// Represents a message
#[derive(Debug, PartialEq, Clone)]
pub struct Message {
    /// The message's ID.
    pub id: u32,
    /// The message's author. Note that this field is `None` for messages from
    /// channels.
    pub from: Option<User>,
    /// The message's date.
    pub date: i64,
    /// The chat where the message is from.
    pub chat: raw::Chat,
    /// if this message is a foward, information about the original message.
    pub forward: Option<Forward>,
    /// Reply to this message.
    pub reply_to: Option<Box<Message>>,
    /// If the message was edited, the date of last edit.
    pub edit_date: Option<i64>,
    /// The author's signature, if turned for the channel.
    pub author_signature: Option<String>,
    /// The message's kind.
    pub kind: MessageKind,
}

pub(crate) struct MessageData {
    pub id: u32,
    pub from: Option<User>,
    pub date: i64,
    pub chat: raw::Chat,
    pub forward: Option<Forward>,
    pub reply_to: Option<Box<Message>>,
    pub edit_date: Option<i64>,
    pub author_signature: Option<String>,
}

impl Message {
    pub(crate) fn new(data: MessageData, kind: MessageKind) -> Message {
        Message {
            id: data.id,
            from: data.from,
            date: data.date,
            chat: data.chat,
            forward: data.forward,
            reply_to: data.reply_to,
            edit_date: data.edit_date,
            author_signature: data.author_signature,
            kind,
        }
    }

    pub(crate) fn split(self) -> (MessageData, MessageKind) {
        let data = MessageData {
            id: self.id,
            from: self.from,
            date: self.date,
            chat: self.chat,
            forward: self.forward,
            reply_to: self.reply_to,
            edit_date: self.edit_date,
            author_signature: self.author_signature,
        };

        (data, self.kind)
    }
}

// Prepare for crap.

const MESSAGE_ID: &str = "message_id";
const FROM: &str = "from";
const DATE: &str = "date";
const CHAT: &str = "chat";
const FORWARD_FROM: &str = "forward_from";
const FORWARD_FROM_CHAT: &str = "forward_from_chat";
const FORWARD_FROM_MESSAGE_ID: &str = "forward_from_message_id";
const FORWARD_SIGNATURE: &str = "forward_signature";
const FORWARD_SENDER_NAME: &str = "forward_sender_name";
const FORWARD_DATE: &str = "forward_date";
const REPLY_TO_MESSAGE: &str = "reply_to_message";
const EDIT_DATE: &str = "edit_date";
const MEDIA_GROUP_ID: &str = "media_group_id";
const AUTHOR_SIGNATURE: &str = "author_signature";
const TEXT: &str = "text";
const ENTITIES: &str = "entities";
const CAPTION_ENTITIES: &str = "caption_entities";
const AUDIO: &str = "audio";
const DOCUMENT: &str = "document";
const ANIMATION: &str = "animation";
const GAME: &str = "game";
const PHOTO: &str = "photo";
const STICKER: &str = "sticker";
const VIDEO: &str = "video";
const VOICE: &str = "voice";
const VIDEO_NOTE: &str = "video_note";
const CAPTION: &str = "caption";
const CONTACT: &str = "contact";
const LOCATION: &str = "location";
const VENUE: &str = "venue";
const POLL: &str = "poll";
const NEW_CHAT_MEMBERS: &str = "new_chat_members";
const LEFT_CHAT_MEMBER: &str = "left_chat_member";
const NEW_CHAT_TITLE: &str = "new_chat_title";
const NEW_CHAT_PHOTO: &str = "new_chat_photo";
const DELETE_CHAT_PHOTO: &str = "delete_chat_photo";
const GROUP_CHAT_CREATED: &str = "group_chat_created";
const SUPERGROUP_CHAT_CREATED: &str = "supergroup_chat_created";
const CHANNEL_CHAT_CREATED: &str = "channel_chat_created";
const MIGRATE_TO_CHAT_ID: &str = "migrate_to_chat_id";
const MIGRATE_FROM_CHAT_ID: &str = "migrate_from_chat_id";
const PINNED_MESSAGE: &str = "pinned_message";
const INVOICE: &str = "invoice";
const SUCCESSFUL_PAYMENT: &str = "successful_payment";
const CONNECTED_WEBSITE: &str = "connected_website";
const PASSPORT_DATA: &str = "passport_data";

struct MessageVisitor;

impl<'v> serde::de::Visitor<'v> for MessageVisitor {
    type Value = Message;

    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "struct Message")
    }

    #[allow(clippy::cyclomatic_complexity)] // can't do much
    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: serde::de::MapAccess<'v>,
    {
        let mut message_id = None;
        let mut from = None;
        let mut date = None;
        let mut chat = None;
        let mut forward_from = None;
        let mut forward_from_chat = None;
        let mut forward_from_message_id = None;
        let mut forward_signature = None;
        let mut forward_sender_name = None;
        let mut forward_date = None;
        let mut reply_to_message = None;
        let mut edit_date = None;
        let mut media_group_id = None;
        let mut author_signature = None;
        let mut text = None;
        let mut entities = None;
        let mut caption_entities = None;
        let mut audio = None;
        let mut document = None;
        let mut animation = None;
        let mut game = None;
        let mut photo = None;
        let mut sticker = None;
        let mut video = None;
        let mut voice = None;
        let mut video_note = None;
        let mut caption = None;
        let mut contact = None;
        let mut location = None;
        let mut venue = None;
        let mut poll = None;
        let mut new_chat_members = None;
        let mut left_chat_member = None;
        let mut new_chat_title = None;
        let mut new_chat_photo = None;
        let mut delete_chat_photo = false;
        let mut group_chat_created = false;
        let mut supergroup_chat_created = false;
        let mut channel_chat_created = false;
        let mut migrate_to_chat_id = None;
        let mut migrate_from_chat_id = None;
        let mut pinned_message = None;
        let mut invoice = None;
        let mut successful_payment = None;
        let mut connected_website = None;
        let mut passport_data = None;

        while let Some(key) = map.next_key()? {
            match key {
                MESSAGE_ID => message_id = Some(map.next_value()?),
                FROM => from = Some(map.next_value()?),
                DATE => date = Some(map.next_value()?),
                CHAT => chat = Some(map.next_value()?),
                FORWARD_FROM => forward_from = Some(map.next_value()?),
                FORWARD_FROM_CHAT => {
                    forward_from_chat = Some(map.next_value()?)
                }
                FORWARD_FROM_MESSAGE_ID => {
                    forward_from_message_id = Some(map.next_value()?)
                }
                FORWARD_SIGNATURE => {
                    forward_signature = Some(map.next_value()?)
                }
                FORWARD_SENDER_NAME => {
                    forward_sender_name = Some(map.next_value()?)
                }
                FORWARD_DATE => forward_date = Some(map.next_value()?),
                REPLY_TO_MESSAGE => reply_to_message = Some(map.next_value()?),
                EDIT_DATE => edit_date = Some(map.next_value()?),
                MEDIA_GROUP_ID => media_group_id = Some(map.next_value()?),
                AUTHOR_SIGNATURE => author_signature = Some(map.next_value()?),
                TEXT => text = Some(map.next_value()?),
                ENTITIES => entities = Some(map.next_value()?),
                CAPTION_ENTITIES => caption_entities = Some(map.next_value()?),
                AUDIO => audio = Some(map.next_value()?),
                DOCUMENT => document = Some(map.next_value()?),
                ANIMATION => animation = Some(map.next_value()?),
                GAME => game = Some(map.next_value()?),
                PHOTO => photo = Some(map.next_value()?),
                STICKER => sticker = Some(map.next_value()?),
                VIDEO => video = Some(map.next_value()?),
                VOICE => voice = Some(map.next_value()?),
                VIDEO_NOTE => video_note = Some(map.next_value()?),
                CAPTION => caption = Some(map.next_value()?),
                CONTACT => contact = Some(map.next_value()?),
                LOCATION => location = Some(map.next_value()?),
                VENUE => venue = Some(map.next_value()?),
                POLL => poll = Some(map.next_value()?),
                NEW_CHAT_MEMBERS => new_chat_members = Some(map.next_value()?),
                LEFT_CHAT_MEMBER => left_chat_member = Some(map.next_value()?),
                NEW_CHAT_TITLE => new_chat_title = Some(map.next_value()?),
                NEW_CHAT_PHOTO => new_chat_photo = Some(map.next_value()?),
                DELETE_CHAT_PHOTO => delete_chat_photo = map.next_value()?,
                GROUP_CHAT_CREATED => group_chat_created = map.next_value()?,
                SUPERGROUP_CHAT_CREATED => {
                    supergroup_chat_created = map.next_value()?
                }
                CHANNEL_CHAT_CREATED => {
                    channel_chat_created = map.next_value()?
                }
                MIGRATE_TO_CHAT_ID => {
                    migrate_to_chat_id = Some(map.next_value()?)
                }
                MIGRATE_FROM_CHAT_ID => {
                    migrate_from_chat_id = Some(map.next_value()?)
                }
                PINNED_MESSAGE => pinned_message = Some(map.next_value()?),
                INVOICE => invoice = Some(map.next_value()?),
                SUCCESSFUL_PAYMENT => {
                    successful_payment = Some(map.next_value()?)
                }
                CONNECTED_WEBSITE => {
                    connected_website = Some(map.next_value()?)
                }
                PASSPORT_DATA => passport_data = Some(map.next_value()?),
                _ => {
                    let _ = map.next_value::<serde_json::Value>();
                }
            }
        }

        let forward_source = if let Some(chat) = forward_from_chat {
            Some(ForwardSource::Channel {
                chat,
                message_id: forward_from_message_id.ok_or_else(|| {
                    serde::de::Error::missing_field(FORWARD_FROM_MESSAGE_ID)
                })?,
                signature: forward_signature,
            })
        } else if let Some(user) = forward_from {
            Some(ForwardSource::User(user))
        } else if let Some(hidden_user) = forward_sender_name {
            Some(ForwardSource::HiddenUser(hidden_user))
        } else {
            None
        };

        let forward = if let Some(from) = forward_source {
            Some(Forward {
                from,
                date: forward_date.ok_or_else(|| {
                    serde::de::Error::missing_field(FORWARD_DATE)
                })?,
            })
        } else {
            None
        };

        let caption = Text {
            text: caption.unwrap_or_else(String::new),
            entities: caption_entities.unwrap_or_else(Vec::new),
        };

        let kind = if let Some(text) = text {
            let text = Text {
                text,
                entities: entities.unwrap_or_else(Vec::new),
            };

            MessageKind::Text(text)
        } else if let Some(audio) = audio {
            MessageKind::Audio(audio, caption)
        } else if let Some(game) = game {
            MessageKind::Game(game)
        } else if let Some(photo) = photo {
            MessageKind::Photo(photo, caption, media_group_id)
        } else if let Some(sticker) = sticker {
            MessageKind::Sticker(sticker)
        } else if let Some(video) = video {
            MessageKind::Video(video, caption, media_group_id)
        } else if let Some(voice) = voice {
            MessageKind::Voice(voice, caption)
        } else if let Some(video_note) = video_note {
            MessageKind::VideoNote(video_note)
        } else if let Some(contact) = contact {
            MessageKind::Contact(contact)
        } else if let Some(location) = location {
            MessageKind::Location(location)
        } else if let Some(venue) = venue {
            MessageKind::Venue(venue)
        } else if let Some(poll) = poll {
            MessageKind::Poll(poll)
        } else if let Some(animation) = animation {
            MessageKind::Animation(animation, caption)
        } else if let Some(document) = document {
            MessageKind::Document(document, caption)
        } else if let Some(new_chat_members) = new_chat_members {
            MessageKind::NewChatMembers(new_chat_members)
        } else if let Some(left_chat_member) = left_chat_member {
            MessageKind::LeftChatMember(left_chat_member)
        } else if let Some(new_chat_title) = new_chat_title {
            MessageKind::NewChatTitle(new_chat_title)
        } else if let Some(new_chat_photo) = new_chat_photo {
            MessageKind::NewChatPhoto(new_chat_photo)
        } else if delete_chat_photo {
            MessageKind::ChatPhotoDeleted
        } else if group_chat_created {
            MessageKind::GroupCreated
        } else if supergroup_chat_created {
            MessageKind::SupergroupCreated
        } else if channel_chat_created {
            MessageKind::ChannelCreated
        } else if let Some(migrate_to) = migrate_to_chat_id {
            MessageKind::MigrateTo(migrate_to)
        } else if let Some(migrate_from) = migrate_from_chat_id {
            MessageKind::MigrateFrom(migrate_from)
        } else if let Some(pinned) = pinned_message {
            MessageKind::Pinned(pinned)
        } else if let Some(invoice) = invoice {
            MessageKind::Invoice(invoice)
        } else if let Some(successful_payment) = successful_payment {
            MessageKind::SuccessfulPayment(successful_payment)
        } else if let Some(connected_website) = connected_website {
            MessageKind::ConnectedWebsite(connected_website)
        } else if let Some(passport_data) = passport_data {
            MessageKind::PassportData(passport_data)
        } else {
            MessageKind::Unknown
        };

        Ok(Message {
            id: message_id
                .ok_or_else(|| serde::de::Error::missing_field(MESSAGE_ID))?,
            from,
            date: date.ok_or_else(|| serde::de::Error::missing_field(DATE))?,
            chat: chat.ok_or_else(|| serde::de::Error::missing_field(CHAT))?,
            forward,
            reply_to: reply_to_message,
            edit_date,
            author_signature,
            kind,
        })
    }
}

impl<'de> serde::Deserialize<'de> for Message {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        d.deserialize_struct(
            "Message",
            &[
                MESSAGE_ID,
                FROM,
                DATE,
                CHAT,
                FORWARD_FROM,
                FORWARD_FROM_CHAT,
                FORWARD_FROM_MESSAGE_ID,
                FORWARD_SIGNATURE,
                FORWARD_SENDER_NAME,
                FORWARD_SENDER_NAME,
                FORWARD_DATE,
                REPLY_TO_MESSAGE,
                EDIT_DATE,
                MEDIA_GROUP_ID,
                AUTHOR_SIGNATURE,
                TEXT,
                ENTITIES,
                CAPTION_ENTITIES,
                AUDIO,
                DOCUMENT,
                ANIMATION,
                GAME,
                PHOTO,
                STICKER,
                VIDEO,
                VOICE,
                VIDEO_NOTE,
                CAPTION,
                CONTACT,
                LOCATION,
                VENUE,
                POLL,
                NEW_CHAT_MEMBERS,
                LEFT_CHAT_MEMBER,
                NEW_CHAT_TITLE,
                NEW_CHAT_PHOTO,
                DELETE_CHAT_PHOTO,
                GROUP_CHAT_CREATED,
                SUPERGROUP_CHAT_CREATED,
                CHANNEL_CHAT_CREATED,
                MIGRATE_TO_CHAT_ID,
                MIGRATE_FROM_CHAT_ID,
                PINNED_MESSAGE,
                INVOICE,
                SUCCESSFUL_PAYMENT,
                CONNECTED_WEBSITE,
                PASSPORT_DATA,
            ],
            MessageVisitor,
        )
    }
}

//! Types related to messages.

use super::*;

pub mod forward;
mod kind;
pub mod text;

pub use {forward::Forward, kind::Kind, text::Text};

/// Represents a message.
#[derive(Debug, PartialEq, Clone)]
// todo: #[non_exhaustive]
pub struct Message {
    /// The ID of the message.
    pub id: u32,
    /// The author of the message. Note that this field is `None` for messages
    /// from channels.
    pub from: Option<User>,
    /// The timestamp of the messagr.
    pub date: i64,
    /// The chat to which the message was sent.
    pub chat: Chat,
    /// If this message is a forward, information about the original message.
    pub forward: Option<Forward>,
    /// If `Some`, the message that this message replies to.
    pub reply_to: Option<Box<Message>>,
    /// If the message was edited, the date of last edit.
    pub edit_date: Option<i64>,
    /// The author's signature, if enabled for the channel.
    pub author_signature: Option<String>,
    /// The kind of the message.
    pub kind: Kind,
}

pub(crate) struct Data {
    pub id: u32,
    pub from: Option<User>,
    pub date: i64,
    pub chat: Chat,
    pub forward: Option<Forward>,
    pub reply_to: Option<Message>,
    pub edit_date: Option<i64>,
    pub author_signature: Option<String>,
}

impl Message {
    // https://github.com/rust-lang/rust-clippy/issues/4041
    #[allow(clippy::missing_const_for_fn)]
    pub(crate) fn new(data: Data, kind: Kind) -> Self {
        Self {
            id: data.id,
            from: data.from,
            date: data.date,
            chat: data.chat,
            forward: data.forward,
            reply_to: data.reply_to.map(Box::new),
            edit_date: data.edit_date,
            author_signature: data.author_signature,
            kind,
        }
    }

    // https://github.com/rust-lang/rust-clippy/issues/4041
    #[allow(clippy::missing_const_for_fn)]
    pub(crate) fn split(self) -> (Data, Kind) {
        let data = Data {
            id: self.id,
            from: self.from,
            date: self.date,
            chat: self.chat,
            forward: self.forward,
            reply_to: self.reply_to.map(|message| *message),
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

    #[allow(clippy::cognitive_complexity)] // can't do much
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
            Some(forward::From::Channel {
                chat,
                message_id: forward_from_message_id.ok_or_else(|| {
                    serde::de::Error::missing_field(FORWARD_FROM_MESSAGE_ID)
                })?,
                signature: forward_signature,
            })
        } else if let Some(user) = forward_from {
            Some(forward::From::User(user))
        } else if let Some(hidden_user) = forward_sender_name {
            Some(forward::From::HiddenUser(hidden_user))
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
            value: caption.unwrap_or_else(String::new),
            entities: caption_entities.unwrap_or_else(Vec::new),
        };

        let kind = if let Some(value) = text {
            let text = Text {
                value,
                entities: entities.unwrap_or_else(Vec::new),
            };

            Kind::Text(text)
        } else if let Some(audio) = audio {
            Kind::Audio(audio, caption)
        } else if let Some(game) = game {
            Kind::Game(game)
        } else if let Some(photo) = photo {
            Kind::Photo(photo, caption, media_group_id)
        } else if let Some(sticker) = sticker {
            Kind::Sticker(sticker)
        } else if let Some(video) = video {
            Kind::Video(video, caption, media_group_id)
        } else if let Some(voice) = voice {
            Kind::Voice(voice, caption)
        } else if let Some(video_note) = video_note {
            Kind::VideoNote(video_note)
        } else if let Some(contact) = contact {
            Kind::Contact(contact)
        } else if let Some(location) = location {
            Kind::Location(location)
        } else if let Some(venue) = venue {
            Kind::Venue(venue)
        } else if let Some(poll) = poll {
            Kind::Poll(poll)
        } else if let Some(animation) = animation {
            Kind::Animation(animation, caption)
        } else if let Some(document) = document {
            Kind::Document(document, caption)
        } else if let Some(new_chat_members) = new_chat_members {
            Kind::NewChatMembers(new_chat_members)
        } else if let Some(left_chat_member) = left_chat_member {
            Kind::LeftChatMember(left_chat_member)
        } else if let Some(new_chat_title) = new_chat_title {
            Kind::NewChatTitle(new_chat_title)
        } else if let Some(new_chat_photo) = new_chat_photo {
            Kind::NewChatPhoto(new_chat_photo)
        } else if delete_chat_photo {
            Kind::ChatPhotoDeleted
        } else if group_chat_created {
            Kind::GroupCreated
        } else if supergroup_chat_created {
            Kind::SupergroupCreated
        } else if channel_chat_created {
            Kind::ChannelCreated
        } else if let Some(migrate_to) = migrate_to_chat_id {
            Kind::MigrateTo(migrate_to)
        } else if let Some(migrate_from) = migrate_from_chat_id {
            Kind::MigrateFrom(migrate_from)
        } else if let Some(pinned) = pinned_message {
            Kind::Pinned(pinned)
        } else if let Some(invoice) = invoice {
            Kind::Invoice(invoice)
        } else if let Some(successful_payment) = successful_payment {
            Kind::SuccessfulPayment(successful_payment)
        } else if let Some(connected_website) = connected_website {
            Kind::ConnectedWebsite(connected_website)
        } else if let Some(passport_data) = passport_data {
            Kind::PassportData(passport_data)
        } else {
            Kind::Unknown
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

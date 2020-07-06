use crate::{
    contexts::{
        fields,
        methods::{Forwardable, Pinnable},
    },
    types::{message, Chat, User},
};

callback! {
    struct MessageGameCallback {
        /// The requested game.
        game: String,
        message: crate::types::Message,
    } -> EventLoop::message_game_callback
}

impl fields::Message for MessageGameCallback {
    fn message_id(&self) -> message::Id {
        self.message.id
    }

    fn from(&self) -> Option<&User> {
        self.message.from.as_ref()
    }

    fn date(&self) -> i64 {
        self.message.date
    }

    fn chat(&self) -> &Chat {
        &self.message.chat
    }
}

impl Forwardable for MessageGameCallback {}
impl Pinnable for MessageGameCallback {}

callback! {
    struct InlineGameCallback {
        /// The requested game.
        game: String,
        inline_message_id: String,
    } -> EventLoop::inline_game_callback
}

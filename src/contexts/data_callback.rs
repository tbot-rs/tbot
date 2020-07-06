use crate::{
    contexts::{
        fields,
        methods::{Forwardable, Pinnable},
    },
    types::{message, Chat, User},
};

callback! {
    struct MessageDataCallback {
        /// Data from the callback.
        data: String,
        message: crate::types::Message,
    } -> EventLoop::message_data_callback
}

impl fields::Message for MessageDataCallback {
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

impl Forwardable for MessageDataCallback {}
impl Pinnable for MessageDataCallback {}

callback! {
    struct InlineDataCallback {
        /// Data from the callback.
        data: String,
        inline_message_id: String,
    } -> EventLoop::inline_data_callback
}

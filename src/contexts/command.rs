use crate::{
    contexts::{
        fields::{
            self, AnyText, Context, EditedMessage, Forward, MediaMessage,
            Message,
        },
        traits::{Forwardable, Pinnable},
        EditedText, Text,
    },
    internal::Sealed,
    types::{
        self,
        message::{self, inline_markup},
        Chat, User,
    },
    Bot,
};
use std::ops::Deref;

/// A wrapping context for commands.
pub struct Command<C> {
    /// The command which fired the handler.
    pub command: String,
    context: C,
}

impl<C> Deref for Command<C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl<C> Command<C> {
    #[allow(clippy::missing_const_for_fn)]
    pub(crate) fn new(command: String, context: C) -> Self {
        Self { command, context }
    }
}

// Prepare for hell.

impl<T> Sealed for Command<T> {}

impl Context for Command<Text> {
    fn bot(&self) -> &Bot {
        &self.context.bot
    }
}

impl Message for Command<Text> {
    #[must_use]
    fn message_id(&self) -> message::Id {
        self.context.message_id
    }

    #[must_use]
    fn from(&self) -> Option<&User> {
        self.context.from.as_ref()
    }

    #[must_use]
    fn date(&self) -> i64 {
        self.context.date
    }

    #[must_use]
    fn chat(&self) -> &Chat {
        &self.context.chat
    }
}

impl MediaMessage for Command<Text> {
    #[must_use]
    fn reply_to(&self) -> Option<&types::Message> {
        self.context.reply_to.as_ref()
    }

    #[must_use]
    fn author_signature(&self) -> Option<&str> {
        self.context.author_signature.as_deref()
    }

    #[must_use]
    fn reply_markup(&self) -> Option<&inline_markup::Keyboard> {
        self.context.reply_markup.as_ref()
    }
}

impl Forward for Command<Text> {
    #[must_use]
    fn forward(&self) -> Option<&message::Forward> {
        self.context.forward()
    }
}

impl fields::Text for Command<Text> {
    #[must_use]
    fn text(&self) -> &message::Text {
        &self.context.text
    }
}

impl AnyText for Command<Text> {
    #[must_use]
    fn text(&self) -> &message::Text {
        &self.context.text
    }
}

impl<'a> Forwardable for Command<Text> {}
impl<'a> Pinnable for Command<Text> {}

// Once again.

impl Context for Command<EditedText> {
    fn bot(&self) -> &Bot {
        &self.context.bot
    }
}

impl Message for Command<EditedText> {
    #[must_use]
    fn message_id(&self) -> message::Id {
        self.context.message_id
    }

    #[must_use]
    fn from(&self) -> Option<&User> {
        self.context.from.as_ref()
    }

    #[must_use]
    fn date(&self) -> i64 {
        self.context.date
    }

    #[must_use]
    fn chat(&self) -> &Chat {
        &self.context.chat
    }
}

impl MediaMessage for Command<EditedText> {
    #[must_use]
    fn reply_to(&self) -> Option<&types::Message> {
        self.context.reply_to.as_ref()
    }

    #[must_use]
    fn author_signature(&self) -> Option<&str> {
        self.context.author_signature.as_deref()
    }

    #[must_use]
    fn reply_markup(&self) -> Option<&inline_markup::Keyboard> {
        self.context.reply_markup.as_ref()
    }
}

impl EditedMessage for Command<EditedText> {
    #[must_use]
    fn edit_date(&self) -> i64 {
        self.context.edit_date
    }
}

impl fields::Text for Command<EditedText> {
    #[must_use]
    fn text(&self) -> &message::Text {
        &self.context.text
    }
}

impl AnyText for Command<EditedText> {
    #[must_use]
    fn text(&self) -> &message::Text {
        &self.context.text
    }
}

impl<'a> Forwardable for Command<EditedText> {}
impl<'a> Pinnable for Command<EditedText> {}

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

impl<C> Context<C> for Command<Text<C>> {
    fn bot(&self) -> &Bot<C> {
        self.context.bot()
    }
}

impl<C> Message<C> for Command<Text<C>> {
    #[must_use]
    fn message_id(&self) -> message::Id {
        self.context.message_id()
    }

    #[must_use]
    fn from(&self) -> Option<&User> {
        self.context.from()
    }

    #[must_use]
    fn date(&self) -> i64 {
        self.context.date()
    }

    #[must_use]
    fn chat(&self) -> &Chat {
        self.context.chat()
    }
}

impl<C> MediaMessage<C> for Command<Text<C>> {
    #[must_use]
    fn reply_to(&self) -> Option<&types::Message> {
        self.context.reply_to()
    }

    #[must_use]
    fn author_signature(&self) -> Option<&str> {
        self.context.author_signature()
    }

    #[must_use]
    fn reply_markup(&self) -> Option<&inline_markup::Keyboard> {
        self.context.reply_markup()
    }
}

impl<C> Forward<C> for Command<Text<C>> {
    #[must_use]
    fn forward(&self) -> Option<&message::Forward> {
        self.context.forward()
    }
}

impl<C> fields::Text<C> for Command<Text<C>> {
    #[must_use]
    fn text(&self) -> &message::Text {
        &self.context.text
    }
}

impl<C> AnyText<C> for Command<Text<C>> {
    #[must_use]
    fn text(&self) -> &message::Text {
        &self.context.text
    }
}

impl<'a, C: 'static> Forwardable<'a, C> for Command<Text<C>> {}
impl<'a, C: 'static> Pinnable<'a, C> for Command<Text<C>> {}

// Once again.

impl<C> Context<C> for Command<EditedText<C>> {
    fn bot(&self) -> &Bot<C> {
        self.context.bot()
    }
}

impl<C> Message<C> for Command<EditedText<C>> {
    #[must_use]
    fn message_id(&self) -> message::Id {
        self.context.message_id()
    }

    #[must_use]
    fn from(&self) -> Option<&User> {
        self.context.from()
    }

    #[must_use]
    fn date(&self) -> i64 {
        self.context.date()
    }

    #[must_use]
    fn chat(&self) -> &Chat {
        self.context.chat()
    }
}

impl<C> MediaMessage<C> for Command<EditedText<C>> {
    #[must_use]
    fn reply_to(&self) -> Option<&types::Message> {
        self.context.reply_to()
    }

    #[must_use]
    fn author_signature(&self) -> Option<&str> {
        self.context.author_signature()
    }

    #[must_use]
    fn reply_markup(&self) -> Option<&inline_markup::Keyboard> {
        self.context.reply_markup()
    }
}

impl<C> EditedMessage<C> for Command<EditedText<C>> {
    #[must_use]
    fn edit_date(&self) -> i64 {
        self.context.edit_date()
    }
}

impl<C> fields::Text<C> for Command<EditedText<C>> {
    #[must_use]
    fn text(&self) -> &message::Text {
        &self.context.text
    }
}

impl<C> AnyText<C> for Command<EditedText<C>> {
    #[must_use]
    fn text(&self) -> &message::Text {
        &self.context.text
    }
}

impl<'a, C: 'static> Forwardable<'a, C> for Command<EditedText<C>> {}
impl<'a, C: 'static> Pinnable<'a, C> for Command<EditedText<C>> {}

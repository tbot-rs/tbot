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

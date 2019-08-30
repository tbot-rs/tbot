use super::MediaMessage;

/// A general trait for edited messages.
pub trait EditedMessage<C>: MediaMessage<C> {
    /// The last time when the message was edited.
    fn edit_date(&self) -> i64;
}

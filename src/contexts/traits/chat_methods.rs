use super::*;

/// Contains general methods which can infer some data from the context.
pub trait ChatMethods {
    // Wish trait fields came out soon
    /// Gets the bot.
    fn bot<'a>(&'a self) -> &'a MockBot;
    /// Gets the chat ID.
    fn chat_id(&self) -> i64;
    /// Gets the message ID.
    fn message_id(&self) -> u64;
    /// Gets the sender ID.
    fn from_id(&self) -> i64;

    /// Constructs a [`SendMessage`] inferring the token and the chat ID.
    ///
    /// [`SendMessage`]: ../methods/struct.SendMessage.html
    fn send_message<'a>(&'a self, text: &'a str) -> SendMessage<'a> {
        self.bot().send_message(self.chat_id(), text)
    }

    /// Constructs a [`SendMessage`] inferring the token, chat ID and the
    /// message ID.
    ///
    /// [`SendMessage`]: ../methods/struct.SendMessage.html
    fn send_message_in_reply<'a>(&'a self, text: &'a str) -> SendMessage<'a> {
        self.send_message(text).reply_to_message_id(self.message_id())
    }

    /// Constructs a [`SendLocation`] inferring the token and the chat ID.
    fn send_location<'a>(&'a self, location: (f64, f64)) -> SendLocation<'a> {
        self.bot().send_location(self.chat_id(), location)
    }
}

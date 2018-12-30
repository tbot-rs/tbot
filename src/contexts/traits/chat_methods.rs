use super::*;

/// Contains general methods which can infer some data from the context.
pub trait ChatMethods {
    // Wish trait fields came out soon
    /// Gets the bot.
    fn bot(&self) -> &MockBot;
    /// Gets the chat ID.
    fn chat_id(&self) -> i64;
    /// Gets the message ID.
    fn message_id(&self) -> u64;
    /// Gets the sender ID.
    fn from_id(&self) -> i64;

    /// Constructs a new [`ForwardMessage`] inferring the token and the
    /// destination chat ID.
    ///
    /// [`ForwardMessage`]: ../methods/struct.ForwardMessage.html
    fn forward_here<'a>(
        &'a self,
        from_chat_id: impl Into<types::ChatId<'a>>,
        message_id: u64,
    ) -> ForwardMessage<'a> {
        self.bot().forward_message(self.chat_id(), from_chat_id, message_id)
    }

    /// Constructs a [`GetUserProfilePhotos`] inferring the token and sender ID.
    ///
    /// [`GetUserProfilePhotos`]: ../methods/struct.GetUserProfilePhotos.html
    fn get_sender_profile_photos(&self) -> GetUserProfilePhotos {
        self.bot().get_user_profile_photos(self.from_id())
    }

    /// Constructs a [`SendChatAction`] inferring the token and the chat ID.
    ///
    /// [`SendChatAction`]: ../methods/struct.SendChatAction.html
    fn send_chat_action(&self, action: types::ChatAction) -> SendChatAction {
        self.bot().send_chat_action(self.chat_id(), action)
    }

    /// Construct a [`SendContact`] inferring the token and the chat ID.
    ///
    /// [`SendContact`]: ../methods/struct.SendContact.html
    fn send_contact<'a>(
        &'a self,
        phone_number: &'a str,
        first_name: &'a str,
    ) -> SendContact<'a> {
        self.bot().send_contact(self.chat_id(), phone_number, first_name)
    }

    /// Construct a [`SendContact`] inferring the token, chat ID and message ID.
    ///
    /// [`SendContact`]: ../methods/struct.SendContact.html
    fn send_contact_in_reply<'a>(
        &'a self,
        phone_number: &'a str,
        first_name: &'a str,
    ) -> SendContact<'a> {
        self.send_contact(phone_number, first_name)
            .reply_to_message_id(self.message_id())
    }

    /// Constructs a [`SendLocation`] inferring the token and the chat ID.
    ///
    /// [`SendLocation`]: ../methods/struct.SendLocation.html
    fn send_location(&self, location: (f64, f64)) -> SendLocation<'_> {
        self.bot().send_location(self.chat_id(), location)
    }

    /// Constructs a [`SendLocation`] inferring the token, chat ID and
    /// message ID.
    ///
    /// [`SendLocation`]: ../methods/struct.SendLocation.html
    fn send_location_in_reply(&self, location: (f64, f64)) -> SendLocation<'_> {
        self.send_location(location).reply_to_message_id(self.message_id())
    }

    /// Constructs a [`SendMessage`] inferring the token and the chat ID.
    ///
    /// [`SendMessage`]: ../methods/struct.SendMessage.html
    fn send_message<'a>(&'a self, text: &'a str) -> SendMessage<'a> {
        self.bot().send_message(self.chat_id(), text)
    }

    /// Constructs a [`SendMessage`] inferring the token, chat ID and
    /// message ID.
    ///
    /// [`SendMessage`]: ../methods/struct.SendMessage.html
    fn send_message_in_reply<'a>(&'a self, text: &'a str) -> SendMessage<'a> {
        self.send_message(text).reply_to_message_id(self.message_id())
    }

    /// Constructs a [`SendVenue`] inferring the token and the chat ID.
    ///
    /// [`SendVenue`]: ../methods/struct.SendLocation.html
    fn send_venue<'a>(
        &'a self,
        location: (f64, f64),
        title: &'a str,
        address: &'a str,
    ) -> SendVenue<'a> {
        self.bot().send_venue(self.chat_id(), location, title, address)
    }

    /// Constructs a [`SendVenue`] inferring the token, chat ID and
    /// message ID.
    ///
    /// [`SendVenue`]: ../methods/struct.SendLocation.html
    fn send_venue_in_reply<'a>(
        &'a self,
        location: (f64, f64),
        title: &'a str,
        address: &'a str,
    ) -> SendVenue<'a> {
        self.send_venue(location, title, address)
            .reply_to_message_id(self.message_id())
    }
}

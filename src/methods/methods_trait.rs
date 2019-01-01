use super::*;

/// Provides API methods that infer the bot's token.
pub trait Methods {
    /// Gets the token.
    fn token(&self) -> &str;

    /// Constructs a new [`GetMe`] inferring `token`.
    ///
    /// [`GetMe`]: ./struct.GetMe.html
    fn get_me(&self) -> methods::GetMe {
        methods::GetMe::new(self.token())
    }

    /// Constructs a new [`SendMessage`] inferring `token`.
    ///
    /// [`SendMessage`]: ./struct.SendMessage.html
    fn send_message<'a, 'b: 'a>(
        &'b self,
        chat_id: impl Into<types::ChatId<'a>>,
        text: &'a str,
    ) -> methods::SendMessage<'a> {
        methods::SendMessage::new(self.token(), chat_id, text)
    }

    /// Constructs a new [`SendPhoto`] inferring `token`.
    ///
    /// [`SendPhoto`]: ./struct.SendPhoto.html
    fn send_photo<'a, 'b: 'a>(
        &'b self,
        chat_id: impl Into<types::ChatId<'a>>,
        photo: types::Photo<'a>,
    ) -> methods::SendPhoto<'a> {
        methods::SendPhoto::new(self.token(), chat_id, photo)
    }

    /// Constructs a new [`SendAnimation`] inferring `token`.
    ///
    /// [`SendAnimation`]: ./struct.SendAnimation.html
    fn send_animation<'a, 'b: 'a>(
        &'b self,
        chat_id: impl Into<types::ChatId<'a>>,
        animation: types::Animation<'a>,
    ) -> methods::SendAnimation<'a> {
        methods::SendAnimation::new(self.token(), chat_id, animation)
    }

    /// Constructs a new [`ForwardMessage`] inferring `token`.
    ///
    /// [`ForwardMessage`]: ./struct.ForwardMessage.html
    fn forward_message<'a, 'b: 'a>(
        &'b self,
        chat_id: impl Into<types::ChatId<'a>>,
        from_chat_id: impl Into<types::ChatId<'a>>,
        message_id: u64,
    ) -> methods::ForwardMessage<'a> {
        methods::ForwardMessage::new(
            self.token(),
            chat_id,
            from_chat_id,
            message_id,
        )
    }

    /// Constructs a new [`SendLocation`] inferring `token`.
    ///
    /// [`SendLocation`]: ./struct.SendLocation.html
    fn send_location<'a, 'b: 'a>(
        &'b self,
        chat_id: impl Into<types::ChatId<'a>>,
        position: (f64, f64),
    ) -> methods::SendLocation<'a> {
        methods::SendLocation::new(self.token(), chat_id, position)
    }

    /// Constructs a new [`EditInlineLocation`] inferring `token`.
    ///
    /// [`EditInlineLocation`]: ./struct.EditInlineLocation.html
    fn edit_inline_location<'a, 'b: 'a>(
        &'b self,
        inline_message_id: u64,
        position: (f64, f64),
    ) -> methods::EditInlineLocation<'a> {
        methods::EditInlineLocation::new(
            self.token(),
            inline_message_id,
            position,
        )
    }

    /// Constructs a new [`EditMessageLocation`] inferring `token`.
    ///
    /// [`EditMessageLocation`]: ./struct.EditMessageLocation.html
    fn edit_message_location<'a, 'b: 'a>(
        &'b self,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u64,
        position: (f64, f64),
    ) -> methods::EditMessageLocation<'a> {
        methods::EditMessageLocation::new(
            self.token(),
            chat_id,
            message_id,
            position,
        )
    }

    /// Constructs a new [`StopInlineLocation`] inferring `token`.
    ///
    /// [`StopInlineLocation`]: ./struct.StopInlineLocation.html
    fn stop_inline_location<'a, 'b: 'a>(
        &'b self,
        inline_message_id: u64,
    ) -> methods::StopInlineLocation<'a> {
        methods::StopInlineLocation::new(self.token(), inline_message_id)
    }

    /// Constructs a new [`StopMessageLocation`] inferring `token`.
    ///
    /// [`StopMessageLocation`]: ./struct.StopMessageLocation.html
    fn stop_message_location<'a, 'b: 'a>(
        &'b self,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u64,
    ) -> methods::StopMessageLocation<'a> {
        methods::StopMessageLocation::new(self.token(), chat_id, message_id)
    }

    /// Constructs a new [`SendVenue`] inferring `token`.
    ///
    /// [`SendVenue`]: ./struct.SendVenue.html
    fn send_venue<'a, 'b: 'a>(
        &'b self,
        chat_id: impl Into<types::ChatId<'a>>,
        position: (f64, f64),
        title: &'a str,
        address: &'a str,
    ) -> methods::SendVenue<'a> {
        methods::SendVenue::new(self.token(), chat_id, position, title, address)
    }

    /// Constructs a new [`SendContact`] inferring `token`.
    ///
    /// [`SendContact`]: ./struct.SendContact.html
    fn send_contact<'a, 'b: 'a>(
        &'b self,
        chat_id: impl Into<types::ChatId<'a>>,
        phone_number: &'a str,
        first_name: &'a str,
    ) -> methods::SendContact<'a> {
        methods::SendContact::new(
            self.token(),
            chat_id,
            phone_number,
            first_name,
        )
    }

    /// Constructs a new [`SendChatAction`] inferring `token`.
    ///
    /// [`SendChatAction`]: ./struct.SendChatAction.html
    fn send_chat_action<'a, 'b: 'a>(
        &'b self,
        chat_id: impl Into<types::ChatId<'a>>,
        action: types::ChatAction,
    ) -> methods::SendChatAction<'a> {
        methods::SendChatAction::new(self.token(), chat_id, action)
    }

    /// Constructs a new [`GetUserProfilePhotos`] inferring `token`.
    ///
    /// [`GetUserProfilePhotos`]: ./struct.GetUserProfilePhotos.html
    fn get_user_profile_photos<'a, 'b: 'a>(
        &'b self,
        user_id: i64,
    ) -> methods::GetUserProfilePhotos<'a> {
        methods::GetUserProfilePhotos::new(self.token(), user_id)
    }
}

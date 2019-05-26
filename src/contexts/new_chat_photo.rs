message_base! {
    struct NewChatPhoto {
        /// The photo.
        photo: Vec<types::PhotoSize>,
    } -> Bot::new_chat_photo

    fn new(photo: Vec<types::PhotoSize>,) -> Self {
        Self {
            photo: photo,
        }
    }
}

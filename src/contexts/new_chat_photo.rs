message_base! {
    struct NewChatPhoto {
        /// The photo.
        photo: Vec<types::PhotoSize>,
    } -> EventLoop::new_chat_photo

    fn new(photo: Vec<types::PhotoSize>,) -> Self {
        Self {
            photo: photo,
        }
    }
}

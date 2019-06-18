use crate::types::PhotoSize;

message_base! {
    struct NewChatPhoto {
        /// The photo.
        photo: Vec<PhotoSize>,
    } -> EventLoop::new_chat_photo

    fn new(photo: Vec<PhotoSize>,) -> Self {
        Self {
            photo: photo,
        }
    }
}

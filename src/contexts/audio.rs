use crate::types::{self, message::Text};

media_message! {
    struct Audio {
        /// The audio.
        audio: types::Audio,
        /// The caption of the audio.
        caption: Text,
    } -> EventLoop::audio

    fn new(caption: Text,) -> Self {
        Self {
            caption: caption,
        }
    }
}

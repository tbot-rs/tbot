use crate::types::{Audio, message::Text};

edited_message! {
    struct EditedAudio {
        /// The audio.
        audio: Audio,
        /// The caption of the audio.
        caption: Text,
    } -> EventLoop::edited_audio

    fn new(caption: Text,) -> Self {
        Self {
            caption: caption,
        }
    }
}

use crate::types::{self, message::Text};

media_message! {
    struct Voice {
        /// The voice.
        voice: types::Voice,
        /// The caption of the voice.
        caption: Text,
    } -> EventLoop::voice

    fn new(caption: Text,) -> Self {
        Self {
            caption: caption,
        }
    }
}

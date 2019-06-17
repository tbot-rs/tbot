media_message! {
    struct Audio {
        /// The audio.
        audio: types::Audio,
        /// The caption of the audio.
        caption: types::message::Text,
    } -> Bot::audio

    fn new(caption: types::message::Text,) -> Self {
        Self {
            caption: caption,
        }
    }
}

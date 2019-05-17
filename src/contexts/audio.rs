media_message! {
    struct AudioContext {
        /// The audio.
        audio: types::Audio,
        /// The caption of the audio.
        caption: types::Text,
    } -> Bot::audio

    fn new(caption: types::Text,) -> Self {
        Self {
            caption: caption,
        }
    }
}

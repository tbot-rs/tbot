edited_message! {
    struct EditedAudio {
        /// The audio.
        audio: types::Audio,
        /// The caption of the audio.
        caption: types::Text,
    } -> Bot::edited_audio

    fn new(caption: types::Text,) -> Self {
        Self {
            caption: caption,
        }
    }
}

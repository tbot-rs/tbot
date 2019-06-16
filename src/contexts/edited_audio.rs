edited_message! {
    struct EditedAudio {
        /// The audio.
        audio: types::Audio,
        /// The caption of the audio.
        caption: types::message::Text,
    } -> Bot::edited_audio

    fn new(caption: types::message::Text,) -> Self {
        Self {
            caption: caption,
        }
    }
}

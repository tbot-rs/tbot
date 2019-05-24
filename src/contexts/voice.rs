media_message! {
    struct Voice {
        /// The voice.
        voice: types::Voice,
        /// The caption of the voice.
        caption: types::Text,
    } -> Bot::voice

    fn new(caption: types::Text,) -> Self {
        Self {
            caption: caption,
        }
    }
}

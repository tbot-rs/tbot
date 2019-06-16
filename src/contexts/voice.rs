media_message! {
    struct Voice {
        /// The voice.
        voice: types::Voice,
        /// The caption of the voice.
        caption: types::message::Text,
    } -> Bot::voice

    fn new(caption: types::message::Text,) -> Self {
        Self {
            caption: caption,
        }
    }
}

media_message! {
    struct VoiceContext {
        /// The voice.
        voice: types::Voice,
        /// The voice's caption.
        caption: String,
        /// Entities in the caption (links, formatting, etc).
        caption_entities: Vec<types::MessageEntity>,
    } -> Bot::voice

    fn new(caption: types::Text,) -> Self {
        Self {
            caption: caption.text,
            caption_entities: caption.entities,
        }
    }
}

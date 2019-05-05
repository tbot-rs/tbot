media_message! {
    struct AnimationContext {
        /// The animation.
        animation: types::Animation,
        /// The animation's caption.
        caption: String,
        /// Entities in the caption (links, formatting, etc).
        caption_entities: Vec<types::MessageEntity>,
    } -> Bot::animation

    fn new(caption: types::Text,) -> Self {
        Self {
            caption: caption.text,
            caption_entities: caption.entities,
        }
    }
}

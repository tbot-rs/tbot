media_message! {
    struct Animation {
        /// The animation.
        animation: types::Animation,
        /// The caption of the animation.
        caption: types::Text,
    } -> Bot::animation

    fn new(caption: types::Text,) -> Self {
        Self {
            caption: caption,
        }
    }
}

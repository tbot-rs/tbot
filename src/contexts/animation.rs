media_message! {
    struct AnimationContext {
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

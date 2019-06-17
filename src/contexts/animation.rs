media_message! {
    struct Animation {
        /// The animation.
        animation: types::Animation,
        /// The caption of the animation.
        caption: types::message::Text,
    } -> Bot::animation

    fn new(caption: types::message::Text,) -> Self {
        Self {
            caption: caption,
        }
    }
}

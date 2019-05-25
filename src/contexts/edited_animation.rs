edited_message! {
    struct EditedAnimation {
        /// The animation.
        animation: types::Animation,
        /// The caption of the animation.
        caption: types::Text,
    } -> Bot::edited_animation

    fn new(caption: types::Text,) -> Self {
        Self {
            caption: caption,
        }
    }
}

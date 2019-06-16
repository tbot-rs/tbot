edited_message! {
    struct EditedAnimation {
        /// The animation.
        animation: types::Animation,
        /// The caption of the animation.
        caption: types::message::Text,
    } -> Bot::edited_animation

    fn new(caption: types::message::Text,) -> Self {
        Self {
            caption: caption,
        }
    }
}

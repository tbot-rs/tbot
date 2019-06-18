use crate::types::{Animation, message::Text};

edited_message! {
    struct EditedAnimation {
        /// The animation.
        animation: Animation,
        /// The caption of the animation.
        caption: Text,
    } -> EventLoop::edited_animation

    fn new(caption: Text,) -> Self {
        Self {
            caption: caption,
        }
    }
}

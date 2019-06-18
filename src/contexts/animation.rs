use crate::types::{self, message::Text};

media_message! {
    struct Animation {
        /// The animation.
        animation: types::Animation,
        /// The caption of the animation.
        caption: Text,
    } -> EventLoop::animation

    fn new(caption: Text,) -> Self {
        Self {
            caption: caption,
        }
    }
}

use crate::{contexts::fields::{AnyText, Caption}, types::{message::Text, Animation}};

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

impl<C> Caption<C> for EditedAnimation<C> {
    fn caption(&self) -> &Text {
        &self.caption
    }
}

impl<C> AnyText<C> for EditedAnimation<C> {
    fn text(&self) -> &Text {
        &self.caption
    }
}


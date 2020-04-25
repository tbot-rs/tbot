use crate::{
    contexts::fields::{self, AnyText, Caption},
    types::{message::Text, Animation},
};

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

impl fields::Animation for EditedAnimation {
    #[must_use]
    fn animation(&self) -> &Animation {
        &self.animation
    }
}

impl Caption for EditedAnimation {
    #[must_use]
    fn caption(&self) -> &Text {
        &self.caption
    }
}

impl AnyText for EditedAnimation {
    #[must_use]
    fn text(&self) -> &Text {
        &self.caption
    }
}

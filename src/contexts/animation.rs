use crate::{
    contexts::fields::{self, AnyText, Caption},
    types::{self, message::Text},
};

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

impl fields::Animation for Animation {
    #[must_use]
    fn animation(&self) -> &types::Animation {
        &self.animation
    }
}

impl Caption for Animation {
    #[must_use]
    fn caption(&self) -> &Text {
        &self.caption
    }
}

impl AnyText for Animation {
    #[must_use]
    fn text(&self) -> &Text {
        &self.caption
    }
}

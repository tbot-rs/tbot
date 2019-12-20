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

impl<C> fields::Animation<C> for Animation<C> {
    #[must_use]
    fn animation(&self) -> &types::Animation {
        &self.animation
    }
}

impl<C> Caption<C> for Animation<C> {
    #[must_use]
    fn caption(&self) -> &Text {
        &self.caption
    }
}

impl<C> AnyText<C> for Animation<C> {
    #[must_use]
    fn text(&self) -> &Text {
        &self.caption
    }
}

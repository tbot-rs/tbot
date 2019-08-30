use crate::{
    contexts::fields::{AnyText, Caption},
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

impl<C> Caption<C> for Animation<C> {
    fn caption(&self) -> &Text {
        &self.caption
    }
}

impl<C> AnyText<C> for Animation<C> {
    fn text(&self) -> &Text {
        &self.caption
    }
}

use crate::{
    contexts::fields::{AnyText, Caption},
    types::{self, message::Text},
};

media_message! {
    struct Voice {
        /// The voice.
        voice: types::Voice,
        /// The caption of the voice.
        caption: Text,
    } -> EventLoop::voice

    fn new(caption: Text,) -> Self {
        Self {
            caption: caption,
        }
    }
}

impl<C> Caption<C> for Voice<C> {
    #[must_use]
    fn caption(&self) -> &Text {
        &self.caption
    }
}

impl<C> AnyText<C> for Voice<C> {
    #[must_use]
    fn text(&self) -> &Text {
        &self.caption
    }
}

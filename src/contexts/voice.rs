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

impl Caption for Voice {
    #[must_use]
    fn caption(&self) -> &Text {
        &self.caption
    }
}

impl AnyText for Voice {
    #[must_use]
    fn text(&self) -> &Text {
        &self.caption
    }
}

use crate::{contexts::fields::{AnyText, Caption}, types::{self, message::Text}};

media_message! {
    struct Audio {
        /// The audio.
        audio: types::Audio,
        /// The caption of the audio.
        caption: Text,
    } -> EventLoop::audio

    fn new(caption: Text,) -> Self {
        Self {
            caption: caption,
        }
    }
}

impl<C> Caption<C> for Audio<C> {
    fn caption(&self) -> &Text {
        &self.caption
    }
}

impl<C> AnyText<C> for Audio<C> {
    fn text(&self) -> &Text {
        &self.caption
    }
}

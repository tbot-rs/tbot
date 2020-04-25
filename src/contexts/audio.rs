use crate::{
    contexts::fields::{self, AnyText, Caption},
    types::{self, message::Text},
};

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

impl fields::Audio for Audio {
    #[must_use]
    fn audio(&self) -> &types::Audio {
        &self.audio
    }
}

impl Caption for Audio {
    #[must_use]
    fn caption(&self) -> &Text {
        &self.caption
    }
}

impl AnyText for Audio {
    #[must_use]
    fn text(&self) -> &Text {
        &self.caption
    }
}

use crate::{
    contexts::fields::{self, AnyText, Caption},
    types::{message::Text, Audio},
};

edited_message! {
    struct EditedAudio {
        /// The audio.
        audio: Audio,
        /// The caption of the audio.
        caption: Text,
    } -> EventLoop::edited_audio

    fn new(caption: Text,) -> Self {
        Self {
            caption: caption,
        }
    }
}

impl fields::Audio for EditedAudio {
    #[must_use]
    fn audio(&self) -> &Audio {
        &self.audio
    }
}

impl Caption for EditedAudio {
    #[must_use]
    fn caption(&self) -> &Text {
        &self.caption
    }
}

impl AnyText for EditedAudio {
    #[must_use]
    fn text(&self) -> &Text {
        &self.caption
    }
}

use crate::{contexts::fields::{AnyText, Caption}, types::{message::Text, Audio}};

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

impl<C> Caption<C> for EditedAudio<C> {
    fn caption(&self) -> &Text {
        &self.caption
    }
}

impl<C> AnyText<C> for EditedAudio<C> {
    fn text(&self) -> &Text {
        &self.caption
    }
}


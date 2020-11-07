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
        /// The media group's ID.
        media_group_id: Option<String>,
    } -> EventLoop::edited_audio

    fn new(caption: Text, media_group_id: Option<String>,) -> Self {
        Self {
            caption: caption,
            media_group_id: media_group_id,
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

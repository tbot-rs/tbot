use crate::{
    contexts::fields::{self, AnyText, Caption},
    types::{self, message::Text},
};

use super::fields::Album;

media_message! {
    struct Audio {
        /// The audio.
        audio: types::Audio,
        /// The caption of the audio.
        caption: Text,
        /// The media group's ID.
        media_group_id: Option<String>,
    } -> EventLoop::audio

    fn new(caption: Text, media_group_id: Option<String>,) -> Self {
        Self {
            caption: caption,
            media_group_id: media_group_id,
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

impl Album for Audio {
    #[must_use]
    fn media_group_id(&self) -> Option<&str> {
        self.media_group_id.as_ref().map(String::as_ref)
    }
}

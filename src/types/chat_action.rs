use super::*;

/// Represents possible chat actions.
#[derive(Serialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ChatAction {
    /// About to send a text message.
    Typing,
    /// About to send a photo.
    UploadPhoto,
    /// About to send a generated video.
    RecordVideo,
    /// About to send a video.
    UploadVideo,
    /// About to send an audio.
    RecordAudio,
    /// About to send a generated audio.
    UploadAudio,
    /// About to send a document.
    UploadDocument,
    /// About to send a location.
    FindLocation,
    /// About to send a generated video note.
    RecordVideoNote,
    /// About to send a video note.
    UploadVideoNote,
}

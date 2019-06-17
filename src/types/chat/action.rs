use serde::Serialize;

/// Represents possible chat actions.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
// todo: #[non_exhaustive]
pub enum Action {
    /// About to send a text message.
    Typing,
    /// About to send a photo.
    UploadPhoto,
    /// About to send a generated video.
    RecordVideo,
    /// About to send a video.
    UploadVideo,
    /// About to send a generated audio.
    RecordAudio,
    /// About to send an audio.
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

impl Action {
    /// Checks if `self` is `Typing`.
    pub fn is_typing(self) -> bool {
        self == Action::Typing
    }

    /// Checks if `self` is `UploadPhoto`.
    pub fn is_upload_photo(self) -> bool {
        self == Action::UploadPhoto
    }

    /// Checks if `self` is `RecordVideo`.
    pub fn is_record_video(self) -> bool {
        self == Action::RecordVideo
    }

    /// Checks if `self` is `UploadVideo`.
    pub fn is_upload_video(self) -> bool {
        self == Action::UploadVideo
    }

    /// Checks if `self` is `RecordAudio`.
    pub fn is_record_audio(self) -> bool {
        self == Action::RecordAudio
    }

    /// Checks if `self` is `UploadAudio`.
    pub fn is_upload_audio(self) -> bool {
        self == Action::UploadAudio
    }

    /// Checks if `self` is `UploadDocument`.
    pub fn is_upload_document(self) -> bool {
        self == Action::UploadDocument
    }

    /// Checks if `self` is `FindLocation`.
    pub fn is_find_location(self) -> bool {
        self == Action::FindLocation
    }

    /// Checks if `self` is `RecordVideoNote`.
    pub fn is_record_video_note(self) -> bool {
        self == Action::RecordVideoNote
    }

    /// Checks if `self` is `UploadVideoNote`.
    pub fn is_upload_video_note(self) -> bool {
        self == Action::UploadVideoNote
    }
}

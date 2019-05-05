media_message! {
    struct VideoNoteContext {
        /// The video note.
        video_note: types::VideoNote,
    } -> Bot::video_note

    fn new() -> Self {
        Self { }
    }
}

media_message! {
    struct VideoNote {
        /// The video note.
        video_note: types::VideoNote,
    } -> EventLoop::video_note

    fn new() -> Self {
        Self { }
    }
}

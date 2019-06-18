media_message! {
    struct Sticker {
        /// The sticker.
        sticker: types::Sticker,
    } -> EventLoop::sticker

    fn new() -> Self {
        Self { }
    }
}

media_message! {
    struct Sticker {
        /// The sticker.
        sticker: types::Sticker,
    } -> Bot::sticker

    fn new() -> Self {
        Self { }
    }
}

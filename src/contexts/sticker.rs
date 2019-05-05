media_message! {
    struct StickerContext {
        /// The sticker.
        sticker: types::Sticker,
    } -> Bot::sticker

    fn new() -> Self {
        Self { }
    }
}

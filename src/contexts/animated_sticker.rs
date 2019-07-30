use crate::types::sticker;

media_message! {
    struct AnimatedSticker {
        /// The animated sticker.
        sticker: sticker::Animated,
    } -> EventLoop::animated_sticker

    fn new() -> Self {
        Self { }
    }
}

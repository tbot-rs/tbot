use crate::types;

media_message! {
    struct Dice {
        /// The dice.
        dice: types::Dice,
    } -> EventLoop::dice

    fn new() -> Self {
        Self { }
    }
}

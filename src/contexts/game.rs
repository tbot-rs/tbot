use crate::types;

media_message! {
    struct Game {
        /// The game.
        game: types::Game,
    } -> EventLoop::game

    fn new() -> Self {
        Self { }
    }
}

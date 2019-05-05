media_message! {
    struct GameContext {
        /// The game.
        game: types::Game,
    } -> Bot::game

    fn new() -> Self {
        Self { }
    }
}

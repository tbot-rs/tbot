media_message! {
    struct Game {
        /// The game.
        game: types::Game,
    } -> Bot::game

    fn new() -> Self {
        Self { }
    }
}

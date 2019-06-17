media_message! {
    struct Invoice {
        /// The invoice.
        invoice: types::Invoice,
    } -> Bot::game

    fn new() -> Self {
        Self { }
    }
}

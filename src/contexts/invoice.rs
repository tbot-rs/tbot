media_message! {
    struct Invoice {
        /// The invoice.
        invoice: types::Invoice,
    } -> Bot::invoice

    fn new() -> Self {
        Self { }
    }
}

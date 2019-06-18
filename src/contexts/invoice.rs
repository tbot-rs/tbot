media_message! {
    struct Invoice {
        /// The invoice.
        invoice: types::Invoice,
    } -> EventLoop::invoice

    fn new() -> Self {
        Self { }
    }
}

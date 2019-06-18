media_message! {
    struct Contact {
        /// The contact.
        contact: types::Contact,
    } -> EventLoop::contact

    fn new() -> Self {
        Self { }
    }
}

media_message! {
    struct Contact {
        /// The contact.
        contact: types::Contact,
    } -> Bot::contact

    fn new() -> Self {
        Self { }
    }
}

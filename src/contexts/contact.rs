media_message! {
    struct ContactContext {
        /// The contact.
        contact: types::Contact,
    } -> Bot::contact

    fn new() -> Self {
        Self { }
    }
}

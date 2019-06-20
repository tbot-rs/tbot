media_message! {
    struct ConnectedWebsite {
        /// The connected website.
        website: String,
    } -> EventLoop::connected_website

    fn new() -> Self {
        Self { }
    }
}

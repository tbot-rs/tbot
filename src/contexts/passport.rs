use crate::types::passport;

media_message! {
    struct Passport {
        /// The passport data.
        passport_data: passport::Data,
    } -> EventLoop::passport

    fn new() -> Self {
        Self { }
    }
}

message_base! {
    struct CreatedGroup { } -> Bot::created_group

    fn new() -> Self {
        Self {}
    }
}

message_base! {
    struct CreatedGroup { } -> EventLoop::created_group

    fn new() -> Self {
        Self {}
    }
}

media_message! {
    struct Payment {
        /// Information about the payment.
        invoice: types::SuccessfulPayment,
    } -> EventLoop::payment

    fn new() -> Self {
        Self { }
    }
}

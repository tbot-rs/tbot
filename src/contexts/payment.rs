media_message! {
    struct Payment {
        /// Information about the payment.
        invoice: types::SuccessfulPayment,
    } -> Bot::payment

    fn new() -> Self {
        Self { }
    }
}

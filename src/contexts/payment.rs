use crate::types::SuccessfulPayment;

media_message! {
    struct Payment {
        /// Information about the payment.
        invoice: SuccessfulPayment,
    } -> EventLoop::payment

    fn new() -> Self {
        Self { }
    }
}

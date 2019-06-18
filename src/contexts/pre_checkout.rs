use crate::types::{User, pre_checkout_query, PreCheckoutQuery, OrderInfo};

common! {
    /// The context for [`pre_checkout`][handler] handlers.
    ///
    /// [handler]: ../event_loop/struct.EventLoop.html#method.pre_checkout
    struct PreCheckout {
        /// The ID of the query.
        id: pre_checkout_query::Id,
        /// The user who sent the query.
        from: User,
        /// The currency of of the invoice.
        currency: String,
        /// The total price.
        total_amount: u32,
        /// The invoice payload sent previously by the bot.
        invoice_payload: String,
        /// The ID of the chosen shipping option.
        shipping_option_id: Option<String>,
        /// The order information.
        order_info: Option<OrderInfo>,
    }
}

impl<C> PreCheckout<C> {
    // https://github.com/rust-lang/rust-clippy/issues/4041
    #[allow(clippy::missing_const_for_fn)]
    pub(crate) fn new(
        bot: Arc<Bot<C>>,
        query: PreCheckoutQuery,
    ) -> Self {
        Self {
            bot,
            id: query.id,
            from: query.from,
            currency: query.currency,
            total_amount: query.total_amount,
            invoice_payload: query.invoice_payload,
            shipping_option_id: query.shipping_option_id,
            order_info: query.order_info,
        }
    }
}

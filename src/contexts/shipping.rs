use crate::types::{User, shipping};

common! {
    /// The context for [`shipping`][handler] handlers.
    ///
    /// [handler]: ../event_loop/struct.EventLoop.html#method.shipping
    struct Shipping {
        /// The ID of the query.
        id: shipping::query::Id,
        /// The user who sent the query.
        from: User,
        /// The invoice payload sent previously by the bot.
        invoice_payload: String,
        /// The shipping address specified by the user.
        shipping_address: shipping::Address,
    }
}

impl<C> Shipping<C> {
    // https://github.com/rust-lang/rust-clippy/issues/4041
    #[allow(clippy::missing_const_for_fn)]
    pub(crate) fn new(
        bot: Arc<Bot<C>>,
        query: shipping::Query,
    ) -> Self {
        Self {
            bot,
            id: query.id,
            from: query.from,
            invoice_payload: query.invoice_payload,
            shipping_address: query.shipping_address,
        }
    }
}

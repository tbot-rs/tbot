use crate::{
    methods::AnswerShippingQuery,
    types::{shipping, User},
    Bot,
};
use std::borrow::Cow;

common! {
    /// The context for [`shipping`][handler] handlers.
    ///
    /// [handler]: ../event_loop/struct.EventLoop.html#method.shipping
    struct Shipping {
        /// The ID of the query.
        id: shipping::query::Id<'static>,
        /// The user who sent the query.
        from: User,
        /// The invoice payload sent previously by the bot.
        invoice_payload: String,
        /// The shipping address specified by the user.
        shipping_address: shipping::Address,
    }
}

impl Shipping {
    #[allow(clippy::missing_const_for_fn)]
    pub(crate) fn new(bot: Bot, query: shipping::Query) -> Self {
        Self {
            bot,
            id: query.id,
            from: query.from,
            invoice_payload: query.invoice_payload,
            shipping_address: query.shipping_address,
        }
    }

    /// Reports if shipping is possible.
    ///
    /// Note that this method suits better when you already deal with
    /// an `Option`. You might also want to use the [`ok`] and [`err`]
    /// methods from this context.
    ///
    /// [`ok`]: #method.ok
    /// [`err`]: #method.err
    pub fn answer<'a>(
        &'a self,
        result: Result<
            impl Into<Cow<'a, [shipping::Option<'a>]>>,
            impl Into<Cow<'a, str>>,
        >,
    ) -> AnswerShippingQuery<'a> {
        self.bot
            .answer_shipping_query(self.id.as_borrowed(), result)
    }

    /// Reports that shipping is possible and shows possible shipping options.
    pub fn ok<'a>(
        &'a self,
        options: impl Into<Cow<'a, [shipping::Option<'a>]>>,
    ) -> AnswerShippingQuery<'a> {
        let answer: Result<_, String> = Ok(options);
        self.answer(answer)
    }

    /// Reports that shipping is impossible and shows the error message.
    pub fn err<'a>(
        &'a self,
        err: impl Into<Cow<'a, str>>,
    ) -> AnswerShippingQuery<'a> {
        let answer: Result<Vec<_>, _> = Err(err);
        self.answer(answer)
    }
}

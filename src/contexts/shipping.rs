use crate::{
    methods::AnswerShippingQuery,
    types::{shipping, User},
    Bot,
};

common! {
    /// The context for [`shipping`] handlers.
    ///
    /// [`shipping`]: crate::EventLoop::shipping
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
    /// [`ok`]: Self::ok
    /// [`err`]: Self::err
    pub fn answer(
        &self,
        result: Result<impl Into<Vec<shipping::Option>>, impl Into<String>>,
    ) -> AnswerShippingQuery<'_> {
        self.bot.answer_shipping_query(self.id.clone(), result)
    }

    /// Reports that shipping is possible and shows possible shipping options.
    pub fn ok(
        &self,
        options: impl Into<Vec<shipping::Option>>,
    ) -> AnswerShippingQuery<'_> {
        let answer: Result<_, String> = Ok(options);
        self.answer(answer)
    }

    /// Reports that shipping is impossible and shows the error message.
    pub fn err(&self, err: impl Into<String>) -> AnswerShippingQuery<'_> {
        let answer: Result<Vec<_>, _> = Err(err);
        self.answer(answer)
    }
}

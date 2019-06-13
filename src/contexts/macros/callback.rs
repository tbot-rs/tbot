macro_rules! callback {
    (
        struct $name:ident {
            #[doc = $kind_doc:literal] $kind:ident: $kind_type:ty,
        } -> Bot::$handler:ident
    ) => {
        use types::*;

        common! {
            #[doc = concat!(
                "Context for the [`", stringify!($handler), "`][handler] handler.\n\n",
                "[handler]: ../struct.Bot.html#method.", stringify!($handler),
            )]
            struct $name {
                /// The ID of the callback.
                id: String,
                /// The user who initiated the callback.
                from: User,
                /// The origin of the query.
                origin: CallbackOrigin,
                /// The identifier of the chat.
                chat_instance: String,
                #[doc = $kind_doc]
                $kind: $kind_type,
            }
        }

        impl<C> $name<C> {
            // https://github.com/rust-lang/rust-clippy/issues/4041
            #[allow(clippy::missing_const_for_fn)]
            pub(crate) fn new(
                bot: Arc<Bot<C>>,
                id: String,
                from: User,
                origin: CallbackOrigin,
                chat_instance: String,
                $kind: $kind_type,
            ) -> Self {
                Self {
                    bot,
                    id,
                    from,
                    origin,
                    chat_instance,
                    $kind,
                }
            }
        }

        impl<'a, C: 'static> traits::Callback<'a, C> for $name<C> {
            fn bot(&self) -> &Bot<C> {
                &self.bot
            }

            fn id(&self) -> &str {
                &self.id
            }
        }
    }
}

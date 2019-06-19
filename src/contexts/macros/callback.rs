macro_rules! callback {
    (
        struct $name:ident {
            #[doc = $kind_doc:literal] $kind:ident: $kind_type:ty,
        } -> EventLoop::$handler:ident
    ) => {
        common! {
            #[doc = concat!(
                "Context for the [`", stringify!($handler), "`][handler] ", "handler.\n\n",

                "[handler]: ../event_loop/struct.Event.html#method.",
                stringify!($handler),
            )]
            struct $name {
                /// The ID of the callback.
                id: crate::types::callback::query::Id,
                /// The user who initiated the callback.
                from: crate::types::User,
                /// The origin of the query.
                origin: crate::types::callback::Origin,
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
                bot: std::sync::Arc<crate::Bot<C>>,
                id: crate::types::callback::query::Id,
                from: crate::types::User,
                origin: crate::types::callback::Origin,
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

        impl<'a, C: 'static> super::traits::Callback<'a, C> for $name<C> {
            fn bot(&self) -> &crate::Bot<C> {
                &self.bot
            }

            fn id(&self) -> crate::types::callback::query::id::Ref<'_> {
                self.id.as_ref()
            }
        }
    }
}

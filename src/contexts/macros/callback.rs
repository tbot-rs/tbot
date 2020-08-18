macro_rules! callback {
    (
        struct $name:ident {
            #[doc = $kind_doc:literal] $kind:ident: $kind_type:ty,
            $origin:ident: $origin_type:ty,
        } -> EventLoop::$handler:ident
    ) => {
        common! {
            #[doc = concat!(
                "Context for the [`", stringify!($handler), "`][handler] ", "handler.\n\n",

                "[handler]: ../event_loop/struct.EventLoop.html#method.",
                stringify!($handler),
            )]
            struct $name {
                /// The ID of the callback.
                id: crate::types::callback::query::Id<'static>,
                /// The user who initiated the callback.
                from: crate::types::User,
                /// The origin of the query.
                $origin: $origin_type,
                /// The identifier of the chat.
                chat_instance: String,
                #[doc = $kind_doc]
                $kind: $kind_type,
            }
        }

        impl $name {
            #[allow(clippy::missing_const_for_fn)]
            pub(crate) fn new(
                bot: crate::Bot,
                id: crate::types::callback::query::Id<'static>,
                from: crate::types::User,
                $origin: $origin_type,
                chat_instance: String,
                $kind: $kind_type,
            ) -> Self {
                Self {
                    bot,
                    id,
                    from,
                    $origin,
                    chat_instance,
                    $kind,
                }
            }
        }

        impl crate::contexts::fields::Callback for $name {
            #[must_use]
            fn id(&self) -> &crate::types::callback::query::Id {
                &self.id
            }

            #[must_use]
            fn from(&self) -> &crate::types::User {
                &self.from
            }

            #[must_use]
            fn chat_instance(&self) -> &str {
                &self.chat_instance
            }
        }
    }
}

macro_rules! message_base {
    (
        struct $name:ident {
            $(#[doc = $field_doc:literal] $field:ident: $type:ty,)*
        } -> EventLoop::$handler:ident

        fn new(
            $($param:ident: $param_type:ty,)*
        ) -> Self {
            $(infer $infer:ident;)*

            Self {
                $($new_field:ident: $value:expr,)*
            }
        }
    ) => {
        common! {
            #[doc = concat!(
                "The context for [`", stringify!($handler), "`][handler] ",
                "handlers.\n\n",
                "[handler]: ../event_loop/struct.EventLoop.html#method.",
                stringify!($handler),
            )]
            struct $name {
                /// ID of the message.
                message_id: crate::types::message::Id,
                /// The author of the message.
                from: Option<crate::types::User>,
                /// The timestamp of the message.
                date: i64,
                /// The chat to which the message was sent.
                chat: crate::types::Chat,
                $(#[doc = $field_doc] $field: $type,)*
            }
        }

        impl<C> $name<C> {
            // https://github.com/rust-lang/rust-clippy/issues/4041
            #[allow(clippy::missing_const_for_fn)]
            pub(crate) fn new(
                bot: std::sync::Arc<crate::Bot<C>>,
                data: crate::types::message::Data,
                $($param: $param_type,)*
            ) -> Self {
                Self {
                    bot,
                    message_id: data.id,
                    from: data.from,
                    date: data.date,
                    chat: data.chat,
                    $($new_field: $value,)*
                    $($infer: data.$infer,)*
                }
            }
        }

        impl<'a, C: 'static> super::traits::ChatMethods<'a, C> for $name<C> {
            fn bot(&self) -> &crate::Bot<C> {
                &self.bot
            }

            fn chat_id(&self) -> crate::types::chat::Id {
                self.chat.id
            }

            fn message_id(&self) -> crate::types::message::Id {
                self.message_id
            }
        }
    }
}

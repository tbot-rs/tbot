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
                from: Option<crate::types::message::From>,
                /// The timestamp of the message.
                date: i64,
                /// The chat to which the message was sent.
                chat: crate::types::Chat,
                $(#[doc = $field_doc] $field: $type,)*
            }
        }

        impl $name {
            #[allow(clippy::redundant_field_names, clippy::missing_const_for_fn)]
            pub(crate) fn new(
                bot: crate::Bot,
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

        impl crate::contexts::fields::Message for $name {
            #[must_use]
            fn message_id(&self) -> crate::types::message::Id {
                self.message_id
            }

            #[must_use]
            fn from(&self) -> Option<&crate::types::message::From> {
                self.from.as_ref()
            }

            #[must_use]
            fn date(&self) -> i64 {
                self.date
            }

            #[must_use]
            fn chat(&self) -> &crate::types::Chat {
                &self.chat
            }
        }
    }
}

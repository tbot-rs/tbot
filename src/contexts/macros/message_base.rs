macro_rules! message_base {
    (
        struct $name:ident {
            $(#[doc = $field_doc:literal] $field:ident: $type:ty,)*
        } -> Bot::$handler:ident

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
                "Context for the [`", stringify!($handler), "`][handler] handler.\n\n",
                "[handler]: ../struct.Bot.html#method.", stringify!($handler),
            )]
            struct $name {
                /// Id of the message.
                message_id: u32,
                /// The sender of the message.
                from: Option<types::User>,
                /// The time the message was sent at.
                date: i64,
                /// The chat where the message was sent.
                chat: types::raw::Chat,
                $(#[doc = $field_doc] $field: $type,)*
            }
        }

        impl $name {
            // https://github.com/rust-lang/rust-clippy/issues/4041
            #[allow(clippy::missing_const_for_fn)]
            pub(crate) fn new(
                bot: Arc<MockBot>,
                data: types::MessageData,
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

        impl<'a> traits::ChatMethods<'a> for $name {
            fn bot(&self) -> &MockBot {
                &self.bot
            }

            fn chat_id(&self) -> i64 {
                self.chat.id
            }

            fn message_id(&self) -> u32 {
                self.message_id
            }
        }
    }
}

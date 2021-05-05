macro_rules! edited_message {
    (
        struct $name:ident {
            #[doc = $media_doc:literal] $media:ident: $media_type:ty,
            $(#[doc = $field_doc:literal] $field:ident: $type:ty,)*
        } -> EventLoop::$handler:ident

        fn new(
            $($param:ident: $param_type:ty,)*
        ) -> Self {
            Self {
                $($new_field:ident: $value:expr,)*
            }
        }
    ) => {
        message_base! {
            struct $name {
                /// The replied message.
                reply_to: Option<crate::types::Message>,
                /// The author's signature, if enabled for the channel.
                author_signature: Option<String>,
                /// The last time when the message was edited.
                edit_date: i64,
                /// The inline keyboard attached to the message.
                reply_markup: Option<crate::types::keyboard::inline::Keyboard>,
                /// The bot via which the message was sent.
                via_bot: Option<crate::types::User>,
                #[doc = $media_doc]
                $media: $media_type,
                $(#[doc = $field_doc] $field: $type,)*
            } -> EventLoop::$handler

            fn new(
                edit_date: i64,
                $media: $media_type,
                $($param: $param_type,)*
            ) -> Self {
                infer reply_to;
                infer author_signature;
                infer reply_markup;
                infer via_bot;

                Self {
                    edit_date: edit_date,
                    $media: $media,
                    $($new_field: $value,)*
                }
            }
        }

        impl super::methods::Copyable for $name {}
        impl super::methods::Forwardable for $name {}
        impl super::methods::Pinnable for $name {}

        impl crate::contexts::fields::MediaMessage for $name {
            #[must_use]
            fn reply_to(&self) -> Option<&crate::types::Message> {
                self.reply_to.as_ref()
            }

            #[must_use]
            fn author_signature(&self) -> Option<&str> {
                self.author_signature.as_ref().map(String::as_str)
            }

            #[must_use]
            fn reply_markup(
                &self
            ) -> Option<&crate::types::keyboard::inline::Keyboard> {
                self.reply_markup.as_ref()
            }

            #[must_use]
            fn via_bot(&self) -> Option<&crate::types::User> {
                self.via_bot.as_ref()
            }
        }

        impl crate::contexts::fields::EditedMessage for $name {
            #[must_use]
            fn edit_date(&self) -> i64 {
                self.edit_date
            }
        }
    };
}

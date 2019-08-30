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
                reply_markup: Option<crate::types::message::inline_markup::Keyboard>,
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

                Self {
                    edit_date: edit_date,
                    $media: $media,
                    $($new_field: $value,)*
                }
            }
        }

        impl<'a, C: 'static> super::traits::Forwardable<'a, C> for $name<C> {}
        impl<'a, C: 'static> super::traits::Pinnable<'a, C> for $name<C> {}

        impl<C> crate::contexts::fields::MediaMessage<C> for $name<C> {
            fn reply_to(&self) -> Option<&crate::types::Message> {
                self.reply_to.as_ref()
            }

            fn author_signature(&self) -> Option<&str> {
                self.author_signature.as_ref().map(String::as_str)
            }

            fn reply_markup(
                &self
            ) -> Option<&crate::types::message::inline_markup::Keyboard> {
                self.reply_markup.as_ref()
            }
        }

        impl<C> crate::contexts::fields::EditedMessage<C> for $name<C> {
            fn edit_date(&self) -> i64 {
                self.edit_date
            }
        }
    };
}

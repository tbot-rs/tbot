macro_rules! media_message {
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
                reply_to: Option<types::Message>,
                /// The author's signature, if enabled for the channel.
                author_signature: Option<String>,
                /// The origin of the message if it's a forward.
                forward: Option<types::message::Forward>,
                #[doc = $media_doc]
                $media: $media_type,
                $(#[doc = $field_doc] $field: $type,)*
            } -> EventLoop::$handler

            fn new(
                $media: $media_type,
                $($param: $param_type,)*
            ) -> Self {
                infer reply_to;
                infer author_signature;
                infer forward;

                Self {
                    $media: $media,
                    $($new_field: $value,)*
                }
            }
        }

        impl<'a, C: 'static> Forwardable<'a, C> for $name<C> {}
        impl<'a, C: 'static> Pinnable<'a, C> for $name<C> {}
    };
}

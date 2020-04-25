macro_rules! common {
    (
        $(#[doc = $doc:expr])+
        struct $name:ident {
            $(#[doc = $field_doc:literal] $field:ident: $type:ty,)+
        }
    ) => {
        $(#[doc = $doc])+
        #[derive(Debug, Clone)]
        #[non_exhaustive]
        pub struct $name {
            /// A bot for calling API without information inference.
            pub bot: std::sync::Arc<crate::Bot>,
            $(#[doc = $field_doc] pub $field: $type,)+
        }

        impl crate::internal::Sealed for $name { }

        impl crate::contexts::fields::Context for $name {
            fn bot(&self) -> &crate::Bot {
                &self.bot
            }
        }
    }
}

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
        #[must_use]
        pub struct $name<C> {
            /// A bot for calling API without information inference.
            pub bot: std::sync::Arc<crate::Bot<C>>,
            $(#[doc = $field_doc] pub $field: $type,)+
        }

        impl<C> crate::internal::Sealed for $name<C> { }

        impl<C> crate::contexts::fields::Context<C> for $name<C> {
            fn bot(&self) -> &crate::Bot<C> {
                &self.bot
            }
        }
    }
}

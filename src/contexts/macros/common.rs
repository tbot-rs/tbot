macro_rules! common {
    (
        $(#[doc = $doc:expr])+
        struct $name:ident {
            $(#[doc = $field_doc:literal] $field:ident: $type:ty,)+
        }
    ) => {
        $(#[doc = $doc])+
        #[derive(Debug, Clone)]
        // todo: #[non_exhaustive]
        pub struct $name<C> {
            /// A bot for calling API without information inference.
            pub bot: std::sync::Arc<crate::Bot<C>>,
            $(#[doc = $field_doc] pub $field: $type,)+
        }

        impl<C> crate::internal::Sealed for $name<C> { }
    }
}

macro_rules! common {
    (
        $(#[doc = $doc:expr])+
        struct $name:ident {
            $(#[doc = $field_doc:literal] $field:ident: $type:ty,)+
        }
    ) => {
        use super::*;
        use std::sync::Arc;

        $(#[doc = $doc])+
        #[derive(Clone)]
        // todo: #[non_exhaustive]
        pub struct $name<C> {
            /// A mock bot for calling API without information inference.
            pub bot: Arc<Bot<C>>,
            $(#[doc = $field_doc] pub $field: $type,)+
        }

        impl<C> crate::internal::Sealed for $name<C> { }
    }
}

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
        pub struct $name {
            /// A mock bot for calling API without information inference.
            pub bot: Arc<Bot>,
            $(#[doc = $field_doc] pub $field: $type,)+
        }

        impl crate::Sealed for $name { }
    }
}

use crate::{internal::Sealed, Bot};

/// A general trait for all contexts.
pub trait Context: Sealed {
    /// A bot for calling API without information inference.
    fn bot(&self) -> &Bot;
}

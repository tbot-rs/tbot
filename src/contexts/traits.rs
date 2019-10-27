//! Contains traits for updates that infer some data from them, simplifying
//! calling several methods.

mod callback;
mod chat_methods;
mod forwardable;
mod pinnable;

pub use {
    callback::Callback, chat_methods::ChatMethods, forwardable::Forwardable,
    pinnable::Pinnable,
};

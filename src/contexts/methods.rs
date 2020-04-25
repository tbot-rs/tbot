//! Traits for calling methods inferring as much data as possible from
//! the context.

mod callback;
mod chat_methods;
mod forwardable;
mod pinnable;

pub use {
    callback::Callback, chat_methods::ChatMethods, forwardable::Forwardable,
    pinnable::Pinnable,
};

//! Traits for calling methods inferring as much data as possible from
//! the context.

mod callback;
mod copyable;
mod forwardable;
mod message;
mod pinnable;

pub use {
    callback::Callback, copyable::Copyable, forwardable::Forwardable,
    message::Message, pinnable::Pinnable,
};

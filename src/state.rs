//! The stateful event loop and utilities for it.

pub mod chats;
mod event_loop;

pub use chats::Chats;
pub use event_loop::StatefulEventLoop;

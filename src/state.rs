//! The stateful event loop and utilities for it.
//!
//! The stateful event loop makes it easier to write bots that depend on some
//! kind of state. It can be a chatting bot, a quiz bot or any other bot that
//! needs to store data. The stateful event loop also can be used to easily
//! share some data between all handlers, e.g. if you use a database that
//! stores all your data, you can implement a utility struct with methods that
//! simplify access to the database, and then share it across all your handlers.
//!
//! For example, let's think of a bot with a global counter, and the bot allows
//! increasing and showing it. If we'd have gone with a stateless event loop,
//! we'd start with this:
//!
//! ```
//! let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();
//! ```
//!
//! To use the stateful event loop, we simply call [`stateful_event_loop`]
//! instead of [`event_loop`]:
//!
//! ```
//! # let initial_state = ();
//! let mut bot = tbot::from_env!("BOT_TOKEN")
//!     .stateful_event_loop(initial_state);
//! ```
//!
//! [`stateful_event_loop`]: crate::Bot::stateful_event_loop
//! [`event_loop`]: crate::Bot::event_loop
//!
//! What should `initial_state` be? It can actually be any value (that can be
//! shared across threads). You'd think that, as we only have a counter,
//! we would simply use any integer type. However, `tbot` won't let you mutate
//! the state — instead, you should use interior mutability. This design
//! decision was made to avoid mutability if it isn't needed and for users
//! to decide what parts of their state should be mutable — that is to prevent
//! all the state being locked when only a part of it is actually needs to be
//! locked. So what we need is an integer wrapped in an [`RwLock`]:
//!
//! ```
//! # use std::sync::RwLock; /*
//! use tokio::sync::RwLock;
//! # */
//!
//! let mut bot = tbot::from_env!("BOT_TOKEN")
//!     .stateful_event_loop(RwLock::new(0));
//! ```
//!
//! [`RwLock`]: tokio::sync::RwLock
//!
//! Now, if we would have gone with the stateless event loop, we'd write this:
//!
//! ```compile_fail
//! # let mut bot = tbot::Bot::new(String::new()).stateful_event_loop(());
//! bot.command("increase", |context| async move { /* .. */ });
//! ```
//!
//! Once we opt in to the stateful event loop, we need to write this:
//!
//! ```
//! # let mut bot = tbot::Bot::new(String::new()).stateful_event_loop(());
//! bot.command("increase", |context, state| async move { /* .. */ });
//! ```
//!
//! The state is passed being wrapped in an `Arc`, that is, this handler
//! receives [`Arc`]`<`[`RwLock`]`<i32>>` as the second argument. This allows
//! parallel access to the state. Now you only need to use the state:
//!
//! ```
//! # struct RwLock(std::sync::RwLock<i32>);
//! # impl RwLock {
//! #     async fn write(&self) -> std::sync::RwLockWriteGuard<'_, i32> {
//! #         self.0.write().unwrap()
//! #     }
//! # }
//! # let mut bot = tbot::Bot::new(String::new())
//! #     .stateful_event_loop(RwLock(std::sync::RwLock::new(0)));
//! use tbot::prelude::*;
//!
//! bot.command("increase", |context, state| async move {
//!     *state.write().await += 1;
//!     let call_result =
//!         context.send_message("Increased the counter").call().await;
//!
//!     if let Err(err) = call_result {
//!         dbg!(err);
//!     }
//! });
//! ```
//!
//! [`Arc`]: std::sync::Arc
//!
//! `tbot` also provides a few utility state storages for common patterns.
//! You can combine them with other state storages or with your own storage
//! if needed.

pub mod chats;
mod event_loop;
pub mod messages;
mod polling;

pub use chats::Chats;
pub use event_loop::StatefulEventLoop;
pub use messages::Messages;
pub use polling::Polling;

//! Types related to callbacks.

mod game;
pub mod query;

pub use {
    game::*,
    query::{Kind, Origin, Query},
};

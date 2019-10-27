//! Types related to callbacks.

mod game;
pub mod query;

pub use {
    game::Game,
    query::{Kind, Origin, Query},
};

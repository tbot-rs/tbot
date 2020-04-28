//! A storage of state per chat.
//!
//! The [`Chats`] storage can be used to store some state for each chat
//! separately. An example of it is a questionary bot when the bot collects
//! some data from the user step-by-step:
//!
//! ```
//! enum Questionary {
//!     AskName,
//!     AskIfPraisedTheBorrowChecker {
//!         name: String,
//!     },
//!     Done {
//!         name: String,
//!         has_praised_the_borrow_checker: bool,
//!     },
//! }
//! ```
//!
//! [`Chats`] stores its states in a hash map where the key is the chat ID and
//! the value is the state for the chat. As explained in docs for [`state`],
//! if you want to mutate state, you need to manually wrap it in a lock,
//! and this applies to [`Chats`] as well. Let's wrap our state in a [`Mutex`]:
//!
//! ```
//! use tbot::state::Chats;
//! # use std::sync::Mutex; /*
//! use tokio::sync::Mutex;
//! # */
//!
//! let mut bot = tbot::from_env!("BOT_TOKEN")
//!     .stateful_event_loop(Mutex::new(Chats::new()));
//! # let _: std::sync::Arc<Mutex<Chats<()>>> = bot.get_state();
//! ```
//!
//! [`Chats`]: ./struct.Chats.html
//! [`state`]: ../index.html
//! [`Mutex`]: https://docs.rs/tokio/0.2.*/tokio/sync/struct.Mutex.html
//!
//! Let's start our questionary once the user starts the bot:
//!
//! ```
//! # use {std::sync, tbot::state::Chats};
//! # enum Questionary { AskName };
//! # struct Mutex(sync::Mutex<Chats<Questionary>>);
//! # impl Mutex {
//! #     async fn lock(&self) -> sync::MutexGuard<'_, Chats<Questionary>> {
//! #         self.0.lock().unwrap()
//! #     }
//! # }
//! # let mut bot = tbot::Bot::new(String::new())
//! #     .stateful_event_loop(Mutex(sync::Mutex::new(Chats::new())));
//! use tbot::prelude::*;
//!
//! bot.start(|context, state| async move {
//!     state.lock().await.insert(&*context, Questionary::AskName);
//!     let call_result =
//!         context.send_message("Hello! What's your name?").call().await;
//!
//!     if let Err(err) = call_result {
//!         dbg!(err);
//!     }
//! });
//! ```
//!
//!
//! You can see that the [`insert`] method can figure out the chat ID from
//! the context, but there's still [`insert_by_id`] if you need it. In fact,
//! [`Chats`]'s API is very similar to the API of `std`'s [`HashMap`],
//! but instead of the key you need to provide the context or use the equivalent
//! method with the `_by_id` postfix.
//!
//! [`insert`]: ./struct.Chats.html#method.insert
//! [`insert_by_id`]: ./struct.Chats.html#method.insert_by_id
//! [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
//!
//! If you need to, you can combine [`Chats`] with other state stores like this:
//!
//! ```
//! # #[derive(Default)]
//! # struct SomeOtherState;
//! # use std::sync::RwLock;
//! use tbot::state::Chats;
//! # /*
//! use tokio::sync::RwLock;
//! # */
//!
//! #[derive(Default)]
//! struct State {
//!     chats: RwLock<Chats<String>>,
//!     some_other_state: SomeOtherState,
//! }
//!
//! let mut bot = tbot::from_env!("BOT_TOKEN")
//!     .stateful_event_loop(State::default());
//! ```

use crate::{contexts::fields::Message, types::chat};
use serde::{Deserialize, Serialize};
use std::{
    collections::hash_map::{self, Entry, HashMap, IntoIter},
    iter::FromIterator,
    ops::Index,
};

/// A storage of state per chat. See [the module's docs] to learn how to use it.
///
/// [the module's docs]: ./index.html
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Chats<S> {
    chats: HashMap<chat::Id, S>,
}

impl<S> Chats<S> {
    /// Constructs a new chat storage.
    #[must_use]
    pub fn new() -> Self {
        Self {
            chats: HashMap::new(),
        }
    }

    /// Constructs a new chat storage with capacity for `n` chats.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            chats: HashMap::with_capacity(capacity),
        }
    }

    /// Returns an iterator over the stored chat IDs.
    pub fn chats(&self) -> impl Iterator<Item = chat::Id> + '_ {
        self.chats.keys().copied()
    }

    /// Returns an iterator over the stored states.
    pub fn states(&self) -> impl Iterator<Item = &S> {
        self.chats.values()
    }

    /// Returns an iterator over the stored chat IDs and their states.
    pub fn iter(&self) -> impl Iterator<Item = (chat::Id, &S)> {
        Iter(self.chats.iter())
    }

    /// Returns a mutable iterator over the stored chat IDs and their states.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (chat::Id, &mut S)> {
        IterMut(self.chats.iter_mut())
    }

    /// Returns how many chats are stored.
    #[must_use]
    pub fn len(&self) -> usize {
        self.chats.len()
    }

    /// Returns the storage's capacity.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.chats.capacity()
    }

    /// Returns `true` if the store is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.chats.is_empty()
    }

    /// Clears the store, returning each stored item in an iterator.
    #[must_use = "use `clear` if you don't need the iterator"]
    pub fn drain(&mut self) -> impl Iterator<Item = (chat::Id, S)> + '_ {
        self.chats.drain()
    }

    /// Clears the store.
    pub fn clear(&mut self) {
        self.chats.clear()
    }

    /// Reserves capacity for `n` additional chats.
    pub fn reserve(&mut self, additional: usize) {
        self.chats.reserve(additional)
    }

    /// Shrinks the storage to already stored chats.
    pub fn shrink_to_fit(&mut self) {
        self.chats.shrink_to_fit()
    }

    /// Gets a chat's state by its ID.
    #[must_use]
    pub fn get_by_id(&self, id: chat::Id) -> Option<&S> {
        self.chats.get(&id)
    }

    /// Gets a chat's state, inferring its ID from the context.
    #[must_use]
    pub fn get<C>(&self, context: &C) -> Option<&S>
    where
        C: Message,
    {
        self.get_by_id(context.chat().id)
    }

    /// Gets a mutable reference to a chat's state by its ID.
    #[must_use]
    pub fn get_mut_by_id(&mut self, id: chat::Id) -> Option<&mut S> {
        self.chats.get_mut(&id)
    }

    /// Gets a mutable reference to a chat's state, inferring its ID
    /// from the context.
    #[must_use]
    pub fn get_mut<C>(&mut self, context: &C) -> Option<&mut S>
    where
        C: Message,
    {
        self.get_mut_by_id(context.chat().id)
    }

    /// Gets an entry for a chat's state by its ID.
    #[must_use]
    pub fn entry_by_id(&mut self, id: chat::Id) -> Entry<chat::Id, S> {
        self.chats.entry(id)
    }

    /// Gets an entry for a chat's state, inferring its ID from the context.
    #[must_use]
    pub fn entry<C>(&mut self, context: &C) -> Entry<chat::Id, S>
    where
        C: Message,
    {
        self.entry_by_id(context.chat().id)
    }

    /// Checks if there's state for a chat by its ID.
    #[must_use]
    pub fn has_by_id(&self, id: chat::Id) -> bool {
        self.chats.contains_key(&id)
    }

    /// Checks if there's state for a chat, inferring its ID from the context.
    #[must_use]
    pub fn has<C>(&self, context: &C) -> bool
    where
        C: Message,
    {
        self.has_by_id(context.chat().id)
    }

    /// Inserts state for a chat by its ID. Returns the previous state.
    pub fn insert_by_id(&mut self, id: chat::Id, value: S) -> Option<S> {
        self.chats.insert(id, value)
    }

    /// Inserts state for a chat, inferring its ID from the context.
    /// Returns the previous state.
    pub fn insert<C>(&mut self, context: &C, value: S) -> Option<S>
    where
        C: Message,
    {
        self.insert_by_id(context.chat().id, value)
    }

    /// Removes and returns a chat's state by its ID.
    pub fn remove_by_id(&mut self, id: chat::Id) -> Option<S> {
        self.chats.remove(&id)
    }

    /// Removes and returns a chat's state, inferring its ID from the context.
    pub fn remove<C>(&mut self, context: &C) -> Option<S>
    where
        C: Message,
    {
        self.remove_by_id(context.chat().id)
    }

    /// Calls the predicate for each stored entry and deletes entries for which
    /// the predicate returns `false`.
    pub fn retain<P>(&mut self, mut predicate: P)
    where
        P: FnMut(chat::Id, &mut S) -> bool,
    {
        self.chats.retain(|&id, state| predicate(id, state))
    }
}

impl<S> IntoIterator for Chats<S> {
    type Item = (chat::Id, S);
    type IntoIter = IntoIter<chat::Id, S>;

    fn into_iter(self) -> Self::IntoIter {
        self.chats.into_iter()
    }
}

/// An iterator over the entries of [`Chats`].
///
/// [`Chats`]: ./struct.Chats.html
pub struct Iter<'a, S>(hash_map::Iter<'a, chat::Id, S>);

impl<'a, S> Iterator for Iter<'a, S> {
    type Item = (chat::Id, &'a S);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(&id, state)| (id, state))
    }
}

impl<'a, S> IntoIterator for &'a Chats<S> {
    type Item = (chat::Id, &'a S);
    type IntoIter = Iter<'a, S>;

    fn into_iter(self) -> Self::IntoIter {
        Iter(self.chats.iter())
    }
}

/// A mutable iterator over the entries of [`Chats`].
///
/// [`Chats`]: ./struct.Chats.html
pub struct IterMut<'a, S>(hash_map::IterMut<'a, chat::Id, S>);

impl<'a, S> Iterator for IterMut<'a, S> {
    type Item = (chat::Id, &'a mut S);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(&id, state)| (id, state))
    }
}

impl<'a, S> IntoIterator for &'a mut Chats<S> {
    type Item = (chat::Id, &'a mut S);
    type IntoIter = IterMut<'a, S>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut(self.chats.iter_mut())
    }
}

impl<S> Extend<(chat::Id, S)> for Chats<S> {
    fn extend<I: IntoIterator<Item = (chat::Id, S)>>(&mut self, iterator: I) {
        self.chats.extend(iterator)
    }
}

impl<'a, S: Copy> Extend<(chat::Id, &'a S)> for Chats<S> {
    fn extend<I: IntoIterator<Item = (chat::Id, &'a S)>>(
        &mut self,
        iterator: I,
    ) {
        self.extend(iterator.into_iter().map(|(id, &state)| (id, state)))
    }
}

impl<S> FromIterator<(chat::Id, S)> for Chats<S> {
    fn from_iter<I: IntoIterator<Item = (chat::Id, S)>>(iter: I) -> Self {
        Self {
            chats: HashMap::from_iter(iter),
        }
    }
}

impl<S> Index<chat::Id> for Chats<S> {
    type Output = S;

    fn index(&self, id: chat::Id) -> &S {
        self.chats.index(&id)
    }
}

impl<S> Default for Chats<S> {
    fn default() -> Self {
        Self::new()
    }
}

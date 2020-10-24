#![allow(deprecated)]

use is_macro::Is;

/// Represent URL visibility state.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Is)]
#[non_exhaustive]
#[deprecated(
    since = "0.6.6",
    note = "use `is_url_hidden` methods that take a `bool`"
)]
#[must_use]
pub enum UrlVisibility {
    /// The URL is visible.
    Shown,
    /// The URL is hidden.
    Hidden,
}

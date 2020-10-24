#![allow(deprecated)]

use is_macro::Is;

/// Chooses if a web page preview will be shown.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Is)]
#[deprecated(
    since = "0.6.6",
    note = "use `is_web_page_preview_disabled` methods that take a `bool`"
)]
#[non_exhaustive]
#[must_use]
pub enum WebPagePreviewState {
    /// The preview will be enabled.
    Enabled,
    /// The preview will be disabled.
    Disabled,
}

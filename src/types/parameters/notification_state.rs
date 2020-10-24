#![allow(deprecated)]

use is_macro::Is;

/// Chooses if a notification will be sent for a new message.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Is)]
#[non_exhaustive]
#[deprecated(
    since = "0.6.6",
    note = "Use `is_notification_disabled` methods which take a `bool`"
)]
#[must_use]
pub enum NotificationState {
    /// The notification will be enabled.
    Enabled,
    /// The notification will be disabled.
    Disabled,
}

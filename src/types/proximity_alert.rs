use super::User;
use serde::Deserialize;

/// Represents a service message sent when a user triggers a proximity alert
/// set by another user.
///
/// See [`ProximityAlertTriggered`] from Bot API docs.
///
/// [`ProximityAlertTriggered`]: https://core.telegram.org/bots/api#proximityalerttriggered
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct ProximityAlert {
    /// The user who triggered the alert.
    pub traveler: User,
    /// The user who set the alert.
    pub watcher: User,
    /// The distance between the users.
    pub distance: u32,
}

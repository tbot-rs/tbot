/// Chooses if a notification will be sent for a new message.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[non_exhaustive]
pub enum NotificationState {
    /// The notification will be enabled.
    Enabled,
    /// The notification will be disabled.
    Disabled,
}

impl NotificationState {
    /// Checks if the state is `Enabled`.
    pub fn is_enabled(self) -> bool {
        self == Self::Enabled
    }

    /// Checks if the state is `Disabled`.
    pub fn is_disabled(self) -> bool {
        self == Self::Disabled
    }
}

use serde::Deserialize;

/// Represents a service message about changed auto-delete timer.
///
/// See [`MessageAutoDeleteTimerChanged`] from Bot API docs.
///
/// [`MessageAutoDeleteTimerChanged`]: https://core.telegram.org/bots/api#messageautodeletetimerchanged
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize)]
pub struct AutoDeleteTimerChanged {
    /// The new auto-delete timer value.
    #[serde(rename = "message_auto_delete_time")]
    pub auto_delete_time: u64,
}

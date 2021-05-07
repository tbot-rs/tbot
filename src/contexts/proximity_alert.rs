use crate::types;

message_base! {
    struct ProximityAlert {
        /// The proximity alert
        alert: types::ProximityAlert,
    } -> EventLoop::proximity_alert

    fn new(alert: types::ProximityAlert,) -> Self {
        Self {
            alert: alert,
        }
    }
}

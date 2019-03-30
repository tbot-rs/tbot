use super::*;

/// Represents where the mask is placed.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaskPositionPoint {
    /// Placed on forehead.
    Forehead,
    /// Placed on eyes.
    Eyes,
    /// Placed on mouth.
    Mouth,
    /// Placed on chin.
    Chin,
}

/// Reperesents [`MaskPosition`].
///
/// [`MaskPosition`]: https://core.telegram.org/bots/api#maskposition
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct MaskPosition {
    /// The mask's position point.
    pub point: MaskPositionPoint,
    /// The mask's shift by X.
    pub x_shift: f64,
    /// The mask's shift by Y.
    pub y_shift: f64,
    /// The mask's scale.
    pub scale: f64,
}

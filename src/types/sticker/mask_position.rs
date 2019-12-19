//! Types representing a mask position.

use serde::{Deserialize, Serialize};

/// Represents where the mask is placed.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Point {
    /// Placed on forehead.
    Forehead,
    /// Placed on eyes.
    Eyes,
    /// Placed on mouth.
    Mouth,
    /// Placed on chin.
    Chin,
}

/// Represents a [`MaskPosition`].
///
/// [`MaskPosition`]: https://core.telegram.org/bots/api#maskposition
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MaskPosition {
    /// The position point of the mask.
    pub point: Point,
    /// The shift of the mask by X.
    pub x_shift: f64,
    /// The shift of the mask by Y.
    pub y_shift: f64,
    /// The scale of the mask.
    pub scale: f64,
}

impl Point {
    /// Checks if `self` is `Forehead`.
    pub fn is_forehead(self) -> bool {
        self == Self::Forehead
    }

    /// Checks if `self` is `Eyes`.
    pub fn is_eyes(self) -> bool {
        self == Self::Eyes
    }

    /// Checks if `self` is `Mouth`.
    pub fn is_mouth(self) -> bool {
        self == Self::Mouth
    }

    /// Checks if `self` is `Chin`.
    pub fn is_chin(self) -> bool {
        self == Self::Chin
    }
}

//! Types related to tips in invoices.

use serde::Serialize;

/// Represents tip parameters.
#[derive(Debug, Clone, Serialize, Eq, PartialEq, Hash)]
pub struct Tip {
    max_tip_amount: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggested_tip_amounts: Option<Vec<u32>>,
}

impl Tip {
    /// Initialize a `Tip` with a given max tip amount.
    #[must_use]
    pub const fn with_max(max_tip: u32) -> Self {
        Self {
            max_tip_amount: max_tip,
            suggested_tip_amounts: None,
        }
    }

    /// Configures suggested tip amounts for the invoice.
    /// At most 4 suggestions can be specified.
    /// Reflects the `suggested_tip_amounts` parameter.
    ///
    /// # Panics
    ///
    /// Panics if there are more than 4 elements.
    pub fn suggested_tips(mut self, suggested: impl Into<Vec<u32>>) -> Self {
        let mut suggested = suggested.into();
        assert!(
            (1..=4).contains(&suggested.len()),
            "[tbot] Received invalid `suggested` in \
             `Tip::suggested_tips`: must contain from 1 to 4 elements, \
             contains {} elements instead.",
            suggested.len(),
        );
        suggested.sort_unstable();
        assert!(
            suggested.windows(2).all(|win| win[0] != win[1]),
            "[tbot] Received invalid `suggested` in \
             `Tip::suggested_tips`: value most consist of unique elements.",
        );
        assert!(
            suggested[3] <= self.max_tip_amount,
            "[tbot] Received invalid `suggested` in \
             `Tip::suggested_tips`: the maximum value {} \
             is exceeding `max_tip_amount` which was set to {}.",
            suggested[3],
            self.max_tip_amount,
        );
        self.suggested_tip_amounts = Some(suggested);
        self
    }
}

//! Types related to tips in invoices.

use serde::Serialize;

/// Represents tip parameters.
#[derive(Debug, Clone, Serialize, Eq, PartialEq, Hash)]
#[must_use]
pub struct Tip {
    max_tip_amount: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggested_tip_amounts: Option<Vec<u32>>,
}

impl Tip {
    /// Constructs a `Tip` with the given max tip amount.
    /// Reflects the `max_tip_amount` parameter.
    pub const fn with_max(max_tip: u32) -> Self {
        Self {
            max_tip_amount: max_tip,
            suggested_tip_amounts: None,
        }
    }

    /// Configures suggested tip amounts for the invoice. At most 4 suggestions
    /// can be specified; this method automatically sorts them.
    /// Reflects the `suggested_tip_amounts` parameter.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - there are 0 or more than 4 tips;
    /// - there are duplicated tips;
    /// - the biggest tip exceeds `max_tip_amount`.
    pub fn suggested_tips(mut self, tips: impl Into<Vec<u32>>) -> Self {
        let mut tips = tips.into();
        assert!(
            (1..=4).contains(&tips.len()),
            "[tbot] Received invalid `tips` in `Tip::suggested_tips`: \
             must contain 1..=4 tips, contains {} instead.",
            tips.len(),
        );

        tips.sort_unstable();
        assert!(
            tips.windows(2).all(|win| win[0] != win[1]),
            "[tbot] Received invalid `tips` in `Tip::suggested_tips`: \
            must contain only unique tips.",
        );
        assert!(
            // this is a safe unwrap, since we've checked the length earlier
            *tips.last().unwrap() <= self.max_tip_amount,
            "[tbot] Received invalid `tips` in `Tip::suggested_tips`: \
             the maximum value {} exceeds `max_tip_amount` ({}).",
            tips.last().unwrap(),
            self.max_tip_amount,
        );

        self.suggested_tip_amounts = Some(tips);
        self
    }
}

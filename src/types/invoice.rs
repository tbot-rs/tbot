use serde::{Deserialize, Deserializer};

/// Represents an [`Invoice`].
///
/// [`Invoice`]: https://core.telegram.org/bots/api#invoice
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
#[non_exhaustive]
pub struct Invoice {
    /// The title of the invoice.
    pub title: String,
    /// The description of the invoice.
    pub description: String,
    /// The start parameter of the invoice.
    #[serde(deserialize_with = "deserialize_start_parameter")]
    pub start_parameter: Option<String>,
    /// The currency of the invoice.
    pub currency: String,
    /// The total amount of the invoice.
    pub total_amount: u32,
}

fn deserialize_start_parameter<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let string = String::deserialize(deserializer)?;

    Ok(if string.is_empty() {
        None
    } else {
        Some(string)
    })
}

use serde::Serialize;

/// Represents an [`InputContactMessageContent`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inputcontactmessagecontent
#[derive(Debug, PartialEq, Clone, Serialize)]
#[must_use]
pub struct Contact {
    phone_number: String,
    first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcard: Option<String>,
}

impl Contact {
    /// Constructs a `Contact`.
    pub fn new(
        phone_number: impl Into<String>,
        first_name: impl Into<String>,
    ) -> Self {
        Self {
            phone_number: phone_number.into(),
            first_name: first_name.into(),
            last_name: None,
            vcard: None,
        }
    }

    /// Configures the last name.
    pub fn last_name(mut self, last_name: impl Into<String>) -> Self {
        self.last_name = Some(last_name.into());
        self
    }

    /// Configures the vCard.
    pub fn vcard(mut self, vcard: impl Into<String>) -> Self {
        self.vcard = Some(vcard.into());
        self
    }
}

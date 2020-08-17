use serde::Serialize;

/// Represents an [`InputContactMessageContent`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inputcontactmessagecontent
#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
#[must_use]
pub struct Contact<'a> {
    phone_number: &'a str,
    first_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcard: Option<&'a str>,
}

impl<'a> Contact<'a> {
    /// Constructs a `Contact`.
    pub const fn new(phone_number: &'a str, first_name: &'a str) -> Self {
        Self {
            phone_number,
            first_name,
            last_name: None,
            vcard: None,
        }
    }

    /// Configures the last name.
    pub const fn last_name(mut self, last_name: &'a str) -> Self {
        self.last_name = Some(last_name);
        self
    }

    /// Configures the vCard.
    pub const fn vcard(mut self, vcard: &'a str) -> Self {
        self.vcard = Some(vcard);
        self
    }
}

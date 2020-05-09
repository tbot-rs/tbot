use serde::Serialize;
use std::borrow::Cow;

/// Represents an [`InputContactMessageContent`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inputcontactmessagecontent
#[derive(Debug, PartialEq, Clone, Serialize)]
#[must_use]
pub struct Contact<'a> {
    phone_number: Cow<'a, str>,
    first_name: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcard: Option<Cow<'a, str>>,
}

impl<'a> Contact<'a> {
    /// Constructs a `Contact`.
    pub fn new(phone_number: impl Into<Cow<'a, str>>, first_name: impl Into<Cow<'a, str>>) -> Self {
        Self {
            phone_number: phone_number.into(),
            first_name: first_name.into(),
            last_name: None,
            vcard: None,
        }
    }

    /// Configures the last name.
    pub fn last_name(mut self, last_name: impl Into<Cow<'a, str>>) -> Self {
        self.last_name = Some(last_name.into());
        self
    }

    /// Configures the vCard.
    pub fn vcard(mut self, vcard: impl Into<Cow<'a, str>>) -> Self {
        self.vcard = Some(vcard.into());
        self
    }
}

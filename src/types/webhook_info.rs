//! Types related to webhook information.

use crate::types::parameters::UpdateKind;
use serde::de::{Deserialize, Deserializer, Error, MapAccess, Visitor};

/// Represents information about the last error.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[non_exhaustive]
pub struct LastError {
    /// The timestamp of the error.
    pub date: i64,
    /// A human-readable description of the error.
    pub message: String,
}

/// Represents [`WebhookInfo`].
///
/// [`WebhookInfo`]: https://core.telegram.org/bots/api#webhookinfo
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[non_exhaustive]
pub struct WebhookInfo {
    /// The URL to which Telegram sends Webhook updates.
    pub url: String,
    /// `true` if a custom certificate was provided.
    pub has_custom_certificate: bool,
    /// Number of pending updates.
    pub pending_update_count: u32,
    /// Information about the last error that happened during sending an update.
    pub last_error: Option<LastError>,
    /// Maximum allowed number of connections at a time.
    pub max_connections: Option<u8>,
    /// A list of updates the bot is subscribed to.
    pub allowed_updates: Option<Vec<UpdateKind>>,
}

const URL: &str = "url";
const HAS_CUSTOM_CERTIFICATE: &str = "has_custom_certificate";
const PENDING_UPDATE_COUNT: &str = "pending_update_count";
const LAST_ERROR_DATE: &str = "last_error_date";
const LAST_ERROR_MESSAGE: &str = "last_error_message";
const MAX_CONNECTIONS: &str = "max_connections";
const ALLOWED_UPDATES: &str = "allowed_updates";

struct WebhookInfoVisitor;

impl<'v> Visitor<'v> for WebhookInfoVisitor {
    type Value = WebhookInfo;

    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "a WebhookInfo struct")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'v>,
    {
        let mut url = None;
        let mut has_custom_certificate = None;
        let mut pending_update_count = None;
        let mut last_error_date = None;
        let mut last_error_message = None;
        let mut max_connections = None;
        let mut allowed_updates = None;

        while let Some(key) = map.next_key()? {
            match key {
                URL => url = Some(map.next_value()?),
                HAS_CUSTOM_CERTIFICATE => {
                    has_custom_certificate = Some(map.next_value()?)
                }
                PENDING_UPDATE_COUNT => {
                    pending_update_count = Some(map.next_value()?)
                }
                LAST_ERROR_DATE => last_error_date = Some(map.next_value()?),
                LAST_ERROR_MESSAGE => {
                    last_error_message = Some(map.next_value()?)
                }
                MAX_CONNECTIONS => max_connections = Some(map.next_value()?),
                ALLOWED_UPDATES => allowed_updates = Some(map.next_value()?),
                _ => (),
            }
        }

        let url = url.ok_or_else(|| Error::missing_field(URL))?;
        let has_custom_certificate = has_custom_certificate
            .ok_or_else(|| Error::missing_field(HAS_CUSTOM_CERTIFICATE))?;
        let pending_update_count = pending_update_count
            .ok_or_else(|| Error::missing_field(PENDING_UPDATE_COUNT))?;
        let last_error = last_error_date.and_then(|date| {
            last_error_message.map(|message| LastError { date, message })
        });

        Ok(WebhookInfo {
            url,
            has_custom_certificate,
            pending_update_count,
            last_error,
            max_connections,
            allowed_updates,
        })
    }
}

impl<'de> Deserialize<'de> for WebhookInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "WebhookInfo",
            &[
                URL,
                HAS_CUSTOM_CERTIFICATE,
                PENDING_UPDATE_COUNT,
                LAST_ERROR_DATE,
                LAST_ERROR_MESSAGE,
                MAX_CONNECTIONS,
                ALLOWED_UPDATES,
            ],
            WebhookInfoVisitor,
        )
    }
}

use super::call_method;
use crate::{bot::InnerBot, errors, types::parameters::UpdateKind, Multipart};
use std::{net::IpAddr, num::NonZeroU32};

/// This method isn't meant to be used by users directly.
#[derive(Debug, Clone)]
#[must_use]
pub struct SetWebhook<'a> {
    bot: &'a InnerBot,
    url: &'a str,
    ip_address: Option<IpAddr>,
    certificate: Option<&'a str>,
    max_connections: Option<NonZeroU32>,
    allowed_updates: Option<&'a [UpdateKind]>,
}

impl<'a> SetWebhook<'a> {
    pub(crate) const fn new(
        bot: &'a InnerBot,
        url: &'a str,
        ip_address: Option<IpAddr>,
        certificate: Option<&'a str>,
        max_connections: Option<NonZeroU32>,
        allowed_updates: Option<&'a [UpdateKind]>,
    ) -> Self {
        Self {
            bot,
            url,
            ip_address,
            certificate,
            max_connections,
            allowed_updates,
        }
    }
}

impl SetWebhook<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        let mut multipart = Multipart::new(5)
            .str("url", self.url)
            .maybe_string("ip_address", self.ip_address)
            .maybe_string("max_connections", self.max_connections)
            .maybe_json("allowed_updates", self.allowed_updates);

        if let Some(certificate) = self.certificate {
            multipart = multipart.file(
                "certificate",
                "certificate.pem",
                certificate.as_bytes(),
            );
        }

        let (boundary, body) = multipart.finish();

        call_method::<bool>(self.bot, "setWebhook", Some(boundary), body)
            .await?;

        Ok(())
    }
}

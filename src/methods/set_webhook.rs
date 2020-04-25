use super::call_method;
use crate::{
    connectors::Client, errors, token, types::parameters::UpdateKind, Multipart,
};

/// This method isn't meant to be used by users directly.
#[derive(Debug, Clone)]
#[must_use]
pub(crate) struct SetWebhook<'a> {
    client: &'a Client,
    token: token::Ref<'a>,
    url: &'a str,
    certificate: Option<&'a str>,
    max_connections: Option<u8>,
    allowed_updates: Option<&'a [UpdateKind]>,
}

impl<'a> SetWebhook<'a> {
    pub(crate) const fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        url: &'a str,
        certificate: Option<&'a str>,
        max_connections: Option<u8>,
        allowed_updates: Option<&'a [UpdateKind]>,
    ) -> Self {
        Self {
            client,
            token,
            url,
            certificate,
            max_connections,
            allowed_updates,
        }
    }
}

impl SetWebhook<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        let mut multipart = Multipart::new(4)
            .str("url", self.url)
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

        call_method::<bool>(
            self.client,
            self.token,
            "setWebhook",
            Some(boundary),
            body,
        )
        .await?;

        Ok(())
    }
}

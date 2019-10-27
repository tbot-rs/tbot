use super::send_method;
use crate::{
    connectors::Connector, errors, internal::Client,
    types::parameters::Updates, Multipart, token,
};

/// This method isn't meant to be used by users directly.
#[derive(Debug, Clone)]
#[must_use]
pub(crate) struct SetWebhook<'a, C> {
    client: &'a Client<C>,
    token: token::Ref<'a>,
    url: &'a str,
    certificate: Option<&'a str>,
    max_connections: Option<u8>,
    allowed_updates: Option<&'a [Updates]>,
}

impl<'a, C> SetWebhook<'a, C> {
    pub(crate) const fn new(
        client: &'a Client<C>,
        token: token::Ref<'a>,
        url: &'a str,
        certificate: Option<&'a str>,
        max_connections: Option<u8>,
        allowed_updates: Option<&'a [Updates]>,
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

impl<C: Connector> SetWebhook<'_, C> {
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

        send_method::<bool, _>(
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

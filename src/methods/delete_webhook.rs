use super::send_method;
use crate::{connectors::Client, errors, token};

#[derive(Debug, Clone)]
#[must_use]
pub struct DeleteWebhook<'a> {
    client: &'a Client,
    token: token::Ref<'a>,
}

impl<'a> DeleteWebhook<'a> {
    pub(crate) const fn new(client: &'a Client, token: token::Ref<'a>) -> Self {
        Self { client, token }
    }
}

impl DeleteWebhook<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        send_method::<bool>(
            self.client,
            self.token,
            "deleteWebhook",
            None,
            Vec::new(),
        )
        .await?;

        Ok(())
    }
}

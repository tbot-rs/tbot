use super::send_method;
use crate::{
    connectors::Connector,
    errors,
    internal::Client,
    types::{parameters::Updates, Update},
    Token,
};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[must_use]
pub(crate) struct GetUpdates<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_updates: Option<&'a [Updates]>,
}

impl<'a, C> GetUpdates<'a, C> {
    pub(crate) const fn new(
        client: &'a Client<C>,
        token: Token,
        offset: Option<isize>,
        limit: Option<u8>,
        timeout: Option<u64>,
        allowed_updates: Option<&'a [Updates]>,
    ) -> Self {
        Self {
            client,
            token,
            offset,
            limit,
            timeout,
            allowed_updates,
        }
    }
}

impl<C: Connector> GetUpdates<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<Vec<Update>, errors::MethodCall> {
        send_method(
            self.client,
            &self.token,
            "getUpdates",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}

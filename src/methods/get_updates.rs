use super::*;
use crate::internal::Client;
use std::sync::Arc;

#[derive(Serialize)]
#[must_use]
pub(crate) struct GetUpdates<'a, C> {
    #[serde(skip)]
    client: Arc<Client<C>>,
    #[serde(skip)]
    token: Token,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_updates: Option<&'a [types::Updates]>,
}

impl<'a, C> GetUpdates<'a, C> {
    pub const fn new(
        client: Arc<Client<C>>,
        token: Token,
        offset: Option<u32>,
        limit: Option<u8>,
        timeout: Option<u32>,
        allowed_updates: Option<&'a [types::Updates]>,
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

impl<C> IntoFuture for GetUpdates<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = Vec<types::Update>;
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            &self.client,
            &self.token,
            "getUpdates",
            None,
            serde_json::to_vec(&self).unwrap(),
        ))
    }
}

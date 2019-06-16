use super::*;
use crate::{internal::Client, types::parameters::Updates};

#[derive(Serialize, Debug, Clone)]
#[must_use]
pub(crate) struct GetUpdates<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u32>,
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
        offset: Option<u32>,
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
            self.client,
            &self.token,
            "getUpdates",
            None,
            serde_json::to_vec(&self).unwrap(),
        ))
    }
}

use super::*;

#[derive(Serialize)]
#[must_use]
pub(crate) struct GetUpdates<'a> {
    #[serde(skip)]
    token: Token,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_updates: Option<&'a [types::Updates]>,
}

impl<'a> GetUpdates<'a> {
    #[cfg(feature = "proxy")]
    pub const fn new(
        token: Token,
        offset: Option<u32>,
        limit: Option<u8>,
        timeout: Option<u32>,
        allowed_updates: Option<&'a [types::Updates]>,
        proxy: Option<proxy::Proxy>,
    ) -> Self {
        Self {
            token,
            offset,
            limit,
            timeout,
            allowed_updates,
            proxy,
        }
    }

    #[cfg(not(feature = "proxy"))]
    pub const fn new(
        token: Token,
        offset: Option<u32>,
        limit: Option<u8>,
        timeout: Option<u32>,
        allowed_updates: Option<&'a [types::Updates]>,
    ) -> Self {
        Self {
            token,
            offset,
            limit,
            timeout,
            allowed_updates,
        }
    }
}

impl IntoFuture for GetUpdates<'_> {
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = Vec<types::Update>;
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            &self.token,
            "getUpdates",
            None,
            serde_json::to_vec(&self).unwrap(),
            #[cfg(feature = "proxy")]
            self.proxy,
        ))
    }
}

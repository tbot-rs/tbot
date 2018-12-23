use super::*;

/// This method isn't meant to be used by users directly.
#[derive(Serialize)]
#[must_use]
pub(crate) struct GetUpdates<'a> {
    #[serde(skip)]
    token: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_updates: Option<&'a [types::Updates]>,
}

impl<'a> GetUpdates<'a> {
    pub fn new(
        token: &'a str,
        offset: Option<u64>,
        limit: Option<u8>,
        timeout: Option<u64>,
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

    #[must_use]
    pub fn into_future(
        self,
    ) -> impl Future<Item = Vec<types::Update>, Error = DeliveryError> {
        send_method::<Vec<types::Update>>(
            self.token,
            "getUpdates",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
    }
}

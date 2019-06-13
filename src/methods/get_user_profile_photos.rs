use super::*;
use crate::internal::Client;
use std::sync::Arc;

// This is a false positive as it's used in `into_future`'s signature
#[allow(dead_code)]
type Photos = Vec<Vec<types::UserProfilePhotos>>;

/// Represents the [`getUserProfilePhotos`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getuserprofilephotos
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetUserProfilePhotos<C> {
    #[serde(skip)]
    client: Arc<Client<C>>,
    #[serde(skip)]
    token: Token,
    user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u8>,
}

impl<C> GetUserProfilePhotos<C> {
    pub(crate) const fn new(
        client: Arc<Client<C>>,
        token: Token,
        user_id: i64,
    ) -> Self {
        Self {
            client,
            token,
            user_id,
            offset: None,
            limit: None,
        }
    }

    /// Configures `offset`.
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Configures `limit`.
    pub fn limit(mut self, limit: u8) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl<C> IntoFuture for GetUserProfilePhotos<C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = Photos;
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            &self.client,
            &self.token,
            "getUserProfilePhotos",
            None,
            serde_json::to_vec(&self).unwrap(),
        ))
    }
}

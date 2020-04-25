use super::call_method;
use crate::{connectors::Client, errors, token, types::user};
use serde::Serialize;

/// Gets a user's profile photos.
///
/// Reflects the [`getUserProfilePhotos`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getuserprofilephotos
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetUserProfilePhotos<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    user_id: user::Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u8>,
}

impl<'a> GetUserProfilePhotos<'a> {
    pub(crate) const fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        user_id: user::Id,
    ) -> Self {
        Self {
            client,
            token,
            user_id,
            offset: None,
            limit: None,
        }
    }

    /// Configures the number of the first photo to be returned.
    /// Reflects the `offset` parameter.
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Configures how many photos will be returned. Must be in the range
    /// `1..=100`; defaults to 100. Reflects the `limit` parameter.
    pub fn limit(mut self, limit: u8) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl GetUserProfilePhotos<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<user::ProfilePhotos, errors::MethodCall> {
        call_method(
            self.client,
            self.token,
            "getUserProfilePhotos",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}

use super::*;

type Photos = Vec<Vec<types::UserProfilePhotos>>;

/// Representation of the [`getUserProfilePhotos`] method.
///
/// [`getUserProfilePhotos`]: https://core.telegram.org/bots/api#getuserprofilephotos
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetUserProfilePhotos<'a> {
    #[serde(skip)]
    token: &'a str,
    user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u8>,
}

impl<'a> GetUserProfilePhotos<'a> {
    /// Constructs a new `GetUserProfilePhotos`.
    pub fn new<'b: 'a>(token: &'b str, user_id: i64) -> Self {
        Self {
            token,
            user_id,
            offset: None,
            limit: None,
        }
    }

    /// Configures `offset`.
    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Configures `limit`.
    pub fn limit(mut self, limit: u8) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Prepares the request and returns a `Future`.
    #[must_use = "futures do nothing unless polled"]
    pub fn into_future(
        self,
    ) -> impl Future<Item = Photos, Error = DeliveryError> {
        send_method::<Photos>(
            self.token,
            "getUserProfilePhotos",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
    }
}

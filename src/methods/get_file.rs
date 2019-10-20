use super::*;
use crate::{
    connectors::Connector,
    errors,
    internal::Client,
    types::file::{self, id::AsFileId, File},
};

/// Gets information about a file.
///
/// Reflects the [`getfile`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getfile
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetFile<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    file_id: file::id::Ref<'a>,
}

impl<'a, C> GetFile<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        file_id: &'a impl AsFileId,
    ) -> Self {
        Self {
            client,
            token,
            file_id: file_id.as_file_id(),
        }
    }
}

impl<C: Connector> GetFile<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<File, errors::MethodCall> {
        send_method(
            self.client,
            &self.token,
            "getFile",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}

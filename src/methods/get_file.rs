use super::send_method;
use crate::{
    connectors::Client,
    errors, token,
    types::file::{self, id::AsFileId, File},
};
use serde::Serialize;

/// Gets information about a file.
///
/// Reflects the [`getfile`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getfile
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetFile<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    file_id: file::id::Ref<'a>,
}

impl<'a> GetFile<'a> {
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        file_id: &'a impl AsFileId,
    ) -> Self {
        Self {
            client,
            token,
            file_id: file_id.as_file_id(),
        }
    }
}

impl GetFile<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<File, errors::MethodCall> {
        send_method(
            self.client,
            self.token,
            "getFile",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}

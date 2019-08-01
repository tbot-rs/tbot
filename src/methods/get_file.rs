use super::*;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    types::{
        file::{id::AsFileId, File},
        value::FileId,
    },
};

/// Represents the [`getfile`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getfile
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetFile<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    file_id: FileId<'a>,
}

impl<'a, C> GetFile<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        file_id: impl AsFileId<'a>,
    ) -> Self {
        Self {
            client,
            token,
            file_id: file_id.as_file_id(),
        }
    }
}

impl<C> IntoFuture for GetFile<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = File;
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            self.client,
            &self.token,
            "getFile",
            None,
            serde_json::to_vec(&self).unwrap(),
        ))
    }
}

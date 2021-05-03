use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::file::{self, File},
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
    bot: &'a InnerBot,
    file_id: file::Id,
}

impl<'a> GetFile<'a> {
    pub(crate) const fn new(bot: &'a InnerBot, file_id: file::Id) -> Self {
        Self { bot, file_id }
    }
}

impl GetFile<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<File, errors::MethodCall> {
        call_method(
            self.bot,
            "getFile",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}

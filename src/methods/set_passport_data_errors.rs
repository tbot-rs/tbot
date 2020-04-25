use super::send_method;
use crate::{
    connectors::Client,
    errors, token,
    types::{passport, user},
};
use serde::Serialize;

/// Reports passport errors to the user.
///
/// Reflects the [`setPassportDataErrors`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setpassportdataerrors
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetPassportDataErrors<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    user_id: user::Id,
    errors: &'a [passport::element::Error<'a>],
}

impl<'a> SetPassportDataErrors<'a> {
    pub(crate) const fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        user_id: user::Id,
        errors: &'a [passport::element::Error<'a>],
    ) -> Self {
        Self {
            client,
            token,
            user_id,
            errors,
        }
    }
}

impl SetPassportDataErrors<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        send_method::<bool>(
            self.client,
            self.token,
            "setPassportDataErrors",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}

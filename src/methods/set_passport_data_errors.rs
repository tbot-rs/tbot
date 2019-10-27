use super::send_method;
use crate::{
    connectors::Connector,
    errors,
    internal::Client,
    types::{passport, user},
    Token,
};
use serde::Serialize;

/// Reports passport errors to the user.
///
/// Reflects the [`setPassportDataErrors`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setpassportdataerrors
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetPassportDataErrors<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    user_id: user::Id,
    errors: &'a [passport::element::Error<'a>],
}

impl<'a, C> SetPassportDataErrors<'a, C> {
    pub(crate) const fn new(
        client: &'a Client<C>,
        token: Token,
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

impl<C: Connector> SetPassportDataErrors<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        send_method::<bool, _>(
            self.client,
            &self.token,
            "setPassportDataErrors",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}

use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
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
    bot: &'a InnerBot,
    user_id: user::Id,
    errors: &'a [passport::element::Error<'a>],
}

impl<'a> SetPassportDataErrors<'a> {
    pub(crate) const fn new(
        bot: &'a InnerBot,
        user_id: user::Id,
        errors: &'a [passport::element::Error<'a>],
    ) -> Self {
        Self {
            bot,
            user_id,
            errors,
        }
    }
}

impl SetPassportDataErrors<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.bot,
            "setPassportDataErrors",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}

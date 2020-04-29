//! A few useful predicates for media messages.

use crate::contexts::fields::Document;
use futures::future::BoxFuture;
use std::{path::Path, sync::Arc, ops::Deref};

/// Checks if document extension matches one of given extensions.
pub fn match_extension<'a, I: 'a, T, C: 'a>(
    extensions: I,
) -> Box<
    dyn Fn(Arc<C>) -> BoxFuture<'a, bool>
        + Send
        + Sync
        + 'a,
>
where
    for<'b> &'b I: IntoIterator<Item = &'b T>,
    T: Deref<Target = str>,
    I: Send + Sync,
    C: Document + Send + Sync,
{
    let extensions = Arc::new(extensions);

    Box::new(move |context| {
        let extensions = Arc::clone(&extensions);

        Box::pin(async move {
            let file_name = if let Some(file_name) =
                context.document().file_name.as_ref()
            {
                file_name
            } else {
                return false;
            };

            let extension =
                if let Some(extension) = Path::new(&file_name).extension() {
                    extension
                } else {
                    return false;
                };

            let extension = extension.to_string_lossy();

            extensions.into_iter().any(|x| x.deref() == extension)
        })
    })
}

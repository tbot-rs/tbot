//! Telegram Bots API methods in form of structs.
//!
//! Note that structs here are somewhat raw, because they require tokens when
//! construcing. You would more like using construction methods on `Tbot` that
//! infer `token`, and methods on context structs that in addition infer things
//! like `chat_id` or `callback_query_id`.
//!
//! Though methods don't implement a trait like `Method` for reasons, they all
//! have methods `new` with varying number of parameters and `get_request` that
//! returns a `Future` which resolves with Telegram's response or an error. The
//! design philosophy is that all required parameters are passed through `new`
//! and all optional parameters are set with a dedicated method. Then
//! `get_request` is called to get the `Future`, handling is done if needed and
//! it's passed to a runner. For convenience, we re-export `tokio::{run, spawn}`
//! as `tbot::{run, spawn}`.
//!
//! For example, here's how you'd call `getMe`:
//!
//! ```
//! use tbot::prelude::*;
//!
//! let token = std::env::var("BOT_TOKEN").unwrap();
//!
//! let request = tbot::methods::GetMe::new(&token)
//!     .into_future()
//!     .map_err(|_| ())
//!     .map(|me| println!("Here I am: {:#?}", me));
//!
//! tbot::run(request);
//! ```
//!
//! Note that getting the request may fail when establishing an HTTPS connector
//! under the hood, so we unwrap the returned request. You should also handle
//! errors properly, unlike we did.

use super::*;

mod edit_inline_location;
mod edit_message_location;
mod forward_message;
mod get_me;
mod send_location;
mod send_message;

pub use self::edit_inline_location::*;
pub use self::edit_message_location::*;
pub use self::forward_message::*;
pub use self::get_me::*;
pub use self::send_location::*;
pub use self::send_message::*;

use futures::{Future, Stream};

#[derive(Deserialize)]
struct ResponseParameters {
    pub migrate_to_chat_id: Option<i64>,
    pub retry_after: Option<u64>,
}

#[derive(Deserialize)]
struct Response<T> {
    result: Option<T>,
    description: Option<String>,
    error_code: Option<i64>,
    parameters: Option<ResponseParameters>,
}

/// An error happened during request. Different errors may happen, so this is
/// an enum representing error that may happen during request.
#[derive(Debug)]
pub enum DeliveryError {
    /// Telegram Bots API responded with an HTML page what usually means it's
    /// down.
    TelegramOutOfService,
    /// `serde_json` couldn't parse the response. Most probably, it's a bug in
    /// `tbot` that tried to parse the response into a wrong struct, what you
    /// should fill an issue for on [our GitLab repository][issues].
    ///
    /// [issues]: https://gitlab.com/snejugal/tbot/issues
    InvalidResponse(serde_json::error::Error),
    /// Some error happened during sending the request.
    NetworkError(hyper::Error),
    /// Telegram returned an error in response. That is most probably your
    /// fault.
    RequestError {
        /// Human-readable description of the error.
        description: String,
        /// Error code reflected through HTTP error codes (for example, 401).
        error_code: i64,
        /// The group moved to a supergroup.
        migrate_to_chat_id: Option<i64>,
        /// When exceeding flood control, you must wait for this amount of
        /// seconds before making another request.
        retry_after: Option<u64>,
    },
}

#[must_use]
fn create_method_url(token: &str, method: &'static str) -> hyper::Uri {
    format!("https://api.telegram.org/bot{}/{}", token, method).parse().unwrap()
}

#[must_use]
fn send_method<'a, T: serde::de::DeserializeOwned + std::fmt::Debug>(
    token: &'a str,
    method: &'static str,
    boundary: Option<String>,
    body: Vec<u8>,
) -> impl Future<Item = T, Error = DeliveryError> {
    let https = hyper_tls::HttpsConnector::new(1).unwrap();

    let mut request = hyper::Request::new(hyper::Body::from(body));
    *request.method_mut() = hyper::Method::POST;
    *request.uri_mut() = create_method_url(token, method);
    request.headers_mut().insert(
        hyper::header::CONTENT_TYPE,
        hyper::header::HeaderValue::from_static(if let Some(_) = &boundary {
            "muiltipart/form-data"
        } else {
            "application/json"
        }),
    );

    let client = hyper::Client::builder()
        .build::<_, hyper::Body>(https)
        .request(request)
        .and_then(|response| response.into_body().concat2())
        .map_err(|error| DeliveryError::NetworkError(error))
        .and_then(|response| {
            if response.starts_with(b"<") {
                // If so, then Bots API is down and returns an HTML. Handling
                // this case specially.
                Err(DeliveryError::TelegramOutOfService)
            } else {
                match serde_json::from_slice::<Response<T>>(&response[..]) {
                    Ok(response) => Ok(response),
                    Err(error) => Err(DeliveryError::InvalidResponse(error)),
                }
            }
        }).and_then(|response| {
            if let Some(result) = response.result {
                Ok(result)
            } else {
                let (migrate_to_chat_id, retry_after) = match response
                    .parameters
                {
                    Some(parameters) => {
                        (parameters.migrate_to_chat_id, parameters.retry_after)
                    }
                    None => (None, None),
                };

                // If result is empty, then it's a error. In this case,
                // description and error_code are guaranteed to be specified in
                // the response, so we can unwrap it.
                Err(DeliveryError::RequestError {
                    description: response.description.unwrap(),
                    error_code: response.error_code.unwrap(),
                    migrate_to_chat_id,
                    retry_after,
                })
            }
        });

    client
}

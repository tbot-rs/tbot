use super::*;
use crate::{errors, internal::Client};
use futures::Stream;

#[derive(Deserialize)]
struct ResponseParameters {
    migrate_to_chat_id: Option<chat::Id>,
    retry_after: Option<u64>,
}

#[derive(Deserialize)]
struct Response<T> {
    result: Option<T>,
    description: Option<String>,
    error_code: Option<u16>,
    parameters: Option<ResponseParameters>,
}

#[must_use]
fn create_method_url(token: &Token, method: &'static str) -> hyper::Uri {
    hyper::Uri::builder()
        .scheme("https")
        .authority("api.telegram.org")
        .path_and_query(format!("/bot{}/{}", token.as_str(), method).as_str())
        .build()
        .unwrap()
}

#[must_use]
fn create_request(
    token: &Token,
    method: &'static str,
    boundary: Option<String>,
    body: Vec<u8>,
) -> hyper::Request<hyper::Body> {
    let mut request = hyper::Request::new(hyper::Body::from(body));
    *request.method_mut() = hyper::Method::POST;
    *request.uri_mut() = create_method_url(token, method);

    if let Some(boundary) = boundary {
        let content_type =
            format!("multipart/form-data; boundary={}", boundary);

        request.headers_mut().insert(
            hyper::header::CONTENT_TYPE,
            // disallowed characters shouldn't appear
            hyper::header::HeaderValue::from_str(&content_type).unwrap(),
        );
    } else {
        request.headers_mut().insert(
            hyper::header::CONTENT_TYPE,
            hyper::header::HeaderValue::from_static("application/json"),
        );
    }

    request
}

#[must_use]
fn process_response<T>(
    request: hyper::client::ResponseFuture,
) -> impl Future<Item = T, Error = errors::MethodCall>
where
    T: serde::de::DeserializeOwned,
{
    request
        .and_then(|response| response.into_body().concat2())
        .map_err(errors::MethodCall::Network)
        .and_then(|response| {
            if response.starts_with(b"<") {
                // If so, then Bots API is down and returns an HTML. Handling
                // this case specially.
                return Err(errors::MethodCall::OutOfService);
            }

            serde_json::from_slice::<Response<T>>(&response[..]).map_err(
                |error| errors::MethodCall::Parse(errors::ParseError {
                    response,
                    error,
                }),
            )
        })
        .and_then(|response| {
            if let Some(result) = response.result {
                return Ok(result);
            }

            let (migrate_to_chat_id, retry_after) = match response.parameters {
                Some(parameters) => {
                    (parameters.migrate_to_chat_id, parameters.retry_after)
                }
                None => (None, None),
            };

            // If result is empty, then it's a error. In this case, description
            // and error_code are guaranteed to be specified in the response,
            // so we can unwrap it.
            Err(errors::MethodCall::RequestError {
                description: response.description.unwrap(),
                error_code: response.error_code.unwrap(),
                migrate_to_chat_id,
                retry_after,
            })
        })
}

#[must_use]
pub fn send_method<T, C>(
    client: &Client<C>,
    token: &Token,
    method: &'static str,
    boundary: Option<String>,
    body: Vec<u8>,
) -> impl Future<Item = T, Error = errors::MethodCall>
where
    T: serde::de::DeserializeOwned,
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    let request = create_request(token, method, boundary, body);

    process_response(client.request(request))
}

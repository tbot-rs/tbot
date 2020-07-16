use crate::{connectors::Client, errors, token, types::chat};
use hyper::{
    body::{Body, HttpBody},
    header::HeaderValue,
    Method, Request, Uri,
};
use serde::{de::DeserializeOwned, Deserialize};
use std::{
    fmt::{self, Debug, Formatter},
    str::from_utf8,
};
use tracing::{error, field::debug, instrument, trace};

struct DebugBytes<'a>(&'a [u8]);

impl Debug for DebugBytes<'_> {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        if let Ok(string) = from_utf8(self.0) {
            write!(formatter, "{}", string)
        } else {
            write!(formatter, "{:?}", self.0)
        }
    }
}

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

#[instrument(skip(client, token, boundary, body))]
pub async fn call_method<'a, T>(
    client: &'a Client,
    token: token::Ref<'a>,
    method: &'static str,
    boundary: Option<String>,
    body: Vec<u8>,
) -> Result<T, errors::MethodCall>
where
    T: DeserializeOwned + Debug,
{
    trace!(body = debug(DebugBytes(&body)), boundary = debug(&boundary));

    let url = Uri::builder()
        .scheme("https")
        .authority("api.telegram.org")
        .path_and_query(format!("/bot{}/{}", token.as_str(), method).as_str())
        .build()
        .expect("[tbot] Method URL construction failed");

    let mut request = Request::new(Body::from(body));
    *request.method_mut() = Method::POST;
    *request.uri_mut() = url;

    let content_type = if let Some(boundary) = boundary {
        let value = format!("multipart/form-data; boundary={}", boundary);

        // disallowed characters shouldn't appear
        HeaderValue::from_str(&value).unwrap()
    } else {
        HeaderValue::from_static("application/json")
    };

    request
        .headers_mut()
        .insert(hyper::header::CONTENT_TYPE, content_type);

    let (parts, mut body) = client
        .request(request)
        .await
        .map_err(|error| {
            let error: errors::MethodCall = error.into();
            error!(error = debug(&error));
            error
        })?
        .into_parts();

    let mut response = parts
        .headers
        .get("Content-Length")
        .and_then(|x| x.to_str().ok().and_then(|x| x.parse().ok()))
        .map_or_else(Vec::new, Vec::with_capacity);

    while let Some(chunk) = body.data().await {
        response.extend(chunk.map_err(|error| {
            let error: errors::MethodCall = error.into();
            error!(error = debug(&error));
            error
        })?);
    }

    if response.starts_with(b"<") {
        // If so, then Bots API is down and returns HTML.
        // Handling this case specially.

        error!(error = debug(&errors::MethodCall::OutOfService));
        return Err(errors::MethodCall::OutOfService);
    }

    let response: Response<T> =
        serde_json::from_slice(&response[..]).map_err(|error| {
            let error = errors::MethodCall::Parse { response, error };
            error!(error = debug(&error));
            error
        })?;

    if let Some(result) = response.result {
        trace!(result = debug(&result));
        return Ok(result);
    }

    let (migrate_to_chat_id, retry_after) = match response.parameters {
        Some(parameters) => {
            (parameters.migrate_to_chat_id, parameters.retry_after)
        }
        None => (None, None),
    };

    // If result is empty, then it's an error. In this case, description and
    // error_code are guaranteed to be specified in the response, so we can
    // unwrap it.
    let error = errors::MethodCall::RequestError {
        description: response.description.unwrap(),
        error_code: response.error_code.unwrap(),
        migrate_to_chat_id,
        retry_after,
    };

    trace!(error = debug(&error));

    Err(error)
}

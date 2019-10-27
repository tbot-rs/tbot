use crate::{
    connectors::Connector, errors, internal::Client, types::chat, Token,
};
use hyper::{header::HeaderValue, Body, Method, Request, Uri};
use serde::{de::DeserializeOwned, Deserialize};

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
pub async fn send_method<'a, T, C>(
    client: &'a Client<C>,
    token: &'a Token,
    method: &'static str,
    boundary: Option<String>,
    body: Vec<u8>,
) -> Result<T, errors::MethodCall>
where
    T: DeserializeOwned,
    C: Connector,
{
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

    let (parts, mut body) = client.request(request).await?.into_parts();

    let mut response = parts
        .headers
        .get("Content-Length")
        .and_then(|x| x.to_str().ok().and_then(|x| x.parse().ok()))
        .map_or_else(Vec::new, Vec::with_capacity);

    while let Some(chunk) = body.next().await {
        response.extend(chunk?);
    }

    if response.starts_with(b"<") {
        // If so, then Bots API is down and returns an HTML.
        // Handling this case specially.
        return Err(errors::MethodCall::OutOfService);
    }

    let response: Response<T> = serde_json::from_slice(&response[..])
        .map_err(|error| errors::MethodCall::Parse { response, error })?;

    if let Some(result) = response.result {
        return Ok(result);
    }

    let (migrate_to_chat_id, retry_after) = match response.parameters {
        Some(parameters) => {
            (parameters.migrate_to_chat_id, parameters.retry_after)
        }
        None => (None, None),
    };

    // If result is empty, then it's a error. In this case, description and
    // error_code are guaranteed to be specified in the response, so we can
    // unwrap it.
    Err(errors::MethodCall::RequestError {
        description: response.description.unwrap(),
        error_code: response.error_code.unwrap(),
        migrate_to_chat_id,
        retry_after,
    })
}

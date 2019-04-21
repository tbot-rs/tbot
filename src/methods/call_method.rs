use super::*;
use futures::Stream;

#[derive(Deserialize)]
struct ResponseParameters {
    migrate_to_chat_id: Option<i64>,
    retry_after: Option<u64>,
}

#[derive(Deserialize)]
struct Response<T> {
    result: Option<T>,
    description: Option<String>,
    error_code: Option<u8>,
    parameters: Option<ResponseParameters>,
}

#[must_use]
fn create_method_url(token: &str, method: &'static str) -> hyper::Uri {
    hyper::Uri::builder()
        .scheme("https")
        .authority("api.telegram.org")
        .path_and_query(format!("/bot{}/{}", token, method).as_str())
        .build()
        .unwrap()
}

#[must_use]
fn create_request(
    token: &str,
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
fn process_response<T: serde::de::DeserializeOwned + std::fmt::Debug>(
    request: hyper::client::ResponseFuture,
) -> impl Future<Item = T, Error = DeliveryError> {
    request
        .and_then(|response| response.into_body().concat2())
        .map_err(DeliveryError::NetworkError)
        .and_then(|response| {
            if response.starts_with(b"<") {
                // If so, then Bots API is down and returns an HTML. Handling
                // this case specially.
                return Err(DeliveryError::TelegramOutOfService);
            }

            match serde_json::from_slice::<Response<T>>(&response[..]) {
                Ok(response) => Ok(response),
                Err(error) => Err(DeliveryError::InvalidResponse(error)),
            }
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
            Err(DeliveryError::RequestError {
                description: response.description.unwrap(),
                error_code: response.error_code.unwrap(),
                migrate_to_chat_id,
                retry_after,
            })
        })
}

#[cfg(not(feature = "proxy"))]
#[must_use]
pub fn send_method<T: serde::de::DeserializeOwned + std::fmt::Debug>(
    token: &str,
    method: &'static str,
    boundary: Option<String>,
    body: Vec<u8>,
) -> impl Future<Item = T, Error = DeliveryError> {
    let https = hyper_tls::HttpsConnector::new(1).unwrap();
    let request = create_request(token, method, boundary, body);

    process_response(
        hyper::Client::builder()
            .build::<_, hyper::Body>(https)
            .request(request),
    )
}

#[cfg(feature = "proxy")]
#[must_use]
pub fn send_method<T: serde::de::DeserializeOwned + std::fmt::Debug>(
    token: &str,
    method: &'static str,
    boundary: Option<String>,
    body: Vec<u8>,
    proxy: Option<proxy::Proxy>,
) -> impl Future<Item = T, Error = DeliveryError> {
    let request = create_request(token, method, boundary, body);
    let https = hyper_tls::HttpsConnector::new(1).unwrap();
    let builder = hyper::Client::builder();

    let request = if let Some(proxy) = proxy {
        let connector =
            proxy::ProxyConnector::from_proxy(https, proxy).unwrap();
        builder.build::<_, hyper::Body>(connector).request(request)
    } else {
        builder.build::<_, hyper::Body>(https).request(request)
    };

    process_response(request)
}

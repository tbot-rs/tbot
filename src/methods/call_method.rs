use crate::{bot::InnerBot, errors, types::chat};
use hyper::{
    body::{Body, HttpBody},
    header::HeaderValue,
    http::uri::PathAndQuery,
    Method, Request, Uri,
};
use serde::{de::DeserializeOwned, Deserialize};
use std::{
    error::Error,
    fmt::{self, Debug, Formatter, Write},
    str::from_utf8,
};
use tracing::{error, instrument, trace};

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

fn construct_uri(
    bot: &InnerBot,
    method: &'static str,
) -> Result<Uri, Box<dyn Error>> {
    let mut uri_parts = bot.uri().into_parts();
    let path = uri_parts.path_and_query.as_ref().map_or("/", |x| x.path());
    let query = uri_parts
        .path_and_query
        .as_ref()
        .and_then(PathAndQuery::query);

    let mut new_path = String::from(path);

    if !new_path.ends_with('/') {
        new_path.push('/');
    }
    write!(&mut new_path, "bot{}/{}", bot.token(), method)?;

    if let Some(query) = query {
        write!(&mut new_path, "?{}", query)?;
    }

    uri_parts.path_and_query = Some(new_path.parse()?);

    Uri::from_parts(uri_parts).map_err(Into::into)
}

#[instrument(skip(bot, boundary, body))]
pub async fn call_method<'a, T>(
    bot: &'a InnerBot,
    method: &'static str,
    boundary: Option<String>,
    body: Vec<u8>,
) -> Result<T, errors::MethodCall>
where
    T: DeserializeOwned + Debug,
{
    trace!(body = ?DebugBytes(&body), ?boundary);

    let mut request = Request::new(Body::from(body));
    *request.method_mut() = Method::POST;
    *request.uri_mut() = construct_uri(bot, method)
        .expect("[tbot] Method URI construction failed");

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

    let (parts, mut body) = bot
        .client()
        .request(request)
        .await
        .map_err(|error| {
            let error: errors::MethodCall = error.into();
            error!(?error);
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
            error!(?error);
            error
        })?);
    }

    trace!(raw_response = ?DebugBytes(&response));

    if response.starts_with(b"<") {
        // If so, then Bots API is down and returns HTML.
        // Handling this case specially.

        let error = errors::MethodCall::OutOfService;
        error!(?error);
        return Err(error);
    }

    let response: Response<T> =
        serde_json::from_slice(&response[..]).map_err(|error| {
            let error = errors::MethodCall::Parse { response, error };
            error!(?error);
            error
        })?;

    if let Some(result) = response.result {
        trace!(?result);
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

    trace!(?error);

    Err(error)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{connectors::Client, token::Token};

    #[test]
    fn construts_uri_correctly() -> Result<(), Box<dyn Error>> {
        let mut bot =
            InnerBot::new(Token(String::from("TOKEN")), Client::https());

        let uri = construct_uri(&bot, "method")?;
        assert_eq!(
            uri,
            Uri::from_static("https://api.telegram.org/botTOKEN/method")
        );

        bot.set_uri(Uri::from_static("http://localhost"));
        let uri = construct_uri(&bot, "method")?;
        assert_eq!(uri, Uri::from_static("http://localhost/botTOKEN/method"));

        bot.set_uri(Uri::from_static("http://localhost/"));
        let uri = construct_uri(&bot, "method")?;
        assert_eq!(uri, Uri::from_static("http://localhost/botTOKEN/method"));

        bot.set_uri(Uri::from_static("http://localhost:8081/"));
        let uri = construct_uri(&bot, "method")?;
        assert_eq!(
            uri,
            Uri::from_static("http://localhost:8081/botTOKEN/method")
        );

        bot.set_uri(Uri::from_static("http://localhost/foo"));
        let uri = construct_uri(&bot, "method")?;
        assert_eq!(
            uri,
            Uri::from_static("http://localhost/foo/botTOKEN/method")
        );

        bot.set_uri(Uri::from_static("http://localhost/?bar"));
        let uri = construct_uri(&bot, "method")?;
        assert_eq!(
            uri,
            Uri::from_static("http://localhost/botTOKEN/method?bar")
        );

        bot.set_uri(Uri::from_static("http://localhost/foo?bar"));
        let uri = construct_uri(&bot, "method")?;
        assert_eq!(
            uri,
            Uri::from_static("http://localhost/foo/botTOKEN/method?bar")
        );

        bot.set_uri(Uri::from_static("http://localhost/foo/?bar"));
        let uri = construct_uri(&bot, "method")?;
        assert_eq!(
            uri,
            Uri::from_static("http://localhost/foo/botTOKEN/method?bar")
        );

        Ok(())
    }
}

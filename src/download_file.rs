use crate::{
    connectors::Connector, errors, internal::Client, token, types::File,
};
use hyper::{body::HttpBody, StatusCode, Uri};

pub async fn download_file<C>(
    client: &Client<C>,
    token: token::Ref<'_>,
    file: &File,
) -> Result<Vec<u8>, errors::Download>
where
    C: Connector,
{
    let path = match &file.path {
        Some(path) => path,
        None => return Err(errors::Download::NoPath),
    };

    let url = Uri::builder()
        .scheme("https")
        .authority("api.telegram.org")
        .path_and_query(
            format!("/file/bot{}/{}", token.as_str(), path).as_str(),
        )
        .build()
        .unwrap_or_else(|err| {
            panic!("\n[tbot] Download URL construction failed: {:#?}\n", err);
        });

    let (parts, mut body) = client.get(url).await?.into_parts();

    if parts.status != StatusCode::OK {
        return Err(errors::Download::InvalidStatusCode(parts.status));
    }

    let mut response = parts
        .headers
        .get("Content-Length")
        .and_then(|x| x.to_str().ok().and_then(|x| x.parse().ok()))
        .map_or_else(Vec::new, Vec::with_capacity);

    while let Some(chunk) = body.data().await {
        response.extend(chunk?);
    }

    Ok(response)
}

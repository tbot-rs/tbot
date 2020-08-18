use crate::{bot::InnerBot, errors, types::File};
use hyper::{body::HttpBody, StatusCode, Uri};

pub async fn download_file(
    bot: &InnerBot,
    file: &File,
) -> Result<Vec<u8>, errors::Download> {
    let path = match &file.path {
        Some(path) => path,
        None => return Err(errors::Download::NoPath),
    };

    let url = Uri::builder()
        .scheme("https")
        .authority("api.telegram.org")
        .path_and_query(format!("/file/bot{}/{}", bot.token(), path).as_str())
        .build()
        .unwrap_or_else(|err| {
            panic!("\n[tbot] Download URL construction failed: {:#?}\n", err);
        });

    let (parts, mut body) = bot.client().get(url).await?.into_parts();

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

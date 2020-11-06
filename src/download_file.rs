use crate::{bot::InnerBot, errors, types::File};
use hyper::{body::HttpBody, http::uri::PathAndQuery, StatusCode, Uri};
use std::{error::Error, fmt::Write, path::Path};
use tokio::fs;

fn construct_uri(
    bot: &InnerBot,
    file_path: &str,
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
    write!(&mut new_path, "file/bot{}/{}", bot.token(), file_path)?;

    if let Some(query) = query {
        write!(&mut new_path, "?{}", query)?;
    }

    uri_parts.path_and_query = Some(new_path.parse()?);

    Ok(Uri::from_parts(uri_parts)?)
}

pub async fn download_file(
    bot: &InnerBot,
    file: &File,
) -> Result<Vec<u8>, errors::Download> {
    let path = match &file.path {
        Some(path) => path,
        None => return Err(errors::Download::NoPath),
    };

    if Path::new(&path).is_absolute() {
        return Ok(fs::read(&path).await?);
    }

    let url = construct_uri(bot, path)
        .expect("[tbot] Download URI construction failed");

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

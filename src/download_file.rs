/// Represents possible errors whic may occur while downloading a file.
#[derive(Debug)]
pub enum DownloadError {
    /// The provided file had the `path` field set to `None`.
    NoPath,
    /// A network error.
    Network(hyper::Error),
    /// Telegram returned a different from 200 status code.
    InvalidStatusCode(StatusCode),
}

use crate::{internal::Client, prelude::*, types::File, Token};
use futures::{
    future::{err, Either},
    Stream,
};
use hyper::{StatusCode, Uri};

pub fn download_file<C>(
    client: &Client<C>,
    token: &Token,
    file: &File,
) -> impl Future<Item = Vec<u8>, Error = DownloadError>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    let path = match &file.path {
        Some(path) => path,
        None => return Either::A(err(DownloadError::NoPath)),
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

    Either::B(
        client
            .get(url)
            .map_err(DownloadError::Network)
            .and_then(|response| {
                let status = response.status();

                if status == StatusCode::OK {
                    Either::A(
                        response
                            .into_body()
                            .concat2()
                            .map_err(DownloadError::Network),
                    )
                } else {
                    Either::B(err(DownloadError::InvalidStatusCode(status)))
                }
            })
            .map(|response| response[..].to_vec()),
    )
}

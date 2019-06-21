use crate::{errors, internal::Client, prelude::*, types::File, Token};
use futures::{
    future::{err, Either},
    Stream,
};
use hyper::{StatusCode, Uri};

pub fn download_file<C>(
    client: &Client<C>,
    token: &Token,
    file: &File,
) -> impl Future<Item = Vec<u8>, Error = errors::Download>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    let path = match &file.path {
        Some(path) => path,
        None => return Either::A(err(errors::Download::NoPath)),
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
            .map_err(errors::Download::Network)
            .and_then(|response| {
                let status = response.status();

                if status == StatusCode::OK {
                    Either::A(
                        response
                            .into_body()
                            .concat2()
                            .map_err(errors::Download::Network),
                    )
                } else {
                    Either::B(err(errors::Download::InvalidStatusCode(status)))
                }
            })
            .map(|response| response[..].to_vec()),
    )
}

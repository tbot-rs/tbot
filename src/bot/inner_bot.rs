use crate::{connectors::Client, token::Token};
use hyper::Uri;

const CLOUD_BOT_API: &str = "https://api.telegram.org/";

#[derive(Debug)]
pub struct InnerBot {
    token: Token,
    client: Client,
    uri: Uri,
}

impl InnerBot {
    pub fn new(token: Token, client: Client) -> Self {
        Self {
            token,
            client,
            uri: Uri::from_static(CLOUD_BOT_API),
        }
    }

    pub fn set_client(&mut self, client: Client) {
        self.client = client;
    }

    pub fn set_uri(&mut self, uri: Uri) {
        self.uri = uri;
    }

    pub fn token(&self) -> &str {
        &self.token.0
    }

    pub const fn client(&self) -> &Client {
        &self.client
    }

    pub fn uri(&self) -> Uri {
        self.uri.clone()
    }
}

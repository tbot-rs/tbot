use crate::{connectors::Client, token::Token};

#[derive(Debug)]
pub struct InnerBot {
    token: Token,
    client: Client,
}

impl InnerBot {
    pub const fn new(token: Token, client: Client) -> Self {
        Self { token, client }
    }

    pub fn set_client(&mut self, client: Client) {
        self.client = client;
    }

    pub fn token(&self) -> &str {
        &self.token.0
    }

    pub const fn client(&self) -> &Client {
        &self.client
    }
}

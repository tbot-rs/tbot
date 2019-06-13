#[cfg(not(feature = "proxy"))]
compile_error!("compile the example with the `proxy` feature enabled");

use tbot::{connectors::proxy::*, prelude::*};

const CHAT: i64 = 0;
const PROXY: &str = "http://127.0.0.1:8080";

fn main() {
    let proxy = Proxy::new(Intercept::All, PROXY.parse().unwrap());
    let connector = tbot::connectors::proxy(proxy);
    let bot = tbot::bot!("BOT_TOKEN", connector);

    let message = bot
        .send_message(CHAT, "Hi from behind a proxy!")
        .into_future()
        .map_err(|error| {
            dbg!(error);
        });

    tbot::run(message);
}

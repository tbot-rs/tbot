#[cfg(not(feature = "proxy"))]
compile_error!("compile the example with the `proxy` feature enabled");

use tbot::{prelude::*, proxy::*, Bot};

const CHAT: i64 = 0;
const PROXY: &str = "http://127.0.0.1:8080";

fn main() {
    let mut bot = Bot::from_env("BOT_TOKEN");
    bot.proxy(Proxy::new(Intercept::All, PROXY.parse().unwrap()));

    let message = bot
        .send_message(CHAT, "Hi from behind a proxy!")
        .into_future()
        .map_err(|error| {
            dbg!(error);
        });

    tbot::run(message);
}

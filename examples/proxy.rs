#[cfg(not(feature = "proxy"))]
compile_error!("compile the example with the `proxy` feature enabled");

use tbot::{connectors::proxy::*, prelude::*};

const PROXY: &str = "http://127.0.0.1:8080";

fn main() {
    let proxy = Proxy::new(Intercept::All, PROXY.parse().unwrap());
    let connector = tbot::connectors::proxy(proxy);

    let mut bot = tbot::from_env!("BOT_TOKEN", connector).event_loop();

    bot.text(|context| {
        let reply = context
            .send_message_in_reply(&context.text.value)
            .into_future()
            .map_err(|err| {
                dbg!(err);
            });

        tbot::spawn(reply);
    });

    bot.polling().start();
}

use tbot::{
    prelude::*,
    proxy::https::{Intercept, Proxy},
    BotBuilder,
};

const PROXY: &str = "http://127.0.0.1:8080";

#[tokio::main]
async fn main() {
    let proxy = Proxy::new(Intercept::All, PROXY.parse().unwrap());
    // or, for SOCKS:
    // let proxy = tbot::proxy::Proxy::socks(SOCKS_PROXY, AUTH);

    let mut bot = BotBuilder::with_env_token("BOT_TOKEN")
        .proxy(proxy)
        .build()
        .event_loop();

    bot.text(|context| async move {
        context
            .send_message_in_reply(&context.text.value)
            .call()
            .await
            .unwrap();
    });

    bot.polling().start().await.unwrap();
}

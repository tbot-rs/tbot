use tbot::{
    prelude::*,
    proxy::{Intercept, Proxy},
    Bot,
};

const PROXY: &str = "http://127.0.0.1:8080";

#[tokio::main]
async fn main() {
    let proxy = Proxy::new(Intercept::All, PROXY.parse().unwrap());

    let mut bot = Bot::from_env_with_proxy("BOT_TOKEN", proxy).event_loop();

    bot.text(|context| async move {
        context
            .send_message_in_reply(&context.text.value)
            .call()
            .await
            .unwrap();
    });

    bot.polling().start().await.unwrap();
}

use tbot::{
    connectors::{
        self,
        proxy::{Intercept, Proxy},
    },
    prelude::*,
};

const PROXY: &str = "http://127.0.0.1:8080";

#[tokio::main]
async fn main() {
    let proxy = Proxy::new(Intercept::All, PROXY.parse().unwrap());
    let connector = connectors::proxy(proxy);

    let mut bot = tbot::from_env!("BOT_TOKEN", connector).event_loop();

    bot.text(|context| {
        async move {
            context
                .send_message_in_reply(&context.text.value)
                .call()
                .await
                .unwrap();
        }
    });

    bot.polling().start().await.unwrap();
}

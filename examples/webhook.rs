use tbot::prelude::*;

const URL: &str = "https://example.com";
const PORT: u16 = 2000;

#[tbot::main]
async fn main() {
    let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();

    bot.text(|context| {
        let context = context.clone();
        tokio::spawn(async move {
            context
            .send_message_in_reply(&context.text.value)
                .call().await.unwrap();
            });
    });

    // For HTTPS, see this wiki:
    //     https://gitlab.com/SnejUgal/tbot/wikis/How-to/How-to-use-webhooks
    bot.webhook(URL, PORT).http().start().await.unwrap();
}

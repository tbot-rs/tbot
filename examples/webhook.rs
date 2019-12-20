use tbot::prelude::*;

const URL: &str = "https://example.com";
const PORT: u16 = 2000;

#[tokio::main]
async fn main() {
    let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();

    bot.text(|context| {
        async move {
            let echo = &context.text.value;
            let call_result = context.send_message_in_reply(echo).call().await;

            if let Err(err) = call_result {
                dbg!(err);
            }
        }
    });

    // For HTTPS, see this wiki:
    //     https://gitlab.com/SnejUgal/tbot/wikis/How-to/How-to-use-webhooks
    bot.webhook(URL, PORT).http().start().await.unwrap();
}

use tbot::{prelude::*, Bot};

#[tokio::main]
async fn main() {
    let mut bot = Bot::from_env("BOT_TOKEN").event_loop();

    bot.command_with_description(
        "hello",
        "Sends hello",
        |context| async move {
            let call_result =
                context.send_message_in_reply("Hello!").call().await;

            if let Err(error) = call_result {
                dbg!(error);
            }
        },
    );

    bot.help_with_description("Shows help", |context| async move {
        let call_result = context
            .send_message_in_reply("Just send me a /hello")
            .call()
            .await;

        if let Err(error) = call_result {
            dbg!(error);
        }
    });

    bot.polling().start().await.unwrap();
}

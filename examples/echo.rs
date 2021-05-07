use tbot::{markup::markdown_v2, prelude::*, util::entities, Bot};

#[tokio::main]
async fn main() {
    let mut bot = Bot::from_env("BOT_TOKEN").event_loop();

    bot.text(|context| async move {
        let entities = entities(&context.text);
        let echo = markdown_v2(entities);

        let call_result = context.send_message(echo).call().await;
        if let Err(error) = call_result {
            dbg!(error);
        }
    });

    bot.polling().start().await.unwrap();
}

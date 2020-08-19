use tbot::{markup::markdown_v2, prelude::*, util::entities, Bot};

#[tokio::main]
async fn main() {
    let mut bot = Bot::from_env("BOT_TOKEN").event_loop();

    bot.text(|context| async move {
        let entities = entities(&context.text);
        let echo = markdown_v2(entities);

        context.send_message(echo).call().await.unwrap();
    });

    bot.polling().start().await.unwrap();
}

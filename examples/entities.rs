use tbot::{prelude::*, util::entities, Bot};

#[tokio::main]
async fn main() {
    let mut bot = Bot::from_env("BOT_TOKEN").event_loop();

    bot.text(|context| async move {
        context
            .send_message_in_reply(
                format!("{:#?}", entities(&context.text)).as_str(),
            )
            .call()
            .await
            .unwrap();
    });

    bot.polling().start().await.unwrap();
}

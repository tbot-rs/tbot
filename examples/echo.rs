use tbot::{
    markup::markdown_v2, prelude::*, types::parameters::Text, util::entities,
    Bot,
};

#[tokio::main]
async fn main() {
    let mut bot = Bot::from_env("BOT_TOKEN").event_loop();

    bot.text(|context| async move {
        let entities = entities(&context.text);

        let echo = markdown_v2(entities).to_string();

        context
            .send_message(Text::with_markdown_v2(&echo))
            .call()
            .await
            .unwrap();
    });

    bot.polling().start().await.unwrap();
}

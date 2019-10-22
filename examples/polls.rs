use tbot::prelude::*;

const QUESTION: &str = "Do you like tbot?";
const OPTIONS: &[&str] =
    &["Yes", "Also yes", "I like shooting myself in the foot more"];

#[tbot::main]
async fn main() {
    let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();

    bot.command("poll", |context| {
        let context = context.clone();
        tokio::spawn(async move {
            context.send_poll(QUESTION, OPTIONS).call().await.unwrap();
        });
    });

    bot.command("close", |context| {
        let context = context.clone();
        tokio::spawn(async move {
            if let Some(message) = &context.reply_to {
                context
                    .bot
                    .stop_poll(context.chat.id, message.id)
                    .call()
                    .await
                    .unwrap();
            } else {
                context
                    .send_message("Please send the command in reply to a poll")
                    .call()
                    .await
                    .unwrap();
            }
        });
    });

    bot.poll(|context| {
        println!("Someone sent a poll: {:#?}", context.poll);
    });

    bot.updated_poll(|context| {
        println!("New update on my poll: {:#?}", context.poll);
    });

    bot.polling().start().await.unwrap();
}

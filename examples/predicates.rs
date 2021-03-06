use std::sync::Arc;
use tbot::{
    compositors::filter,
    contexts::{Document, Text},
    predicates::{
        chat::{is_group, is_supergroup},
        media::match_extension,
        PredicateBooleanOperations,
    },
    prelude::*,
    Bot,
};

async fn is_message_short(context: Arc<Text>) -> bool {
    context.text.value.chars().count() < 5
}

#[tokio::main]
async fn main() {
    let mut bot = Bot::from_env("BOT_TOKEN").event_loop();

    bot.text(filter(
        is_supergroup.or(is_group).and(is_message_short.not()),
        |context| async move {
            let call_result =
                context.send_message_in_reply("Hello group!").call().await;

            if let Err(error) = call_result {
                dbg!(error);
            }
        },
    ));

    bot.text(filter(is_message_short, |context| async move {
        let call_result = context
            .send_message_in_reply("The message is too short!")
            .call()
            .await;

        if let Err(error) = call_result {
            dbg!(error);
        }
    }));

    bot.document(filter(
        match_extension(["rs", "toml"]),
        |context: Arc<Document>| async move {
            let call_result = context
                .send_message_in_reply(
                    "I see you're a man of the culture as well!",
                )
                .call()
                .await;

            if let Err(error) = call_result {
                dbg!(error);
            }
        },
    ));

    bot.polling().start().await.unwrap();
}

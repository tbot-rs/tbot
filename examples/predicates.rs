use std::sync::Arc;
use tbot::{
    contexts::Text,
    predicates::{
        chat::{is_group, is_private, is_supergroup},
        media::match_extension,
        PredicateBooleanOperations,
    },
    prelude::*,
    Bot,
};

async fn is_message_short(context: Arc<Text>) -> bool {
    context.text.value.len() < 5
}

#[tokio::main]
async fn main() {
    let mut bot = Bot::from_env("BOT_TOKEN").event_loop();

    bot.text_if(is_private, |context| async move {
        // We are sure that `from` is Some
        context
            .send_message_in_reply(&format!(
                "Hello {name}!",
                name = &context.from.as_ref().unwrap().first_name
            ))
            .call()
            .await
            .unwrap();
    });

    bot.text_if(
        is_supergroup.or(is_group).and(is_message_short.not()),
        |context| async move {
            context
                .send_message_in_reply("Hello group!")
                .call()
                .await
                .unwrap();
        },
    );

    bot.text_if(is_message_short, |context| async move {
        context
            .send_message_in_reply("Message is too short!")
            .call()
            .await
            .unwrap();
    });

    bot.document_if(match_extension(["rs", "toml"]), |context| async move {
        context
            .send_message_in_reply("I see you're a man of the culture as well!")
            .call()
            .await
            .unwrap();
    });

    bot.polling().start().await.unwrap();
}

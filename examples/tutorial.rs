use std::sync::Arc;
use tbot::{
    prelude::*,
    types::{
        inline_query::{self, result::Article},
        input_message_content::Text,
        parameters::Text as ParseMode,
    },
};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();

    bot.text(|context| {
        async move {
            let message = match meval::eval_str(&context.text.value) {
                Ok(result) => format!("= `{}`", result),
                Err(_) => {
                    "Whops, I couldn't evaluate your expression :(".into()
                }
            };

            context
                .send_message_in_reply(ParseMode::markdown(&message))
                .call()
                .await
                .unwrap();
        }
    });

    let id = Arc::new(Mutex::new(0_u32));
    bot.inline(move |context| {
        let id = Arc::clone(&id);
        async move {
            let (title, message) = match meval::eval_str(&context.query) {
                Ok(result) => (
                    result.to_string(),
                    format!("`{} = {}`", context.query, result),
                ),
                Err(_) => (
                    "Whops...".into(),
                    "I couldn't evaluate your expression :(".into(),
                ),
            };

            let id = {
                let mut id = id.lock().await;
                *id += 1;
                id.to_string()
            };
            let content = Text::new(ParseMode::markdown(&message));
            let article = Article::new(&title, content).description(&message);
            let result = inline_query::Result::new(&id, article);

            context.answer(&[result]).call().await.unwrap();
        }
    });

    bot.polling().start().await.unwrap();
}

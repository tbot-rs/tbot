use std::sync::Arc;
use tbot::{
    markup::{inline_code, markdown_v2},
    prelude::*,
    types::{
        inline_query::{self, result::Article},
        input_message_content::Text,
        parameters::Text as ParseMode,
    },
    Bot,
};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let mut bot = Bot::from_env("BOT_TOKEN").event_loop();

    bot.text(|context| async move {
        let calc_result = meval::eval_str(&context.text.value);
        let message = if let Ok(answer) = calc_result {
            markdown_v2(("= ", inline_code([answer.to_string()]))).to_string()
        } else {
            markdown_v2("Whops, I couldn't evaluate your expression :(")
                .to_string()
        };
        let reply = ParseMode::markdown_v2(&message);

        let call_result = context.send_message_in_reply(reply).call().await;
        if let Err(err) = call_result {
            dbg!(err);
        }
    });

    let id = Arc::new(Mutex::new(0_u32));
    bot.inline(move |context| {
        let id = Arc::clone(&id);
        async move {
            let calc_result = meval::eval_str(&context.query);
            let (title, message) = if let Ok(answer) = calc_result {
                let answer = answer.to_string();
                let message = markdown_v2(inline_code([
                    context.query.as_str(),
                    " = ",
                    answer.as_str(),
                ]))
                .to_string();
                (answer, message)
            } else {
                let title = "Whops...".into();
                let message =
                    markdown_v2("I couldn't evaluate your expression :(")
                        .to_string();
                (title, message)
            };

            let id = {
                let mut id = id.lock().await;
                *id += 1;
                id.to_string()
            };
            let content = Text::new(ParseMode::markdown_v2(&message));
            let article = Article::new(&title, content).description(&message);
            let result = inline_query::Result::new(&id, article);

            let call_result = context.answer(&[result]).call().await;
            if let Err(err) = call_result {
                dbg!(err);
            }
        }
    });

    bot.polling().start().await.unwrap();
}

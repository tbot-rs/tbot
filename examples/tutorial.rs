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
    let mut bot =
        Bot::from_env("BOT_TOKEN").stateful_event_loop(Mutex::new(0_u32));

    bot.text(|context, _| async move {
        let calc_result = meval::eval_str(&context.text.value);
        let message: ParseMode = if let Ok(answer) = calc_result {
            markdown_v2(("= ", inline_code(answer.to_string()))).into()
        } else {
            markdown_v2("Whops, I couldn't evaluate your expression :(").into()
        };

        let call_result = context.send_message_in_reply(message).call().await;
        if let Err(err) = call_result {
            dbg!(err);
        }
    });

    bot.inline(move |context, id| async move {
        let calc_result = meval::eval_str(&context.query);
        let title;
        let message: ParseMode;
        let description;

        if let Ok(answer) = calc_result {
            title = answer.to_string();
            message = markdown_v2(inline_code(format!(
                "{} = {}",
                context.query, answer,
            )))
            .into();
            description = format!("{} = {}", context.query, answer);
        } else {
            title = "Whops...".into();
            message =
                markdown_v2("I couldn't evaluate your expression :(").into();
            description =
                String::from("I couldn't evaluate your expression :(");
        };

        let id = {
            let mut id = id.lock().await;
            *id += 1;
            id.to_string()
        };
        let content = Text::new(message);
        let article = Article::new(title, content).description(description);
        let result = inline_query::Result::new(id, article);

        let call_result = context.answer(&[result][..]).call().await;
        if let Err(err) = call_result {
            dbg!(err);
        }
    });

    bot.polling().start().await.unwrap();
}

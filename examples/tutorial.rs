use tbot::{
    prelude::*,
    types::{
        inline_query::{self, result::Article},
        input_message_content::Text,
        parameters::Text as ParseMode,
        InputMessageContent,
    },
};

fn main() {
    let mut bot = tbot::bot!("BOT_TOKEN").event_loop();

    bot.text(|context| {
        let message = match meval::eval_str(&context.text.value) {
            Ok(result) => format!("= `{}`", result),
            Err(_) => "Whops, I couldn't evaluate your expression :(".into(),
        };

        let reply = context
            .send_message_in_reply(ParseMode::markdown(&message))
            .into_future()
            .map_err(|err| {
                dbg!(err);
            });

        tbot::spawn(reply);
    });

    let mut id: u32 = 0;

    bot.inline(move |context| {
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

        id += 1;

        let content: InputMessageContent =
            Text::new(ParseMode::markdown(&message)).into();

        let article: inline_query::result::Kind =
            Article::new(title, content).description(&message).into();

        let result = inline_query::Result::new(id.to_string(), article);
        let answer =
            context.answer(&[result.into()][..]).into_future().map_err(|err| {
                dbg!(err);
            });

        tbot::spawn(answer);
    });

    bot.polling().start();
}

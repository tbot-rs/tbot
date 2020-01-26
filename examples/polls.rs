use tbot::{prelude::*, types::parameters::Poll};

const QUESTION: &str = "Do you like tbot?";
const OPTIONS: &[&str] =
    &["Yes", "Also yes", "I like shooting myself in the foot more"];
const SEND_IN_REPLY_ERROR: &str = "Please send the command in reply to a poll";

const QUIZ_QUESTION: &str = "The best telegram bot library is?";
const QUIZ_OPTIONS: &[&str] = &["aiogram", "tbot", "pythontelegrambot"];

#[tokio::main]
async fn main() {
    let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();

    let regular = Poll::regular(QUESTION, OPTIONS, false);

    let quiz = Poll::quiz(QUIZ_QUESTION, QUIZ_OPTIONS, 1);

    dbg!(serde_json::to_string(&quiz));

    bot.command("poll", move |context| {
        async move {
            let call_result = context.send_poll(&regular).call().await;
            if let Err(err) = call_result {
                dbg!(err);
            }
        }
    });

    bot.command("quiz", move |context| {
        async move {
            let call_result = context.send_poll(&quiz).call().await;
            if let Err(err) = call_result {
                dbg!(err);
            }
        }
    });

    bot.command("close", |context| {
        async move {
            let err = if let Some(message) = &context.reply_to {
                let chat_id = context.chat.id;
                let call_result =
                    context.bot.stop_poll(chat_id, message.id).call().await;

                call_result.err()
            } else {
                context.send_message(SEND_IN_REPLY_ERROR).call().await.err()
            };

            if let Some(err) = err {
                dbg!(err);
            }
        }
    });

    bot.poll(|context| {
        println!("Someone sent a poll: {:#?}", context.poll);
        async move {}
    });

    bot.updated_poll(|context| {
        println!("New update on my poll: {:#?}", context.poll);
        async move {}
    });

    bot.polling().start().await.unwrap();
}

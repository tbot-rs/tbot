use tbot::{
    prelude::*,
    types::parameters::{
        poll::{Answer, AutoClose},
        Poll,
    },
    Bot,
};

const QUESTION: &str = "Do you like tbot?";
const OPTIONS: &[&str] =
    &["Yes", "Also yes", "I like shooting myself in the foot more"];
const SEND_IN_REPLY_ERROR: &str = "Please send the command in reply to a poll";

const QUIZ_QUESTION: &str = "The best Telegram bot library is...";
const QUIZ_OPTIONS: &[&str] = &["aiogram", "tbot", "python-telegram-bot"];
const QUIZ_CORRECT_OPTION: usize = 1;
const QUIZ_EXPLANATION: &str =
    "Why would you want to use something else than tbot for writing bots?";

#[tokio::main]
async fn main() {
    let mut bot = Bot::from_env("BOT_TOKEN").event_loop();

    let regular = Poll::regular(QUESTION, OPTIONS, Answer::Single);

    let quiz = Poll::quiz(
        QUIZ_QUESTION,
        QUIZ_OPTIONS,
        QUIZ_CORRECT_OPTION,
        Some(QUIZ_EXPLANATION),
    )
    .anonymous(false);

    bot.command("poll", move |context| async move {
        let call_result = context
            .send_poll(&regular)
            .auto_close(AutoClose::OpenPeriod(60))
            .call()
            .await;
        if let Err(err) = call_result {
            dbg!(err);
        }
    });

    bot.command("quiz", move |context| async move {
        let call_result = context.send_poll(&quiz).call().await;
        if let Err(err) = call_result {
            dbg!(err);
        }
    });

    bot.command("close", |context| async move {
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
    });

    bot.poll(|context| {
        println!("Someone sent a poll: {:#?}", context.poll);
        async {}
    });

    bot.updated_poll(|context| {
        println!("New update on my poll: {:#?}", context.poll);
        async {}
    });

    bot.poll_answer(|context| {
        println!("New answer in my poll: {:#?}", context.answer);
        async {}
    });

    bot.polling().start().await.unwrap();
}

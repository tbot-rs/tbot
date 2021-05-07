use tbot::{
    prelude::*,
    types::parameters::poll::{self, Answer, AutoClose, Poll, Quiz},
    Bot,
};

#[tokio::main]
async fn main() {
    let mut bot = Bot::from_env("BOT_TOKEN").event_loop();

    bot.command("poll", |context| async move {
        let regular = poll::Any::new(
            "Do you like tbot?",
            [
                "Yes".to_owned(),
                "Also yes".to_owned(),
                "I like shooting myself in the foot more".to_owned(),
            ],
            Poll::new(Answer::Single),
        )
        .auto_close(AutoClose::OpenPeriod(60));

        let call_result = context.send_poll(regular).call().await;
        if let Err(err) = call_result {
            dbg!(err);
        }
    });

    bot.command("quiz", |context| async move {
        let quiz = poll::Any::new(
            "The best Telegram bot library is...",
            [
                "aiogram".to_owned(),
                "tbot".to_owned(),
                "python-telegram-bot".to_owned(),
            ],
            Quiz::new(1).explanation(
                "Why would you want to use something else than tbot for \
                 writing bots?",
            ),
        )
        .is_anonymous(false);

        let call_result = context.send_poll(quiz).call().await;
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
            context
                .send_message("Please send the command in reply to a poll")
                .call()
                .await
                .err()
        };

        if let Some(err) = err {
            dbg!(err);
        }
    });

    bot.poll(|context| async move {
        println!("Someone sent a poll: {:#?}", context.poll);
    });

    bot.updated_poll(|context| async move {
        println!("New update on my poll: {:#?}", context.poll);
    });

    bot.poll_answer(|context| async move {
        println!("New answer in my poll: {:#?}", context.answer);
    });

    bot.polling().start().await.unwrap();
}

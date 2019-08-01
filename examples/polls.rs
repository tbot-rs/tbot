use futures::future::Either;
use tbot::prelude::*;

const QUESTION: &str = "Do you like tbot?";

fn main() {
    let options = [
        "Yes".into(),
        "Also yes".into(),
        "I like shooting myself in the foot more".into(),
    ];
    let mut bot = tbot::bot!("BOT_TOKEN").event_loop();

    bot.command("poll", move |context| {
        let poll = context
            .send_poll(QUESTION, &options[..])
            .into_future()
            .map_err(|err| {
                dbg!(err);
            });

        tbot::spawn(poll);
    });

    bot.command("close", |context| {
        let request = match &context.reply_to {
            Some(message) => {
                let stop = context.bot.stop_poll(context.chat.id, message.id);

                Either::A(stop.into_future().map(|_| ()))
            }
            None => {
                let warning = context
                    .send_message("Please send the command in reply to a poll");

                Either::B(warning.into_future().map(|_| ()))
            }
        };

        tbot::spawn(request.map_err(|err| {
            dbg!(err);
        }));
    });

    bot.poll(|context| {
        println!("Someone sent a poll: {:#?}", context.poll);
    });

    bot.updated_poll(|context| {
        println!("New update on my poll: {:#?}", context.poll);
    });

    bot.polling().start();
}

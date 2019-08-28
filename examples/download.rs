use std::sync::Arc;
use tbot::prelude::*;

fn main() {
    let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();

    bot.document(|context| {
        let bot = Arc::clone(&context.bot);
        let document = context
            .bot
            .get_file(&context.document)
            .into_future()
            .map_err(|err| {
                dbg!(err);
            })
            .and_then(move |file| {
                bot.download_file(&file).map_err(|err| {
                    dbg!(err);
                })
            })
            .map(|bytes| match String::from_utf8(bytes) {
                Ok(document) => println!("{}", document),
                Err(err) => {
                    dbg!(err);
                }
            });

        tbot::spawn(document);
    });

    bot.polling().start();
}

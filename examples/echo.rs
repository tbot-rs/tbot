use tbot::{prelude::*, Bot};

fn main() {
    let mut bot = Bot::from_env("BOT_TOKEN");

    bot.on_message(|context| {
        let reply = context
            .send_message(&context.message)
            .into_future()
            .map_err(|err| eprintln!("Couldn't send a message: {:#?}", err));

        tbot::spawn(reply);
    });

    bot.polling().start();
}

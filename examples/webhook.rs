use tbot::prelude::*;

const URL: &str = "https://example.com";
const PORT: u16 = 2000;

fn main() {
    let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();

    bot.text(|context| {
        let reply = context
            .send_message_in_reply(&context.text.value)
            .into_future()
            .map_err(|err| {
                dbg!(err);
            });

        tbot::spawn(reply);
    });

    bot.webhook(URL, PORT).http().start();
}

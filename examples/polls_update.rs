// You may want to send a poll with your bot first using the `send_poll` example

fn main() {
    let mut bot = tbot::bot!("BOT_TOKEN").event_loop();

    bot.poll(|context| {
        println!("Someone sent a poll: {:#?}", context.poll);
    });

    bot.updated_poll(|context| {
        println!("New update on my poll: {:#?}", context.poll);
    });

    bot.polling().start();
}

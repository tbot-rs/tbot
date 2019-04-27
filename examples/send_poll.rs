use std::time::{Duration, Instant};
use tbot::prelude::*;
use tokio::timer::Delay;

const CHAT: i64 = 0;
const QUESTION: &str = "Do you like tbot?";
const OPTIONS: &[&str] =
    &["Yes", "Also yes", "I like shooting myself in the foot more"];
const CLOSE_AFTER: u64 = 15;

fn main() {
    let bot = tbot::bot!("BOT_TOKEN");

    let poll = bot
        .send_poll(CHAT, QUESTION, OPTIONS)
        .into_future()
        .map_err(|error| {
            dbg!(error);
        })
        .and_then(|message| {
            Delay::new(Instant::now() + Duration::from_secs(CLOSE_AFTER))
                .map(|_| message)
                .map_err(|error| {
                    dbg!(error);
                })
        })
        .and_then(move |message| {
            bot.stop_poll(CHAT, message.id)
                .into_future()
                .map_err(|error| {
                    dbg!(error);
                })
        });

    tbot::run(poll);
}
